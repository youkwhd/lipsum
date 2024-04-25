use lipsum;

mod args;
use args::Args;

fn main() {
    let args = Args::parse().unwrap();
    let lorem_ipsum = lipsum::generate(args.words_length);
    println!("{}", lorem_ipsum);
}
