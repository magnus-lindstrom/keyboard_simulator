mod parameters;

extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate wasm_bindgen;
use std::time::Instant;
use std::process::Command;
use std::convert::TryInto;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin, stdout, Read, Write};
use rand::prelude::*;
use math::round;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{fs, thread};
use rand::Rng;
use parameters::Parameters;
use wasm_bindgen::prelude::*;

static NR_THREADS: u8 = 4;
static MIN_SQRD_KEY_SEP: f32 = 1.6;


#[wasm_bindgen]
pub fn start() {

    println!("Starting optimisation. {} threads active.\n", NR_THREADS);
    let start_time = Instant::now();
    // test_write();
    let par_struct = get_params();
    let data = get_lang_data();
    let key_vector = get_key_vector(&par_struct);
    make_qwerty_comparison(&data, key_vector, &par_struct);

    let (tx, rx) = mpsc::channel();

    for _i in 0..NR_THREADS {
        let thread_tx = tx.clone();
        let thread_par_struct = par_struct.clone();
        let thread_data = data.clone();
        thread::spawn(move || start_worker(thread_tx, thread_par_struct,
                                           thread_data));
    }
    let mut secretary = init_secretary(&par_struct);
    while secretary.need_more_data() {
        let (loss, key_map_string) = rx.recv().unwrap();
        secretary.report(loss, key_map_string);
    }
    secretary.save_results(&par_struct);
    secretary.print_done(start_time);
}

fn test_write() {
    let mut param = Parameters {
        nr_min_loss_target: 10,
        keyboard_file: "hej_och_ha".to_string(),
        letters: vec!['\'', ','],
        free_letters: vec!['c', 'd'],
        unused_keys: vec!["key1".to_string(), "key2".to_string()],
        locked_letters: HashMap::new(),
        loss_params: HashMap::new(),
    };
    param.locked_letters.insert('\'', "key3".to_string());
    param.locked_letters.insert(',', "key4".to_string());
    let s = serde_yaml::to_string(&param).unwrap();
    let mut file = File::create("foo.yaml").unwrap();
    file.write_all(s.as_bytes());
}

fn start_worker(tx: Sender<(f32, String)>, par_struct: Parameters, data: Data) {
    let key_vector = get_key_vector(&par_struct);
    let qwerty_key_map = make_qwerty_map(&key_vector);
    let key_vector = get_key_vector(&par_struct);
    let mut key_map = make_char_key_map(&par_struct, &key_vector);
    make_valid_keyboard(&par_struct, &mut key_map, &qwerty_key_map);
    let mut swappable_pairs = get_swappable_pairs(&par_struct);
    let mut t: Temp = get_temp();

    loop {
        t.init();
        let mut swap_counter: u32 = 0;
        shuffle_keys(&mut key_map, &qwerty_key_map, &par_struct);
        shuffle(&mut swappable_pairs);
        let mut loss = get_loss(&data, &key_map, &par_struct);

        while swap_counter < swappable_pairs.len() as u32 {
            // try swapping a pair of keys, swap has to be legal
            // (not too close to qwerty layout)
            if legal_swap(&key_map, &qwerty_key_map,
                          &swappable_pairs[swap_counter as usize]) {
                swap_keys(&swappable_pairs[swap_counter as usize], &mut key_map);
                let new_loss = get_loss(&data, &key_map, &par_struct);
                if t.should_swap(loss, new_loss) {
                    // if lower loss, keep the new arrangement, lower the temp
                    loss = new_loss;
                    swap_counter = 0;
                    shuffle(&mut swappable_pairs);
                    t.decrease_temp();
                    continue;
                } else { // swap back
                    swap_keys(&swappable_pairs[swap_counter as usize],
                              &mut key_map);
                    swap_counter += 1;
                }
            } else { swap_counter += 1; }
        }
        let key_map_string = stringify_map(&key_map);
        tx.send((loss, key_map_string)).unwrap();
    }
}

