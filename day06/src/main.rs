use regex::Regex;
use std::fs::read_to_string;
use std::iter::zip;

fn parse_input(raw_input: &String) -> Vec<Vec<usize>> {
    let re = Regex::new(r"\s+").unwrap();
    let race_description: Vec<String> = raw_input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            re.replace_all(l.split_once(":").unwrap().1.trim(), " ")
                .to_string()
        })
        .collect::<Vec<String>>();
    let time_to_dist: Vec<Vec<usize>> = race_description
        .iter()
        .map(|l| l.split(" ").map(|v| v.parse::<usize>().unwrap()).collect())
        .collect();
    time_to_dist
}

fn task01(time_to_dist: &Vec<Vec<usize>>) -> usize {
    println!("{:?}", time_to_dist);
    for (time, dist) in zip(&time_to_dist[0], &time_to_dist[1]) {
        println!("{} {}", time, dist);
    }
    0
}

fn main() {
    // let file_path = "../inputs/aoc_06.txt";
    let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let time_to_dist: Vec<Vec<usize>> = parse_input(&raw_input);
    let task01_solution = task01(&time_to_dist);
    println!("task01 solution = {}", task01_solution);
    // let task02_solution = task02(&parsed_input);
    // println!("task02 solution = {}", task02_solution);
}
