import string
import os
from collections import Counter


def get_blank_keyboard_coords():
    w = 20
    h = 20
    shifts = (
        (-w, h),  # unmodified
        (-w, -h),  # shift
        (w, h),  # alt gr
        (w, -h)  # shift + alt gr
    )

    b_row_y = 301
    c_row_y = 212
    d_row_y = 128
    e_row_y = 42
    b_row_x_start = 144
    c_row_x_start = 188
    d_row_x_start = 166
    e_row_x_start = 45
    kw = 85  # key width

    coords = {
        'TLDE': (e_row_x_start        , e_row_y),
        'AE01': (e_row_x_start +  1*kw, e_row_y),
        'AE02': (e_row_x_start +  2*kw, e_row_y),
        'AE03': (e_row_x_start +  3*kw, e_row_y),
        'AE04': (e_row_x_start +  4*kw, e_row_y),
        'AE05': (e_row_x_start +  5*kw, e_row_y),
        'AE06': (e_row_x_start +  6*kw, e_row_y),
        'AE07': (e_row_x_start +  7*kw, e_row_y),
        'AE08': (e_row_x_start +  8*kw, e_row_y),
        'AE09': (e_row_x_start +  9*kw, e_row_y),
        'AE10': (e_row_x_start + 10*kw, e_row_y),
        'AE11': (e_row_x_start + 11*kw, e_row_y),
        'AE12': (e_row_x_start + 12*kw, e_row_y),

        'LSGT': (b_row_x_start        , b_row_y),
        'AB01': (b_row_x_start +  1*kw, b_row_y),
        'AB02': (b_row_x_start +  2*kw, b_row_y),
        'AB03': (b_row_x_start +  3*kw, b_row_y),
        'AB04': (b_row_x_start +  4*kw, b_row_y),
        'AB05': (b_row_x_start +  5*kw, b_row_y),
        'AB06': (b_row_x_start +  6*kw, b_row_y),
        'AB07': (b_row_x_start +  7*kw, b_row_y),
        'AB08': (b_row_x_start +  8*kw, b_row_y),
        'AB09': (b_row_x_start +  9*kw, b_row_y),
        'AB10': (b_row_x_start + 10*kw, b_row_y),

        'AC01': (c_row_x_start        , c_row_y),
        'AC02': (c_row_x_start +  1*kw, c_row_y),
        'AC03': (c_row_x_start +  2*kw, c_row_y),
        'AC04': (c_row_x_start +  3*kw, c_row_y),
        'AC05': (c_row_x_start +  4*kw, c_row_y),
        'AC06': (c_row_x_start +  5*kw, c_row_y),
        'AC07': (c_row_x_start +  6*kw, c_row_y),
        'AC08': (c_row_x_start +  7*kw, c_row_y),
        'AC09': (c_row_x_start +  8*kw, c_row_y),
        'AC10': (c_row_x_start +  9*kw, c_row_y),
        'AC11': (c_row_x_start + 10*kw, c_row_y),
        'BKSL': (c_row_x_start + 11*kw, c_row_y),

        'AD01': (d_row_x_start        , d_row_y),
        'AD02': (d_row_x_start +  1*kw, d_row_y),
        'AD03': (d_row_x_start +  2*kw, d_row_y),
        'AD04': (d_row_x_start +  3*kw, d_row_y),
        'AD05': (d_row_x_start +  4*kw, d_row_y),
        'AD06': (d_row_x_start +  5*kw, d_row_y),
        'AD07': (d_row_x_start +  6*kw, d_row_y),
        'AD08': (d_row_x_start +  7*kw, d_row_y),
        'AD09': (d_row_x_start +  8*kw, d_row_y),
        'AD10': (d_row_x_start +  9*kw, d_row_y),
        'AD11': (d_row_x_start + 10*kw, d_row_y),
        'AD12': (d_row_x_start + 11*kw, d_row_y),
    }
    return coords, shifts


def get_altgr_letters():

    spc_keys = {
        'LSGT': ('', ''),
        'AB01': ('', ''),
        'AB02': ('7', ''),
        'AB03': ('8', ''),
        'AB04': ('9', ''),
        'AB05': ('', ''),
        'AB06': ('', ''),
        'AB07': ('', ''),
        'AB08': ('', ''),
        'AB09': ('', ''),
        'AB10': ('', ''),

        'AC01': ('0', ''),
        'AC02': ('4', ''),
        'AC03': ('5', ''),
        'AC04': ('6', ''),  # f
        'AC05': ('', ''),
        'AC06': ('', ''),
        'AC07': ('@', ''),  # j
        'AC08': ('^', ''),
        'AC09': ('|', ''),
        'AC10': ('', ''),
        'AC11': ('', ''),
        'BKSL': ('', ''),

        'AD01': ('', ''),
        'AD02': ('1', ''),
        'AD03': ('2', ''),
        'AD04': ('3', ''),
        'AD05': ('', ''),
        'AD06': ('', ''),
        'AD07': ('', ''),
        'AD08': ('', ''),
        'AD09': ('', ''),
        'AD10': ('', ''),
        'AD11': ('', ''),
        'AD12': ('', '')
    }

    return spc_keys


def get_number_row_letters():

    keys = {
        'TLDE': ('', '', '', ''),
        'AE01': ('~', '', '', ''),
        'AE02': ('!', '', '', ''),
        'AE03': ('#', '', '', ''),
        'AE04': ('$', '', '', ''),
        'AE05': ('%', '', '', ''),
        'AE06': ('', '', '', ''),
        'AE07': ('&', '', '', ''),
        'AE08': ('*', '', '', ''),
        'AE09': ('(', '[', '{', ''),
        'AE10': (')', ']', '}', ''),
        'AE11': ('_', '-', '', ''),
        'AE12': ('=', '+', '', ''),
    }

    return keys


