use regex::Regex;

pub fn part1(lines: Vec<String>) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut multiplication_accumulator = 0;

    for line in &lines {
        for cap in re.captures_iter(line) {
            //println!("{:?}", cap);
            if let (Ok(a), Ok(b)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                multiplication_accumulator += a * b;
            }
        }
    }

    multiplication_accumulator.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut multiplication_accumulator = 0;
    let mut multiplication_enabled = true;

    for line in &lines {
        for cap in re.captures_iter(line) {
            println!("{:?}", cap);
            if cap[0].eq("do()") {
                multiplication_enabled = true;
                continue
            }

            if cap[0].eq("don't()") {
                multiplication_enabled = false;
                continue
            }

            if multiplication_enabled{
                if let (Ok(a), Ok(b)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                    multiplication_accumulator += a * b;
                }
            }
        }
    }

    multiplication_accumulator.to_string()
}