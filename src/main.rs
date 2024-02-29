use lipsum;

mod args;
use args::Args;

fn main() {
    let args = match Args::parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}.", e);
            std::process::exit(1);
        }
    };

    let lorem_ipsum = lipsum::generate(args.paragraph_length, args.words_length, true);
    println!("{}", lorem_ipsum);
}
