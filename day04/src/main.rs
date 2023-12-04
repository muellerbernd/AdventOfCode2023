use std::collections::HashSet;
use std::fs::read_to_string;

fn task01(puzzle_input: &Vec<String>) {
    let mut matching_numbers_per_card: Vec<Vec<i32>> = Vec::new();
    for line in puzzle_input {
        let scratchcard: String = line
            .split(": ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        let all_numbers: HashSet<i32> = HashSet::from_iter(
            scratchcard
                .split(" | ")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
        let winnig_numbers: HashSet<i32> = HashSet::from_iter(
            scratchcard
                .split(" | ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
        let matching_numbers: Vec<i32> = all_numbers
            .intersection(&winnig_numbers)
            .map(|n| *n as i32)
            .collect();
        matching_numbers_per_card.push(matching_numbers);
    }
    let points: u32 = matching_numbers_per_card
        .iter()
        .map(|e| e.len())
        .filter(|&x| x > 0)
        .map(|l| u32::pow(2, (l - 1) as u32))
        .sum();
    println!("task 1 solution: {}", points);
}

fn main() {
    // let file_path = "../inputs/aoc_04.txt";
    let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace("  ", " ");
    let puzzle_input: Vec<String> = raw_input.lines().map(|l| l.to_string()).collect();
    task01(&puzzle_input)
}
