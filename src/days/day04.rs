use regex::Regex;

const WORD: &str = "XMAS";

pub fn part1(lines: Vec<String>) -> String {

    let directions: Vec<(i8, i8)> = vec![(-1, -1), (-1, 0), (-1, 1), ( 0, -1), ( 0, 1), ( 1, -1), ( 1, 0), ( 1, 1)];

    let matrix_of_characters: Vec<Vec<char>> = lines.iter().map(|row| {
        row.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    //print!("{:?}", matrix_of_characters);

    for row in matrix_of_characters{
        for character in row{

            for direction in directions{

            }





        }
    }

    
    "Day 4 Part 1".to_string()


}

pub fn part2(lines: Vec<String>) -> String {

    let directions: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), ( 0, -1), ( 0, 1), ( 1, -1), ( 1, 0), ( 1, 1)];

    print!("{:?}", directions);
    "Day 4 Part 2".to_string()
}