mod args;
use lipsum;

fn main() {
    let args = match args::Args::parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}.", e);
            std::process::exit(1);
        }
    };

    println!("{}", lipsum::generate_words(args.length, true));
}