fn legal_swap(key_map: &HashMap<char, &Key>, qwerty_key_map: &HashMap<char, &Key>,
              swap_pair: &(char, char)) -> bool {
    // check for the would-be new positions of the two letters
    // letter 0 occupies the space specified in key_map atm, but would occupy the
    // one currently occupied by letter 1 after the swap

    // check letter 0
    let mut x_dist = (key_map[&swap_pair.1].x - qwerty_key_map[&swap_pair.0].x)
        .abs();
    let mut y_dist = (key_map[&swap_pair.1].y - qwerty_key_map[&swap_pair.0].y)
        .abs();
    let mut dist = x_dist.powf(2.0) + y_dist.powf(2.0);
    if dist < MIN_SQRD_KEY_SEP {
        return false
    }

    // check letter 1
    x_dist = (key_map[&swap_pair.0].x - qwerty_key_map[&swap_pair.1].x).abs();
    y_dist = (key_map[&swap_pair.0].y - qwerty_key_map[&swap_pair.1].y).abs();
    dist = x_dist.powf(2.0) + y_dist.powf(2.0);
    if dist < MIN_SQRD_KEY_SEP {
        return false
    }

    return true
}

fn deb<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

#[derive(Debug, Default)]
struct Temp {
    // used to see how many swaps it takes with very low temp
    temp_0_cal: f64,
    temp_0: f64,
    temp: f64,
    alpha: f64,
    // the number of times the temperature has been reset to temp_0
    init_counter: u32,
    // nr of swaps (or temp decreases) made during the last <control_interval>
    // nr of iterations
    swap_counter: u32,
    // nr of swaps to aim for during a run
    swap_target: u32,
    control_interval: u32
}

impl Temp {
    fn init(&mut self) {
        // Init temp to very low for the first 10 rounds to get baseline nr of
        // swaps needed
        if self.init_counter < self.control_interval {
            self.temp = self.temp_0_cal;
        } else { self.temp = self.temp_0; }

        self.init_counter += 1;

        if self.init_counter == self.control_interval {
            // Get average nr of swaps using cold temp
            self.swap_target = 2 * (self.swap_counter as f32
                                    / self.control_interval as f32) as u32;
            self.swap_counter = 0;
        } else if time_for_action(self.init_counter, self.control_interval) {
            // Adjust init temperature to come closer to target
            let swap_offset = (self.swap_counter as f64 / self.control_interval as f64)
                / self.swap_target as f64;
            //println!("Cold/Target/Current/Ratio: {}/{}/{}/{}",
                     //self.swap_target / 2, self.swap_target,
                     //self.swap_counter / self.control_interval, swap_offset);
            self.temp_0 = self.temp_0 as f64 / (swap_offset as f64);
            self.swap_counter = 0;
            if swap_offset > 5.0 || swap_offset < 0.2 {
                print!("WARNING: Algorithm is far away from desired number of successful swaps
                       : {} vs {}.", self.swap_target, self.swap_target as f64 * swap_offset);
            }
        }
    }

    fn decrease_temp(&mut self) {
        self.temp = self.temp * self.alpha;
        self.swap_counter += 1;
    }

    fn should_swap(&self, old_loss: f32, new_loss: f32) -> bool {
        // sometimes get stuck in these places
        if old_loss == new_loss { return false; }

        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>(); // random nr from U(0, 1)

        // if r < ((old_loss - new_loss) / self.temp).exp() {
            // println!("temp: {}, lossdelta: {}, r: {}",
                     // self.temp, new_loss - old_loss, r);
            // println!("exponent: {}\n", ((old_loss - new_loss) / self.temp).exp());
        // }
        return r < ((old_loss - new_loss) / self.temp as f32).exp();
    }

}

fn get_temp() -> Temp {
    return Temp {
        temp_0_cal: 0.0000000001,
        temp_0: 0.001,
        temp: 0.0,
        alpha: 0.99,
        init_counter: 0,
        swap_counter: 0,
        swap_target: 0,
        control_interval: 10
    };
}

fn time_for_action(iteration: u32, action_interval: u32) -> bool {
    if round::floor(iteration as f64 / action_interval as f64, 0)
        == iteration as f64 / action_interval as f64 {
        return true;
    } else { return false; }
}


struct Secretary {
    loss_stat: HashMap<String, u32>,
    iteration: u32,
    print_interval: u32,
    min_loss: f32,
    nr_min_loss: u16,
    nr_min_loss_target: u16,
    best_layout: String
}

impl Secretary {
    fn report(&mut self, loss: f32, key_map_string: String) {
        self.iteration += 1;

        // Report life signs every once in a while
        if time_for_action(self.iteration, self.print_interval) {
            println!("Iteration {}.", self.iteration);
        }

        // if the found layout is invalid, loss is set to 100. don't handle it
        if loss > 10.0 { return }

        // update loss entries
        let stat = self.loss_stat.entry(format!("{:.*}", 7, loss))
            .or_insert(0);
        *stat += 1;

        // notify about new loss min
        if loss < self.min_loss {
            self.min_loss = loss;
            println!("New minimum loss found! {}", loss);
            self.nr_min_loss = 1;
            self.best_layout = key_map_string;
        } else if loss == self.min_loss {
            self.nr_min_loss += 1;
            println!("Same minimum loss found! Count: {}", self.nr_min_loss);
        }

    }
    fn need_more_data(&self) -> bool {
        return self.nr_min_loss < self.nr_min_loss_target
    }

