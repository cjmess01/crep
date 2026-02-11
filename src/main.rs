use std::env;
use std::fs;
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

    let mtd = match fs::metadata(filename) {
        Ok(mtd) => mtd,
        Err(_) => {
            println!("{filename} is not a file or directory");
            exit(100);
        }
    };

    if mtd.file_type().is_file() {
        println!("I am a file");
    } else if mtd.file_type().is_dir() {
        println!("I am a directory");
    } else if mtd.file_type().is_symlink() {
        println!("I am a symbolic link");
    }
}

fn process_file(filename: &str) {
    // Open
}
