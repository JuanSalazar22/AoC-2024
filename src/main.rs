mod days;
mod utils;

use std::env;
use std::fs;

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
                    let result1 = days::day01::part1(lines.clone());
                    let result2 = days::day01::part2(lines);
                    format!("Part 1: {}\nPart 2: {}", result1, result2)
                }
                _ => "Day not implemented!".to_string(),
            };
            println!("{}", result);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
