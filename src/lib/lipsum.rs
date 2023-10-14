use crate::data;
use crate::std::string::Capitalize;
use rand::{self, Rng};

const CHANCE_TO_ADD_QUESTION_MARK: i32 = 3;
const CHANCE_TO_ADD_COMMA: i32 = 8;
const CHANCE_TO_ADD_DOT: i32 = 5;

pub fn generate(n: u32, starts_with_lorem_ipsum: bool) -> String {
    let common_len = data::latin::COMMONS.len() as u32;

    if starts_with_lorem_ipsum && n <= common_len {
        return generate_commons(n)
    }

    let n = if starts_with_lorem_ipsum {
        n - common_len
    } else {
        n
    };

    let mut buf = if starts_with_lorem_ipsum {
        generate_commons(common_len)
    } else {
        String::from("")
    };

    let mut random_words = String::from(" ");
    random_words.push_str(&generate_words(n, buf.ends_with('.')));

    buf.push_str(&random_words);
    buf 
}

pub fn generate_commons(n: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    let mut capitalize = true;

    for i in 0..n {
        let mut word = String::from(data::latin::COMMONS[i as usize]);

        if capitalize {
            word = word.capitalize();
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
    buf.push(if rng.gen_range(0..=1) != 0 { '.' } else { '?' });
    buf
}

pub fn generate_words(n: u32, capitalize: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    let mut capitalize = capitalize;
    let data_len = data::latin::WORDS.len();

    for i in 1..=n {
        let index: usize = rng.gen_range(0..data_len);
        let mut word = String::from(data::latin::WORDS[index]);

        if capitalize {
            word = word.capitalize();
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
    buf.push(if rng.gen_range(0..=1) != 0 { '.' } else { '?' });
    buf
}
