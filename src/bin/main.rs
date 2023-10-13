use clap::{command, value_parser, Command, Arg, ArgAction};
use std::env;
use lipsum;

fn parse_command_line_arguments() -> Command {
    command!()
        .about("lorem ipsum generator")
        .ignore_errors(true)
        .arg(Arg::new("__ARG_LENGTH")
                .value_parser(value_parser!(u32))
                .short('n')
                .action(ArgAction::Set))
}

fn main() {
    let args = parse_command_line_arguments().get_matches();
    let n_word = match args.try_get_one::<u32>("__ARG_LENGTH") {
        Ok(x) => *x.unwrap_or(&10),
        Err(_) => unreachable!(),
    };

    println!("{}", lipsum::generate(n_word));
}
