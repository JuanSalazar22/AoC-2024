mod days;
mod utils;

use std::env;
use crate::days::{day01, day02, day03, day04};

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
                    run_day_parts(lines, day02::part2, day02::part2)
                }
                "3" => {
                    run_day_parts(lines, day03::part1, day03::part2)
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
