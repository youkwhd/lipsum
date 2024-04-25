use crate::data;
use crate::std::string::Capitalize;
use rand::{self, Rng};

pub fn generate(nwords: u32) -> String {
    let mut nwords = nwords;
    let mut capitalize = true;

    if nwords <= 0 {
        return String::from("");
    }

    let common_latin_words_len = data::latin::COMMONS.len();
    let latin_words_len = data::latin::WORDS.len();
    let mut words: Vec<String> = Vec::new();

    let nwords_total = nwords;
    let mut rng = rand::thread_rng();

    for i in 0..common_latin_words_len {
        let mut word = String::from(data::latin::COMMONS[i]);

        if capitalize {
            word.capitalize();
            capitalize = false;
        }

        if i != common_latin_words_len - 1 && i as u32 != nwords_total - 1 {
            let mark: char = generate_punctuation_mark();
            if mark != char::from_u32(0).unwrap() {
                word.push(mark);
            }

            capitalize = mark == '.';
        }

        words.push(word);

        nwords -= 1;
        if nwords == 0 {
            let mut buf: String = words.join(" ");
            buf.push(if rng.gen_range(0..=10) == 0 { '!' } else { '.' });
            return buf;
        }
    }

    for i in 0..nwords {
        let index: usize = rng.gen_range(0..latin_words_len);
        let mut word = String::from(data::latin::WORDS[index]);

        if capitalize {
            word.capitalize();
            capitalize = false;
        }

        if i != nwords - 1 {
            let mark: char = generate_punctuation_mark();
            if mark != char::from_u32(0).unwrap() {
                word.push(mark);
            }

            capitalize = mark == '.';
        }

        words.push(word);
    }

    let mut buf: String = words.join(" ");
    buf.push(if rng.gen_range(0..=10) == 0 { '!' } else { '.' });
    buf
}

fn generate_punctuation_mark() -> char {
    const CHANCE_TO_ADD_QUESTION_MARK: u32 = 3;
    const CHANCE_TO_ADD_EXCLAMATION_MARK: u32 = 2;
    const CHANCE_TO_ADD_COMMA: u32 = 8;
    const CHANCE_TO_ADD_DOT: u32 = 5;

    let mut rng = rand::thread_rng();

    if rng.gen_range(1..=100) <= CHANCE_TO_ADD_DOT {
        return '.';
    } else if rng.gen_range(1..=100) <= CHANCE_TO_ADD_COMMA {
        return ',';
    } else if rng.gen_range(1..=100) <= CHANCE_TO_ADD_QUESTION_MARK {
        return '?';
    } else if rng.gen_range(1..=100) <= CHANCE_TO_ADD_EXCLAMATION_MARK {
        return '!';
    }

    return char::from_u32(0).unwrap();
}
