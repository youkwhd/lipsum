use pico_args;

pub struct Args {
    pub length: u32,
}

impl Args {
    fn print_help_and_exit(exit_code: i32) {
        println!(
"\
Usage: lipsum [-n]
Lorem Ipsum generator.

Options:
  -n NUMBER       n words to generate");
        std::process::exit(exit_code);
    }

    pub fn parse() -> Result<Args, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            Args::print_help_and_exit(0);
        }

        let args = Args {
            length: pargs.value_from_str("-n").or(Ok(10))?,
        };

        Ok(args)
    }
}
