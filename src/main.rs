use clap::{App, Arg, ArgMatches};

mod style_code;
use style_code::StyleCode;

mod tests;

fn parse_command_line<'a>() -> ArgMatches<'a> {
    App::new("servizio-cli by AnFive")
        .version("1.0.0")
        .about("Command-line utility to process INCOM Style Codes")
        .arg(
            Arg::with_name("code")
                .help("The Style Code to decode.")
                .required(true),
        )
        .get_matches()
}

fn main() {
    let matches = parse_command_line();

    let code = matches.value_of("code").unwrap();

    println!("Input code: {}", code);

    match StyleCode::decode(code) {
        None => {
            println!("Invalid style code.");
            std::process::exit(1);
        }
        Some(style) => {
            println!("{}", style.pretty_print());
            std::process::exit(0);
        }
    }
}