    fn save_results(&self, par_struct: &Parameters) {
        save_layout(&self.best_layout, &par_struct);
        save_loss_stats(&self.loss_stat);
    }

    fn print_done(&self, start_time: Instant) {
        let mut secs: f64 = start_time.elapsed().as_secs() as f64;
        let mut mins = round::floor(secs/60.0, 0);
        let mut hours = round::floor(mins/60.0, 0);
        let days = round::floor(hours/24.0, 0);

        secs = secs % 60.0;
        mins = mins % 60.0;
        hours = hours % 24.0;
        if days > 0.0 {
            println!("Optimization done! Time elapsed: {}days {}h {}min {}s.",
                     days, hours, mins, secs);
        } else if hours > 0.0 {
            println!("Optimization done! Time elapsed: {}h {}min {}s.",
                     hours, mins, secs);
        } else if mins > 0.0 {
            println!("Optimization done! Time elapsed: {}min {}s.", mins, secs);
        } else { println!("Optimization done! Time elapsed: {}s.", secs); }
    }
}

fn init_secretary(par_struct: &Parameters) -> Secretary {
    return Secretary {
        loss_stat: HashMap::new(),
        iteration: 0,
        print_interval: 100,
        min_loss: std::f32::MAX,
        nr_min_loss: 0,
        nr_min_loss_target: par_struct.nr_min_loss_target,
        best_layout: "".to_string()
    };
}

fn stringify_map(key_map: &HashMap<char, &Key>) -> String {
    let mut data = "".to_string();
    for (letter, key) in key_map.iter() {
        data.push_str(&letter.to_string());
        data.push_str(" ");
        data.push_str(&key.name);
        data.push_str("\n");
    }
    data
}

fn save_layout(key_map_string: &String, par_struct: &Parameters) {
    let file_name = format!("results/layouts/{}{}{}{}",
                            par_struct.loss_params["out_roll_punish"],
                            par_struct.loss_params["same_fing_punish"],
                            par_struct.loss_params["big_y_jump_punish"],
                            par_struct.loss_params["hill_shape_punish"]);
    //for c in par_struct.locked_letters.keys() {
        //if *c == '.' { file_name.push('p'); }
        //else { file_name.push(*c); }
    //}
    fs::write(&file_name, key_map_string)
        .expect("Unable to write end layout result to file");

    //sort the output file lines alphabetically
    Command::new("sort")
            .arg(&file_name)
            .arg(format!("--output={}", &file_name))
            .spawn()
            .expect("Failed to sort lines in layout file.");
}

fn save_loss_stats(loss_stat: &HashMap<String, u32>) {
    let stat_path = "results/statistics";

    let mut data = "".to_string();
    for (loss, count) in loss_stat.iter() {
        data.push_str(&loss);
        data.push_str(" ");
        data.push_str(&count.to_string());
        data.push_str("\n");
    }
    fs::write(stat_path, data).expect("Unable to write file");
    Command::new("sort")
            .arg(stat_path)
            .arg(format!("--output={}", stat_path))
            .spawn()
            .expect("Failed to sort lines in statistics file.");
}

fn make_qwerty_comparison(data: &Data, key_vector: Vec<Key>,
                          par_struct: &Parameters) {
    let qwerty_key_map = make_qwerty_map(&key_vector);
    let loss = get_loss(&data, &qwerty_key_map, &par_struct);
    println!("Qwerty loss: {:.*}.", 4, loss);
}

fn swap_keys(pair: &(char, char), key_map: &mut HashMap<char, &Key>) {
    let tmp_key: &Key = key_map[&pair.0];
    key_map.insert(pair.0, key_map[&pair.1]);
    key_map.insert(pair.1, tmp_key);
}

fn get_swappable_pairs(par_struct: &Parameters) -> Vec<(char, char)> {
    let mut swap_pairs: Vec<(char, char)> = Vec::new();
    for i in 0..par_struct.free_letters.len(){
        for j in (i+1)..par_struct.free_letters.len(){
            swap_pairs.push((par_struct.free_letters[i], par_struct.free_letters[j]));
        }
    }
    swap_pairs
}

fn get_loss(data: &Data, key_map: &HashMap<char, &Key>,
            par_struct: &Parameters) -> f32 {
    let out_roll_punish = par_struct.loss_params["out_roll_punish"];
    let same_fing_punish = par_struct.loss_params["same_fing_punish"];
    let big_y_jump_punish = par_struct.loss_params["big_y_jump_punish"];
    let hill_shape_punish = par_struct.loss_params["hill_shape_punish"];
    let mut loss1 = 0.0; // bigram loss
    let mut loss2 = 0.0; // one finger loss
    let mut loss3 = 0.0; // ind key freq loss
    for bigram_f in data.bigrams_f.iter() {
        // Set baseline loss, multiply with punishes later, if called for
        let mut l1 = bigram_f.1;
        let mut l2 = bigram_f.1;
        let key1 = key_map.get(&(bigram_f.0).0).expect("Map lookup failure: 1");
        let key2 = key_map[&(bigram_f.0).1];
        if key1.finger * key2.finger > 0 { // check if same hand
            if key1.finger != key2.finger { // check if different fingers

                // check for outward rolls
                if key1.finger > 0 && key1.x < key2.x || key1.finger < 0 && key1.x > key2.x {
                    l1 *= out_roll_punish;
                }
                // Check for adjacent fingers and y jumps
                if (key1.finger - key2.finger).abs() == 1
                    &&
                   (key1.y - key2.y).abs() == 2.0 {
                    l1 *= big_y_jump_punish;
                   }
                // Check if left hand
                if key1.finger < 0 {
                    // check for hill shape
                    if (key1.x > key2.x) && (key1.y < key2.y) {
                        l1 *= hill_shape_punish;
                    } else if (key1.x < key2.x) && (key1.y > key2.y) {
                        l1 *= hill_shape_punish;
                    }
                } else { // is right hand, different fingers
                    if (key1.x > key2.x) && (key1.y > key2.y) {
                        l1 *= hill_shape_punish;
                    } else if (key1.x < key2.x) && (key1.y < key2.y) {
                        l1 *= hill_shape_punish;
                    }
                }
            } else { // is same finger
                // punish one finger bigrams over different
                // rows hard
                let y_diff = (key1.y - key2.y).abs();
                l2 *= same_fing_punish * (y_diff + 1.0);
            }
        }
        loss1 += l1;
        loss2 += l2;
    }
    // Go over all letters and their frequencies, punish frequently used hard to reach keys
    for let_freq in data.letters_f.iter() {
        loss3 += key_map[&let_freq.0].effort * let_freq.1;
    }
    // Third loss type outweighs the other ones
    loss3 *= 0.7;

    // println!("{} {} {}", loss1, loss2, loss3);
    return (loss1 + loss2 + loss3) / 3.0;
}

fn shuffle<T>(vec: &mut [T]) {
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
}

fn shuffle_keys(key_map: &mut HashMap<char, &Key>,
                qwerty_key_map: &HashMap<char, &Key>, par_struct: &Parameters) {
    let mut new_inds: Vec<u8> = Vec::new();
    for i in 0..par_struct.free_letters.len() {
        new_inds.push(i.try_into().unwrap()); // convert usize to u8
    }
    shuffle(&mut new_inds);

    // Doing twice as many swaps as necessary here, but OK for now.
    for (i, new_i) in new_inds.iter().enumerate() {
        let swap_pair: (char, char) = (par_struct.free_letters[i],
                                       par_struct.free_letters[*new_i as usize]);
        // make sure that the swap would not cause letters to come too
        // close to their qwerty positions
        if legal_swap(&key_map, &qwerty_key_map, &swap_pair) {
            let tmp_key: &Key = key_map[&par_struct.free_letters[i]];
            key_map.insert(par_struct.free_letters[i],
                           key_map[&par_struct.free_letters[*new_i as usize]]);
            key_map.insert(par_struct.free_letters[*new_i as usize], tmp_key);
        }
    }
}

fn get_lang_data() -> Data {
    let bigrams_path = "src/stats/tot_bigram_frequency";
    let letters_path = "src/stats/tot_letter_frequency";
    let mut data = Data {
        letters_f: Vec::new(),
        bigrams_f: Vec::new(),
    };
    // Get letters
    let f = File::open(letters_path).expect("Couldn't open letters path.");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
            let mut iter = l.split_whitespace();
            let letter = iter.next().unwrap().chars().nth(0)
                .expect("Tried parsing letter characters");
            let freq = iter.next().unwrap().parse()
                .expect("Tried parsing letter frequencies.");
            data.letters_f.push((letter, freq));
    }
    // Get bigrams
    let f = File::open(bigrams_path).expect("Couldn't open bigrams path.");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
            // iterator over the words in each line
            let mut iter = l.split_whitespace();
            // iterator over the chars in the first word
            let mut char_it = iter.next().unwrap().chars();
            let first = char_it.next().unwrap();
            let second = char_it.next().unwrap();
            let bigram = (first, second);
            let freq: f32 = iter.next().unwrap().parse().unwrap();
            data.bigrams_f.push((bigram, freq));
    }
    data
}

