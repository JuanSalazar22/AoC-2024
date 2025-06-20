const MAX_DELTA: i32 = 3;
const MAX_ERRORS: i32 = 1;

pub fn is_safe(digits: Vec<i32>) -> bool {
    let is_ascending = digits.first() < digits.last();

    for element_index in 0..(digits.len() - 1) {
        if is_ascending && digits[element_index] > digits[element_index+1] {
            return false
        }

        if !is_ascending && digits[element_index] < digits[element_index+1] {
            return false
        }

        if (digits[element_index] - digits[element_index+1]).abs() > MAX_DELTA ||
            digits[element_index] == digits[element_index+1] {
            return false
        }
    }

    true
}

pub fn is_n_times_unsafe(mut digits: Vec<i32>) -> bool {


    if digits.first() == digits.last(){
        println!("cant define if they are descending or ascending")
    }
    
    let mut index_to_remove = 0;
    let mut number_of_bad_levels = 0;



    let is_ascending = digits.first() < digits.last();

    for element_index in 0..(digits.len() - 1) {
        if is_ascending && digits[element_index] > digits[element_index+1] {
            index_to_remove = element_index;
            digits.remove(index_to_remove);
            break
        }

        if !is_ascending && digits[element_index] < digits[element_index+1] {
            index_to_remove = element_index;
            digits.remove(index_to_remove);
            break
        }

        if (digits[element_index] - digits[element_index+1]).abs() > MAX_DELTA ||
            digits[element_index] == digits[element_index+1] {
            index_to_remove = element_index;
            digits.remove(index_to_remove);
            break
        }
    }


    for element_index in 0..(digits.len() - 1) {
        if is_ascending && digits[element_index] >= digits[element_index+1] {

            //println!("{:?} is unsafe", digits);
            number_of_bad_levels += 1;
        }

        if !is_ascending && digits[element_index] <= digits[element_index+1] {

            //println!("{:?} is unsafe", digits);
            number_of_bad_levels += 1;
        }

        if (digits[element_index] - digits[element_index+1]).abs() > MAX_DELTA {

            println!("{:?} is unsafe, Reason: delta exceeded", digits);
            return false
        }
    }

    if number_of_bad_levels > 0 {
        println!("{:?} is unsafe: too much bad levels", digits);
        return false
    } else {
        println!("{:?} is safe", digits);
        return true
    }
}


pub fn part1(lines: Vec<String>) -> String {
    // Solve part 1

    let mut safe_entries = Vec::<(i32, bool)>::new();

    for line_index in 0..lines.len(){
        let parts = lines[line_index]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap()) // Parse each element to i32
            .collect::<Vec<i32>>(); // Collect into a vector of i32

        //println!("Part {:?}, is safe?: {}", parts, is_safe(parts.clone()));
        safe_entries.push((line_index as i32, is_safe(parts.clone())));

    }

    let number_of_safe_entries = safe_entries.iter()
        .filter(|(_, second)| *second) // Filters tuples where the second element is true // Copies the filtered tuples into a new Vec
        .count();

    number_of_safe_entries.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    // Solve part 2
    let mut safe_entries = Vec::<(i32, bool)>::new();

    for line_index in 0..lines.len(){
        let parts = lines[line_index]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap()) // Parse each element to i32
            .collect::<Vec<i32>>(); // Collect into a vector of i32

        //println!("Part {:?}, is safe?: {}", parts, is_safe(parts.clone()));
        safe_entries.push((line_index as i32, is_n_times_unsafe(parts.clone())));

    }

    let number_of_safe_entries = safe_entries.iter()
        .filter(|(_, second)| *second) // Filters tuples where the second element is lower than 1 // Copies the filtered tuples into a new Vec
        .count();

    number_of_safe_entries.to_string()
}
