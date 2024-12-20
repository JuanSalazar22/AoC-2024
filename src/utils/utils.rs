use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?; // Open the file
    let reader = io::BufReader::new(file); // Wrap it in a buffered reader

    // Collect each line into a vector of Strings
    let lines: Vec<String> = reader
        .lines() // Iterator over lines
        .filter_map(Result::ok) // Filter out lines that failed to read
        .collect();

    Ok(lines)
}