def get_unused_keys():
    keys = {
        'AB10': ('/',  '?', '', ''),
        'BKSL': ('\\', '', '', ''),
        'AD12': (';',  ':', '', '')
    }
    return keys


def save_freqs(freqs, name, data_dir):

    with open(data_dir + name, 'w') as f:
        for sequence, freq in freqs.most_common():
            f.write('{} {:.16f}\n'.format(sequence, freq))


def x11_letter_names():

    dicct = {'~': 'asciitilde', '!': 'exclam',      '#': 'numbersign',      '$': 'dollar',
             '%': 'percent',    '&': 'ampersand',   '*': 'asterisk',        '(': 'parenleft',
             ')': 'parenright', '[': 'bracketleft', ']': 'brackeright',     '?': 'question',
             '{': 'braceleft',  '}': 'braceright',  '_': 'underscore',      '-': 'minus',
             '=': 'equal',      '+': 'plus',        '\'': 'apostrophe',     '"': 'quotedbl',
             'å': 'aring',      'Å': 'Aring',       'ä': 'adiaeresis',      'Ä': 'Adiaeresis',
             'ö': 'odiaeresis', 'Ö': 'Odiaeresis',  ';': 'semicolon',       ':': 'colon',
             '@': 'at',         '^': 'asciicircum', '|': 'bar',             '\\': 'backslash',
             ',': 'comma',      '.': 'period',      '<': 'less',            '>': 'greater',
             '/': 'slash'}
    return dicct


def assemble_keyboard(letter_layout, spc_letters, nr_row_let, unused_keys):
    full_layout = {}
    for key in letter_layout.keys():
        full_layout[key] = list(letter_layout[key])
    for key in spc_letters:
        if key in full_layout.keys():
            full_layout[key].extend(list(spc_letters[key]))
        else:
            full_layout[key] = ['', '', spc_letters[key][0], spc_letters[key][1]]
    for key in nr_row_let.keys():
        full_layout[key] = nr_row_let[key]
    for key in unused_keys.keys():
        full_layout[key] = unused_keys[key]

    return full_layout


def save_x11_file(layout, path, name):

    all_keys = ['AE01', 'AE02', 'AE03', 'AE04', 'AE05', 'AE06',
                'AE07', 'AE08', 'AE09', 'AE10', 'AE11', 'AE12',
                'AD01', 'AD02', 'AD03', 'AD04', 'AD05', 'AD06',
                'AD07', 'AD08', 'AD09', 'AD10', 'AD11', 'AD12',
                'AC01', 'AC02', 'AC03', 'AC04', 'AC05', 'AC06',
                'AC07', 'AC08', 'AC09', 'AC10', 'AC11', 'BKSL',
                'LSGT', 'AB01', 'AB02', 'AB03', 'AB04', 'AB05',
                'AB06', 'AB07', 'AB08', 'AB09', 'AB10']
    # to know where to add newlines (for readability)
    newline_keys = ['AE12', 'AD12', 'BKSL']
    x11_names = x11_letter_names()

    file_content = ''

    for key in all_keys:
        file_content = file_content + '    key <{}> {{ [ '.format(key)
        if key in layout.keys():
            symbols = [x11_names[s] if s in x11_names.keys() else s for s in layout[key]]
        else:
            symbols = []
        symbols = [sym for sym in symbols if sym]
        file_content = file_content + ',\t'.join(symbols)
        file_content = file_content + ' ] };\n'
        if key in newline_keys:
            file_content = file_content + '\n'

    with open(path + name, 'w') as f:
        f.write(file_content)


def get_counts(language, data_dir):

    allowed_letters = list(string.ascii_letters)
    allowed_letters.extend([' ', 'å', 'ä', 'ö', 'Å', 'Ä', 'Ö', '.', ',', '\''])

    repl_dict = {
        'á': 'a',
        'é': 'e',
        'í': 'i',
        'ó': 'o',
        'ú': 'u',
        'à': 'a',
        'è': 'e',
        'ì': 'i',
        'ò': 'o',
        'ù': 'u',
        '’': '\''
    }
    # Get most common words

    complete_word_list = []
    for filename in os.listdir(data_dir + '{}_books/'.format(language)):
        with open(data_dir + '{}_books/'.format(language) + filename, 'r') as f:
            text = f.read()
        text = [c.lower() if c in allowed_letters
                else repl_dict[c] if c in repl_dict.keys()
                else ' ' for c in text]
        text = ''.join(text)
        words = text.split()
        for i_word, word in enumerate(words):
            words[i_word] = word.strip('\'')
        complete_word_list.extend(words)

    word_count = Counter(complete_word_list)

    # Get the most common letters and bigrams
    bigram_count = Counter()
    letter_count = Counter()
    for word, count in word_count.most_common():
        for letter in word:
            letter_count.update({letter: count})
        if len(word) > 1:
            for i in range(len(word) - 1):
                bigram = word[i:i+2]
                if bigram[0] != bigram[1]:
                    bigram_count.update({bigram: count})

    return word_count, bigram_count, letter_count


def freq_from_count(count):

    tot_count = sum(count.values())
    freq = Counter()
    for key in count.keys():
        freq.update({key: count[key] / tot_count})

    return freq