#[derive(Debug, Clone)]
struct Data {
    letters_f: Vec<(char, f32)>,
    bigrams_f: Vec<((char, char), f32)>,
}

fn make_char_key_map<'a>(par_struct: &Parameters, key_vector: &'a [Key])
                                       -> HashMap<char, &'a Key> {
    let mut map: HashMap<char, &Key> = HashMap::new();

    let mut key_index = 0;
    for letter in &par_struct.letters {
        let mut iter = par_struct.locked_letters.keys();
        if let Some(_) = iter.find(|&lett| lett == letter) {
            // this letter is locked
            let key_name = &par_struct.locked_letters[letter];
            let mut key_iter = key_vector.iter();
            if let Some(key) = key_iter.find(|&key| &key.name == key_name) {
                map.insert(*letter, key);
            }
        } else { // not a locked letter
            let mut not_found_key = true;
            while not_found_key {
                let key_name = &key_vector[key_index].name;
                let mut keyname_iter_ll = par_struct.locked_letters.values();
                let mut keyname_iter_uk = par_struct.unused_keys.iter();
                if let None = keyname_iter_ll.find(|&k| k == key_name) {
                    // current key is not locked to a letter
                    if let None = keyname_iter_uk.find(|&k| k == key_name) {
                        // current key is not unused
                        not_found_key = false;
                        map.insert(*letter, &key_vector[key_index]);
                    }
                }
                key_index += 1
            }
        }
    }
    map
}

