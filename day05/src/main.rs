use std::fs::read_to_string;

fn parse_input(raw_input: &String) -> Vec<Vec<Vec<usize>>> {
    raw_input
        .split("\n\n")
        .map(|l| {
            l.split_once(":")
                .unwrap()
                .1
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|l| l.split(" ").map(|e| e.parse::<usize>().unwrap()).collect())
                .collect()
        })
        .collect::<Vec<_>>()
}

fn task01(puzzle_input: &Vec<Vec<Vec<usize>>>) -> usize {
    let seeds: Vec<usize> = puzzle_input[0][0].clone();
    let maps = &puzzle_input[1..];

    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut scratch_var = seed;
        for mappings in maps {
            for m in mappings {
                let (dest_start, src_start, range): (usize, usize, usize) = (m[0], m[1], m[2]);
                if scratch_var >= src_start && scratch_var < (src_start + range) {
                    scratch_var = dest_start + (scratch_var - src_start);
                    break;
                }
            }
        }
        locations.push(scratch_var);
    }
    *locations.iter().min().unwrap()
}

fn main() {
    let file_path = "../inputs/aoc_05.txt";
    // let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace(": ", ":");
    println!("{}", raw_input);
    let parsed_input = parse_input(&raw_input);
    let task01_solution = task01(&parsed_input);
    println!("task01 solution = {}", task01_solution);
    // task02(&puzzle_input);
}
