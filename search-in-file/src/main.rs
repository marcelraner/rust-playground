use std::env;

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
    println!("{:?}", arguments);
}
