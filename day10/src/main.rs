use std::fs::read_to_string;

fn parse_input(raw_input: &String) -> (Vec<Vec<char>>, (usize, usize)) {
    println!("{}", raw_input);
    let grid: Vec<Vec<char>> = raw_input.lines().map(|l| l.chars().collect()).collect();
    println!("grid {:?}", grid);
    let mut start_xy: (usize, usize) = (0, 0);
    // let start_xy = grid.iter().enumerate().map(|(r,l)| l.iter().enumerate().);
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_xy = (col, row);
                break;
            }
        }
    }
    println!("start_xy {:?}", start_xy);
    (grid, start_xy)
}

fn main() {
    // let file_path = "../inputs/aoc_10.txt";
    let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let (grid, start_xy): (Vec<Vec<char>>, (usize, usize)) = parse_input(&raw_input);
    println!("{:?} {:?}", grid, start_xy);
    // let camel_card_to_bid: Vec<(Vec<usize>, i32)> = parse_input(raw_input.clone());
}
