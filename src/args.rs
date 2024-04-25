pub struct Args {
    pub words_length: u32,
}

impl Args {
    fn print_help_and_exit(exit_code: i32) {
        println!(
"\
Usage: lipsum <NWORDS>
Lorem Ipsum generator.");

        std::process::exit(exit_code);
    }

    pub fn parse() -> Result<Args, std::io::Error> {
        let args: Vec<String> = std::env::args().collect();

        if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
            Args::print_help_and_exit(0);
        }

        if args.len() <= 1 {
            Args::print_help_and_exit(1);
        }

        Ok(Args { words_length: args[1].parse::<u32>().unwrap_or(10) })
    }
}
