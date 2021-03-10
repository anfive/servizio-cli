use clap::{App, Arg};

mod style_code;
use style_code::StyleCode;
mod file_processing;
use file_processing::process_file;

mod tests;

static NAME: &str = "servizio-cli by AnFive";
static VERSION: &str = "1.0.0";
static ABOUT: &str = "Command-line utility to process INCOM Style Codes";

fn create_clap_app<'a>() -> App<'a, 'a> {
    App::new(NAME)
        .version(VERSION)
        .about(ABOUT)
        .arg(Arg::with_name("code").help("The Style Code to decode."))
        .arg(
            Arg::with_name("encode")
                .help("String containing Style judgement to encode")
                .long("encode")
                .short("e")
                .takes_value(true)
                .conflicts_with("code"),
        )
        .arg(
            Arg::with_name("value")
                .help("The requested value")
                .long("value")
                .short("v")
                .takes_value(true)
                .requires("code"),
        )
        .arg(
            Arg::with_name("raw")
                .help("Output raw data to standard output (useful for scripts)")
                .long("raw")
                .short("r"),
        )
        .arg(
            Arg::with_name("infile")
                .help("(File processing mode) input file to process ")
                .long("infile")
                .requires("outfile")
                .takes_value(true)
                .conflicts_with("encode"),
        )
        .arg(
            Arg::with_name("outfile")
                .help("(File processing mode) output file to write ")
                .long("outfile")
                .takes_value(true)
                .requires("infile"),
        )
        .arg(
            Arg::with_name("headers")
                .help("(File processing mode) output file to write ")
                .long("headers")
                .requires("infile"),
        )
        .arg(
            Arg::with_name("column")
                .help("(File processing mode) specifies the index of the column to decode (zero-based). Default is the last colums, as determined by the first row.")
                .long("column")
                .min_values(0)
                .takes_value(true)
                .requires("infile"),
        )
        .arg(
            Arg::with_name("delimiter")
                .help("(File processing mode) specifies the delimiter in the CSV files. Default is comma")
                .long("delimiter")
                .takes_value(true)
                .requires("infile"),
        )
        .arg(
            Arg::with_name("version")
                .help("Prints the version.")
                .long("version")
                .short("V")
        )
}

fn extract_code(string: &str) -> Option<StyleCode> {
    let mut out = StyleCode {
        bas: 0,
        mov: 0,
        din: 0,
        com: 0,
        sapd: 0,
        gcc: 0,
        dif: 0,
        sog: 0,
        pen: 0,
    };
    for key_val in string.split(",") {
        let split: Vec<&str> = key_val.split("=").collect();
        if split.len() != 2 {
            return None;
        }

        if let Ok(value) = split[1].parse::<u32>() {
            let field: &mut u32 = match split[0].to_ascii_lowercase().as_ref() {
                "bas" => &mut out.bas,
                "mov" => &mut out.mov,
                "din" => &mut out.din,
                "com" => &mut out.com,
                "sapd" => &mut out.sapd,
                "gcc" => &mut out.gcc,
                "dif" => &mut out.dif,
                "sog" => &mut out.sog,
                "pen" => &mut out.pen,
                _ => return None,
            };
            *field = value;
        } else {
            return None;
        }
    }

    if out.valid() {
        Some(out)
    } else {
        None
    }
}

fn main() {
    let app = create_clap_app();
    let matches = app.get_matches();

    if matches.is_present("version") {
        println!("{} {}", NAME, VERSION);
        println!("{}", ABOUT);
        std::process::exit(0);
    }

    let raw = matches.is_present("raw");

    if let Some(code) = matches.value_of("code") {
        // Decode mode
        if !raw {
            println!("Decoding input code: {}", code);
        }

        match StyleCode::decode(code) {
            None => {
                if !raw {
                    println!("Invalid style code.");
                }
                std::process::exit(1);
            }
            Some(style) => {
                match matches.value_of("value") {
                    Some(requested_value) => {
                        let out = match requested_value.to_ascii_lowercase().as_ref() {
                            "score" => style.score().to_string(),
                            "bas" => style.bas.to_string(),
                            "mov" => style.mov.to_string(),
                            "din" => style.din.to_string(),
                            "com" => style.com.to_string(),
                            "sapd" => style.sapd.to_string(),
                            "gcc" => style.gcc.to_string(),
                            "dif" => style.dif.to_string(),
                            "sog" => style.sog.to_string(),
                            "pen" => style.pen.to_string(),
                            _ => {
                                if !raw {
                                    println!("Invalid requested value: {}", requested_value);
                                }
                                std::process::exit(3);
                            }
                        };
                        if raw {
                            println!("{}", out);
                        } else {
                            println!("{} : {}", requested_value.to_ascii_lowercase(), out);
                        }
                    }
                    None => {
                        if raw {
                            println!("{}", style.raw_print());
                        } else {
                            println!("{}", style.pretty_print());
                        }
                    }
                }
                std::process::exit(0);
            }
        }
    } else if let Some(encode_string) = matches.value_of("encode") {
        // Encode mode
        if !raw {
            println!("Encode: {}", encode_string);
        }

        if let Some(encode_code) = extract_code(encode_string) {
            println!("{}", encode_code.encode());
            std::process::exit(0);
        } else {
            std::process::exit(2);
        }
    } else if let Some(infile) = matches.value_of("infile") {
        // File processing mode
        let outfile = matches.value_of("outfile").unwrap();
        let has_headers = matches.is_present("headers");
        let column_index = matches
            .value_of("column")
            .and_then(|s| s.parse::<usize>().ok());

        let delimiter = match matches.value_of("delimiter") {
            Some(d) => {
                let bytes = &d.as_bytes();
                if bytes.len() != 1 {
                    if !raw {
                        println!("Invalid delimiter");
                    }
                    std::process::exit(4);
                }
                bytes[0] as char
            }
            None => ',',
        };

        if let Err((msg, err_code)) =
            process_file(infile, outfile, delimiter, has_headers, column_index)
        {
            println!("An error occurred: {}", msg);
            std::process::exit(err_code);
        } else {
            println!("Processing completed.");
            std::process::exit(0);
        }
    }
}
