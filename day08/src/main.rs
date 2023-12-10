use std::collections::HashMap;
use std::fs::read_to_string;
use std::usize;

fn parse_input(raw_input: &String) -> (String, HashMap<String, Vec<String>>) {
    let (navigation, parsed_network) = raw_input.split_once("\n\n").unwrap();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    for n in parsed_network.lines() {
        let (k, v) = n.split_once(" = ").unwrap();
        let vals: Vec<String> = v.split(", ").map(|x| x.to_string()).collect();
        network.insert(k.to_string(), vals);
    }
    (navigation.to_string(), network)
}

fn task01(navigation: &String, network: &HashMap<String, Vec<String>>) {
    let nav: Vec<i32> = navigation
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => -1,
        })
        .collect();
    let mut way_points: Vec<String> = Vec::new();
    let mut curr_waypoint: String = "AAA".to_string();
    let goal: String = "ZZZ".to_string();
    let mut goal_reached = false;
    let mut nav_counter = 0;
    while !goal_reached {
        let dir: usize = nav[nav_counter % nav.len()] as usize;
        way_points.push(curr_waypoint.to_string());
        let next_waypoint: String = network.get(&curr_waypoint).unwrap()[dir].to_string();
        if curr_waypoint == goal.to_string() {
            println!("goal_reached");
            goal_reached = true;
        } else {
            curr_waypoint = next_waypoint;
            nav_counter += 1;
        }
    }
    println!("task01 goal was reached after {} steps", nav_counter);
}

fn main() {
    let file_path = "../inputs/aoc_08.txt";
    // let file_path = "sample_input.txt";

    let mut raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    raw_input = raw_input.replace("(", "").replace(")", "");
    let (navigation, network): (String, HashMap<String, Vec<String>>) = parse_input(&raw_input);
    task01(&navigation, &network);
    // let camel_card_to_bid: Vec<(Vec<usize>, i32)> = parse_input(raw_input.clone());
    // let task01_solution = task01(camel_card_to_bid);
    // println!("task01 solution = {}", task01_solution);
    // let camel_card_to_bid_task02: Vec<(Vec<usize>, i32)> = parse_input_task02(raw_input.clone());
    // let task02_solution = task02(camel_card_to_bid_task02);
    // println!("task02 solution = {}", task02_solution);
}
