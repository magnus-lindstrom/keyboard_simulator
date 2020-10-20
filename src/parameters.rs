use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub nr_min_loss_target: u16,
    pub keyboard_file: String,
    pub letters: Vec<char>,
    pub free_letters: Vec<char>,
    pub unused_keys: Vec<String>,
    pub locked_letters: HashMap<char, String>,
    pub loss_params: HashMap<String, f32>,
}