fn get_key_vector(par_struct: &Parameters) -> Vec<Key> {
    let f = File::open(&par_struct.keyboard_file).expect("Couldn't open params file");
    let file = BufReader::new(&f);

    let mut key_vector: Vec<Key> = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();

        // Read lines not starting with #
        if !l.starts_with('#') {
            let mut iter = l.split_whitespace();
            let name = iter.next().unwrap();
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let finger: i8 = iter.next().unwrap().parse().unwrap();
            let effort: f32 = iter.next().unwrap().parse().unwrap();

            let key = Key {
                name: name.to_string(),
                x,
                y,
                finger,
                effort,
            };
            key_vector.push(key);
        }
    }
    key_vector
}

#[derive(Debug, Clone)]
struct Key {
    name: String,
    finger: i8,
    effort: f32,
    x: f32,
    y: f32,
}

fn get_params() -> Parameters {
    let param_string = fs::read_to_string("src/params_new.yaml")
        .expect("Could not deserialise input_params file.");
    let params: Parameters = serde_yaml::from_str(&param_string).unwrap();
    return params
}

fn make_qwerty_map<'a>(key_vector: &'a [Key]) -> HashMap<char, &'a Key> {
    let qwerty_keys = [ ("AD01", 'q'), ("AD02", 'w'), ("AD03", 'e'), ("AD04", 'r'), ("AD05", 't'),
        ("AD06", 'y'), ("AD07", 'u'), ("AD08", 'i'), ("AD09", 'o'), ("AD10", 'p'), ("AD11", 'å'),
        ("AC01", 'a'), ("AC02", 's'), ("AC03", 'd'), ("AC04", 'f'), ("AC05", 'g'),
        ("AC06", 'h'), ("AC07", 'j'), ("AC08", 'k'), ("AC09", 'l'),	("AC10", 'ö'), ("AC11", 'ä'),
        ("BKSL", '\''), ("AB01", 'z'), ("AB02", 'x'), ("AB03", 'c'), ("AB04", 'v'),
        ("AB05", 'b'), ("AB06", 'n'), ("AB07", 'm'), ("AB08", ','), ("AB09", '.') ];
    let mut key_map: HashMap<char, &Key> = HashMap::new();
    for key in key_vector {
        let mut iter = qwerty_keys.iter();
        if let Some(tuple) = iter.find(|&tup| tup.0 == key.name) {
            key_map.insert(tuple.1, &key);
        }
    }
    key_map
}

