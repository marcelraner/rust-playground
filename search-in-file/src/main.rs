use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target_filename: String,
    search_pattern: String
}

fn print_usage() {
    let version: Option<&str> = option_env!("CARGO_PKG_VERSION");
    eprintln!("sif v{} - (s)earch for pattern (i)n (f)ile", version.unwrap_or("unknown"));
    eprintln!("Usage: sif <target_filename> <search_pattern>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        print_usage();
        eprintln!("Error: Wrong number of arguments: expected 2, got {}.", args.len());
        std::process::exit(1);
    }

    Arguments {
        target_filename: args[0].clone(),
        search_pattern: args[1].clone()
    }
}

fn main() {
    let arguments = parse_args();
    // println!("{:?}", arguments);

    let data = match fs::read_to_string(&arguments.target_filename) {
        Ok(value) => value,
        Err(e) => {
            print_usage();
            eprintln!("Error: Failed ro read from file '{}': {:?}", arguments.target_filename, e);
            std::process::exit(1);
        }
    };

    // println!("{:?}", data);

    let regex = match Regex::new(&arguments.search_pattern) {
        Ok(value) => value,
        Err(e) => {
            print_usage();
            eprintln!("Error: Failed to compile pattern '{}': {:?}", arguments.search_pattern, e);
            std::process::exit(1);
        }
    };

    for (index, line) in data.lines().enumerate() {
        for mat in regex.find_iter(&line) {
            println!("Ln {}, Col {}", index + 1, mat.start() + 1);
        }
    }
}
