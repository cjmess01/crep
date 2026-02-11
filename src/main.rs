use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pulls args[1] and args[2] and sets them to pattern and filename
    // if case of <3 args, exits
    let (pattern, filename) = match args.as_slice() {
        [_, a, b, ..] => (a, b),
        _ => {
            println!("Usage: crep <pattern> <filename> -flags");
            exit(100);
        }
    };
    println!("{pattern}");
    println!("{filename}");
}
