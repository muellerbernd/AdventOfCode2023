use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_input(puzzle_input: &Vec<String>) {
    for (row, line) in puzzle_input.iter().enumerate() {
        dbg!(line.clone());
        for (col, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' && c != '.' {
                println!("char is number")
            }
            dbg!(c);
        }
    }
}

fn main() {
    // let file_path = "../inputs/aoc_03.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let puzzle_input: Vec<String> = raw_input.lines().map(|l| l.to_string()).collect();
    dbg!(puzzle_input.clone());
    parse_input(&puzzle_input)
}
