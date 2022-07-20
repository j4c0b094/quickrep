use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)] // Allows printing with println!
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickrep".green()
    );
    eprintln!("Usage: quickrep <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect(); // skip 1 for program information & collect produces a Vec of passed args

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expect 4, received {}.",
            "Error".red().bold(),
            args.len().to_string().yellow()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} Failed to read from file '{}: {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} Failed to write to file '{}: {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };
}
