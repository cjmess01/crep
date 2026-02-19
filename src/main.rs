use std::env;
use std::fs;
use std::fs::Metadata;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use regex::Regex;
use std::path::Path;

mod options;
use crate::options::Config;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let (search_target, mtd, pattern, flags) = parse_arguments(args)?;

    if mtd.file_type().is_file() {
        let total_matches =
            crep_file_line_major(&search_target, &pattern).expect("Reading {search_target} failed");
        println!("I am a file");
        println!("Total Matches: {total_matches}");
    } else if mtd.file_type().is_dir() {
        let total_matches = crep_folder(&search_target, &pattern).expect("Error!");
        println!("I am  directory");
        println!("Total Matches: {total_matches}");
    } else if mtd.file_type().is_symlink() {
        let total_matches = 0;
        println!("I am a symbolic link");
        println!("Total Matches: {total_matches}");
    }
    Ok(())
}

fn parse_arguments(args: Vec<String>) -> Result<(PathBuf, Metadata, Regex, Vec<String>), String> {
    let mut search_target: Option<String> = None;
    let mut pattern: Option<String> = None;
    let mut flags: Vec<String> = Vec::new();

    for (_, arg) in args.into_iter().enumerate().skip(1) {
        match (
            arg.starts_with('-'),
            pattern.is_none(),
            search_target.is_none(),
        ) {
            (true, _, _) => flags.push(arg),
            (false, true, _) => pattern = Some(arg),
            (false, false, true) => search_target = Some(arg),
            (false, false, false) => {
                return Err("Usage: crep <pattern> <filename> -flags".to_string());
            }
        }
    }

    // Ensure pettern and target exist
    match (pattern.is_none(), search_target.is_none()) {
        (true, _) => {
            return Err("Missing pattern\nUsage: crep <pattern> <filename> -flags".to_string());
        }
        (_, true) => {
            return Err("Missing pattern\nUsage: crep <pattern> <filename> -flags".to_string());
        }
        (_, _) => {}
    }

    // Translate to regex
    let pattern = match Regex::new(&pattern.unwrap()) {
        Ok(val) => val,
        Err(_) => {
            return Err("Improperly formatted Regex".to_string());
        }
    };

    let search_target = PathBuf::from(search_target.unwrap());
    let mtd = match fs::metadata(&search_target) {
        Ok(mtd) => mtd,
        Err(_) => {
            return Err("File/Folder does not exist.".to_string());
        }
    };

    Ok((search_target, mtd, pattern, flags))
}

// This function scrolls through "line major" or line by line.
// This will not be default behavior but it is good as a first pass
// It will return a vector of "matches"
fn crep_file_line_major(file_path: &Path, pattern: &Regex) -> std::io::Result<u32> {
    let mut total_matches: u32 = 0;

    let fp = fs::File::open(file_path)?;
    let reader = BufReader::new(fp);

    println!(
        "Matches found in {}:",
        file_path
            .to_str()
            .expect("Error converting file path to filename")
    );
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        for ind_match in pattern.find_iter(&line) {
            print_match(
                index,
                ind_match.start(),
                ind_match.end() - ind_match.start(),
                &line,
            );
            total_matches += 1;
        }
    }
    Ok(total_matches)
}

// Searches all files within a directory. Will automatically ignore certain types of files
fn crep_folder(foldername: &Path, pattern: &Regex) -> std::io::Result<u32> {
    let dirp = fs::read_dir(foldername)?;
    let mut result = 0;
    for folder_obj in dirp {
        let folder_obj = folder_obj?;
        if folder_obj.file_type()?.is_file() {
            let file_path = folder_obj.path();
            result += crep_file_line_major(&file_path, pattern)?;
        }
    }
    Ok(result)
}

fn print_match(lineno: usize, colno: usize, length_of_match: usize, text: &str) {
    println!("{lineno}:{text}");
    println!(
        "{}{}",
        " ".repeat(lineno.to_string().len() + 1 + colno),
        "^".repeat(length_of_match)
    );
}
// fn chunk_file(fp: &fs::File) {}
//
