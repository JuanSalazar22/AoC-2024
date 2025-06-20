mod days;
mod utils;

use std::env;
use crate::days::*;

/// Entry point for the application.
///
/// This function expects a single command-line argument, which is the number of the day to run (1-25).
/// The input file is expected to be in the "inputs" directory, and be named "day<NN>.txt", where <NN>
/// is the two-digit day number.
///
/// If the day number is invalid, or if there is an error reading the input file, an error message is
/// printed to stderr, and the program exits without running the day's code.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    let day = &args[1];
    let input_path = format!("inputs/day{:02}.txt", day.parse::<usize>().unwrap_or(0));

    match utils::utils::read_lines(&input_path) {
        Ok(lines) => {
            let result = match day.as_str() {
                "1" => {
                    run_day_parts(lines, day01::part2, day01::part2)
                }
                "2" => {
                    run_day_parts(lines, day02::part1, day02::part2)
                }
                "3" => {
                    run_day_parts(lines, day03::part1, day03::part2)
                }
                "4" => {
                    run_day_parts(lines, day04::part1, day04::part2)
                }
                "5" => {
                    run_day_parts(lines, day05::part1, day05::part2)
                }
                _ => "Day not implemented!".to_string(),
            };
            println!("{}", result);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn run_day_parts(lines: Vec<String>, part1: fn(Vec<String>) -> String, part2: fn(Vec<String>) -> String) -> String {
    format!("Part 1: {}\nPart 2: {}", part1(lines.clone()), part2(lines))
}
