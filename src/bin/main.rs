use clap::{Command, Arg, ArgAction};
use lipsum;

fn parse_command_line_arguments() -> Command {
    Command::new("lipsum")
        .about("lorem ipsum generator")
        .arg(Arg::new("length")
                .value_parser(clap::value_parser!(u32))
                .short('n')
                .action(ArgAction::Set))
}

fn main() {
    let args = parse_command_line_arguments().get_matches();
    let n_word = match args.try_get_one::<u32>("length") {
        Ok(x) => *x.unwrap_or(&10),
        Err(_) => panic!("??????"),
    };

    println!("{}", lipsum::generate(n_word));
}