fn pause() {
    // pause until enter is pressed
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn make_valid_keyboard(par_struct: &Parameters, mut key_map: &mut HashMap<char, &Key>,
                       qwerty_key_map: &HashMap<char, &Key>) {

    let mut swappable_pairs = get_swappable_pairs(&par_struct);

    // A keyboard is valid if no key is too close to its qwerty analogue
    // locked letters necessarily occupy the same places
    for c in &par_struct.free_letters {
        let x_dist = (key_map[c].x - qwerty_key_map[c].x).abs();
        let y_dist = (key_map[c].y - qwerty_key_map[c].y).abs();
        let dist = x_dist.powf(2.0) + y_dist.powf(2.0);
        if dist < MIN_SQRD_KEY_SEP {
            let mut swap_counter: u32 = 0;
            shuffle(&mut swappable_pairs);
            while swap_counter < swappable_pairs.len() as u32 {
                // go through all swaps involving c
                if c != &swappable_pairs[swap_counter as usize].0
                   &&
                   c != &swappable_pairs[swap_counter as usize].1
                {
                   swap_counter += 1;
                   continue;
                }
                // try swapping a pair of keys, swap has to be legal
                // (not too close to qwerty layout)
                if legal_swap(&key_map, &qwerty_key_map,
                              &swappable_pairs[swap_counter as usize]) {
                    swap_keys(&swappable_pairs[swap_counter as usize],
                              &mut key_map);
                    break;
                } else { swap_counter += 1; }
            }
            if swap_counter == swappable_pairs.len() as u32 {
                panic!("Unable to create valid keyboard.");
            }
        }
    }
}

/*
fn valid_keyboard(par_struct: &Parameters, key_map: &HashMap<char, &Key>,
                  qwerty_key_map: &HashMap<char, &Key>) -> bool {
    // A keyboard is valid if no key is too close to its qwerty analogue
    for c in key_map.keys() {
        let mut iter = par_struct.locked_letters.keys();
        if let Some(_) = iter.find(|&letter| letter == c) {
            // locked letters necessarily occupy the same places
            continue;
        }
        let x_dist = (key_map[c].x - qwerty_key_map[c].x).abs();
        let y_dist = (key_map[c].y - qwerty_key_map[c].y).abs();
        let dist = x_dist.powf(2.0) + y_dist.powf(2.0);
        if dist < MIN_SQRD_KEY_SEP {
            deb(c);
            deb(key_map[c]);
            deb(qwerty_key_map[c]);
            print!("xdist: {}", x_dist);
            print!("    ydist: {}", y_dist);
            println!("    dist: {}", dist);
            pause();
            return false
        }
    }
    return true
}

fn get_pairs(par_struct: &Parameters) {
    let mut pairs: Vec<(u8, u8)> = Vec::new();
    for i in 0..par_struct.letters.len() {
        for j in i..par_struct.letters.len() {
            pairs.push((i as u8, j as u8));
        }
    }
}

fn make_qwerty_char_key_map<'a>(par_struct: &'a Parameters, key_vector: &'a [Key]) -> HashMap<char, &'a Key> {
    // Create a qwerty mapping
    // Assumption: The keys in key_vector come in the order
    //      AD01, ..., AD12, AC01, ..., AC12, AB00, ..., AB10

    let mut map: HashMap<char, &Key> = HashMap::new();
    let iter = par_struct.letters.iter().zip(key_vector.iter());
    for i in 0..40 {
        match par_struct.letters.get(i) {
            Some(x) => print!("{:?}", x),
            None => print!("' '")
        } match key_vector.get(i) {
            Some(key) => println!("{:?}", key.name),
            None => println!("\"    \"")
        }
        // map.insert(*letter, key);
    }
    map
}

fn run_optimisation() {
    use std::fs::{File};
    use std::io::prelude::*;
    let file = File::create("test.txt");
    let mut file = match file {
        Ok(f) => f,
        Err(err) => {
            panic!("Couldn't create file. {:?}", err)
        }
    };
    file.write_all(b"Hello, world!").unwrap();
}

fn get_min_loss_count() -> u16 {

    let mut min_count: u16;
    loop {
        print!("Loss count (min loss) before termination: ");
        stdout().flush().unwrap();
        let s = get_string();
        min_count = s.parse().unwrap();
            if min_count < 3 {
                println!("Was expecting a value >= 3.");
            }
            else {break;}
    }
    min_count
}

fn get_string() -> String {

    let mut s = String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string.");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn get_keys(par_struct: &mut Parameters) {
    let f = File::open(&par_struct.keyboard_file)
        .expect("Couldn't open params file");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();

        // Read lines not starting with #
        if l.chars().next() != Some('#') {
            let mut iter = l.split_whitespace();
            let key = match iter.next() {
                Some(x) => x,
                None => "XXXX"
            };
            par_struct.used_keys.push(key.to_string());
        }
    }
}
*/
