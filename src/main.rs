use std::io;

fn main() {
    println!("Usage: crep <pattern> <filename> -flags");
    println!("Flags:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Query: {input}");

    // A comment
}
