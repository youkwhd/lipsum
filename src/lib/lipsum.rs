use crate::data;
use crate::std::string::Capitalize;
use rand::{self, Rng};

const CHANCE_TO_ADD_QUESTION_MARK: u32 = 3;
const CHANCE_TO_ADD_COMMA: u32 = 8;
const CHANCE_TO_ADD_DOT: u32 = 5;

pub fn generate(n_paragraph: u32, n_words: u32, starts_with_lorem_ipsum: bool) -> String {
    let mut n_paragraph = n_paragraph as i32;
    let common_len = data::latin::COMMONS.len() as u32;

    let mut buf = Vec::new();

    if starts_with_lorem_ipsum {
        let mut paragraph = String::new();

        if n_words <= common_len {
            paragraph.push_str(&generate_from_commons(n_words, true));
        } else {
            paragraph.push_str(&generate_from_commons(common_len, true));
            paragraph.push_str(" ");
            paragraph.push_str(&generate_from_words(n_words - common_len, paragraph.ends_with(". ")));
        }

        n_paragraph -= 1;
        buf.push(paragraph);
    }

    for _ in 1..=n_paragraph {
        let paragraph = generate_from_words(n_words, true);
        buf.push(paragraph);
    }

    buf.join("\n\n")
}


pub fn generate_words(n: u32, starts_with_lorem_ipsum: bool) -> String {
    generate(1, n, starts_with_lorem_ipsum)
}

pub fn generate_from_commons(n: u32, capitalize: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    let mut capitalize = capitalize;

    for i in 0..n {
        let mut word = String::from(data::latin::COMMONS[i as usize]);

        if capitalize {
            word.capitalize();
            capitalize = false;
        }

        if i != (n - 1) && rng.gen_range(1..=100) <= CHANCE_TO_ADD_COMMA {
            word.push(',');
        } else if i != (n - 1) && rng.gen_range(1..=100) <= CHANCE_TO_ADD_DOT {
            word.push('.');
            capitalize = true;
        } else if i != (n - 1) && rng.gen_range(1..=100) <= CHANCE_TO_ADD_QUESTION_MARK {
            word.push('?');
        }

        words.push(word);
    }

    let mut buf = words.join(" ");

    if words.len() > 0 {
        buf.push(if rng.gen_range(0..=1) != 0 { '.' } else { '?' });
    }

    buf
}

fn generate_from_words(n: u32, capitalize: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    let mut capitalize = capitalize;
    let data_len = data::latin::WORDS.len();

    for i in 1..=n {
        let index: usize = rng.gen_range(0..data_len);
        let mut word = String::from(data::latin::WORDS[index]);

        if capitalize {
            word.capitalize();
            capitalize = false;
        }

        if i != n && rng.gen_range(1..=100) <= CHANCE_TO_ADD_COMMA {
            word.push(',');
        } else if i != n && rng.gen_range(1..=100) <= CHANCE_TO_ADD_DOT {
            word.push('.');
            capitalize = true;
        } else if i != n && rng.gen_range(1..=100) <= CHANCE_TO_ADD_QUESTION_MARK {
            word.push('?');
        }

        words.push(word);
    }

    let mut buf = words.join(" ");

    if words.len() > 0 {
        buf.push(if rng.gen_range(0..=1) != 0 { '.' } else { '?' });
    }

    buf
}
