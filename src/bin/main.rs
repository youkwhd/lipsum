use lipsum;
use pico_args;

const HELP_MESSAGE: &str = "\
Usage: lipsum [-n]
Lorem Ipsum generator.

Options:
  -n NUMBER       n words to generate";

#[derive(Debug)]
struct Args {
    length: u32,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP_MESSAGE);
        std::process::exit(0);
    }

    let args = Args {
        length: pargs.value_from_str("-n").or(Ok(10))?,
    };

    Ok(args)
}

fn main() {
    let args: Args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {}.", e);
            std::process::exit(1);
        }
    };

    println!("{}", lipsum::generate(args.length));
}
