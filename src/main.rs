use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = env::args().collect();

    let (pattern, filename) = match args.as_slice() {
        [_, a, b, ..] => (a, b),
        _ => {
            eprintln!("Usage: crep <pattern> <filename> -flags");
            return std::process::ExitCode::from(2);
        }
    };

    let mtd = match fs::metadata(filename) {
        Ok(mtd) => mtd,
        Err(_) => {
            eprintln!("{filename} is not a file or directory");
            return std::process::ExitCode::from(2);
        }
    };

    let pattern = match Regex::new(pattern) {
        Ok(p) => p,
        Err(_) => {
            eprintln!("{pattern} is not a valid regex");
            return std::process::ExitCode::from(2);
        }
    };

    println!("Pattern requested: {pattern}");

    if mtd.file_type().is_file() {
        let total_matches = crep_file_line_major(filename, &pattern).expect("Error!");
        println!("Total Matches: {total_matches}");
    } else if mtd.file_type().is_dir() {
        println!("I am a directory");
    } else if mtd.file_type().is_symlink() {
        println!("I am a symbolic link");
    }
    return std::process::ExitCode::from(0);
}

// This function scrolls through "line major" or line by line.
// This will not be default behavior but it is good as a first pass
// It will return a vector of "matches"
fn crep_file_line_major(filename: &str, pattern: &Regex) -> std::io::Result<u32> {
    let mut total_matches: u32 = 0;

    let fp = fs::File::open(filename)?;
    let reader = BufReader::new(fp);

    println!("Matches found in {filename}:");
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        for ind_match in pattern.find_iter(&line) {
            print_match(
                index,
                ind_match.start(),
                ind_match.end() - ind_match.start(),
                &line,
            );
        }
    }
    Ok(total_matches)
}

// Searches all files within a directory. Will automatically ignore certain types of files
// fn crep_folder(foldername: &str, pattern: &str) -> std::io::Result<u32> {
//     Ok(5)
// }

fn print_match(lineno: usize, colno: usize, length_of_match: usize, text: &str) {
    println!("{lineno}:{text}");
    println!(
        "{}{}",
        " ".repeat(lineno.to_string().len() + 1 + colno),
        "^".repeat(length_of_match)
    );
}
// fn chunk_file(fp: &fs::File) {}
