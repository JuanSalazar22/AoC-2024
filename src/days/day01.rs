pub fn part1(lines: Vec<String>) -> String {
    // Solve part 1

    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    let mut total_distance: i32 = 0;

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        left_list.push(parts[0].parse::<i32>().unwrap());
        right_list.push(parts[1].parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs();
    }

    total_distance.to_string()
}

pub fn part2(lines: Vec<String>) -> String {
    // Solve part 2

    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    let mut total_distance: i32 = 0;

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        left_list.push(parts[0].parse::<i32>().unwrap());
        right_list.push(parts[1].parse::<i32>().unwrap());
    }


    "Solution to part 2".to_string()
}