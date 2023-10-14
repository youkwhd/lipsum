mod args;
use args::Args;

use core::ops::Range;
use lipsum;

fn main() {
    let args = match Args::parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}.", e);
            std::process::exit(1);
        }
    };

    let lorem_ipsum = lipsum::generate(args.paragraph_length, Range { start: args.words_length, end: args.words_length + 1 }, true);
    println!("{}", lorem_ipsum);
}
