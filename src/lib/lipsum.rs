use crate::data;
use crate::std::string::Capitalize;
use rand::{self, Rng};

pub fn generate(n: u32) -> String {
    const CHANCE_TO_ADD_QUESTION_MARK: i32 = 2;
    const CHANCE_TO_ADD_COMMA: i32 = 10;
    const CHANCE_TO_ADD_DOT: i32 = 5;

    let mut rng = rand::thread_rng();
    let mut words: Vec<String> = Vec::new();
    let mut capitalize = true;
    let data_len = data::latin::WORDS.len();

    for i in 1..=n {
        let index: usize = rng.gen_range(0..data_len);
        let mut word = String::from(data::latin::WORDS[index]);

        if capitalize {
            word = word.capitalize();
        }

        capitalize = false;

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
