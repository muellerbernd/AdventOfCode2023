use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

const RELATIVE_STRENGTHS: [char; 13] = [
    // 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn parse_input(raw_input: &String) -> Vec<(Vec<char>, i32)> {
    let v: Vec<(Vec<char>, i32)> = raw_input
        .lines()
        .map(|l| {
            let camel_cards_raw: Vec<&str> = l.split(" ").collect();
            (
                camel_cards_raw[0].chars().collect(),
                camel_cards_raw[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    v
}

fn task01(camel_card_to_bid: &Vec<(Vec<char>, i32)>) -> i32 {
    let rankings: Vec<Vec<i32>> = Vec::new();
    for (hand, bid) in camel_card_to_bid {
        let letter_counts: HashMap<char, i32> = hand.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        let hand_type: i32 = match letter_counts.len() {
            5 => {
                println!("High card");
                0
            }
            4 => {
                println!("1 pair");
                1
            }
            3 => match letter_counts.values().max().unwrap() {
                3 => {
                    println!("three of a kind");
                    3
                }
                _ => {
                    println!("2 Pair");
                    2
                }
            },
            2 => match letter_counts.values().max().unwrap() {
                4 => {
                    println!("Four of a kind");
                    5
                }
                _ => {
                    println!("Full house");
                    4
                }
            },
            1 => {
                println!("Five of a kind");
                6
            }
            _ => {
                println!("default");
                -1
            }
        };
        let score: usize = hand
            .iter()
            .enumerate()
            .map(|(i, c)| {
                (RELATIVE_STRENGTHS.iter().position(|r| r == c).unwrap() + 1) * (hand.len() - i)
            })
            .sum::<usize>();
        // println!("{:?} {:?}", hand, bid);
        println!(
            "hand {:?} hand_type {:?} score {:?}",
            hand, hand_type, score
        );
    }
    0
}

fn task02(time_to_dist: &Vec<Vec<usize>>) -> i32 {
    0
}
fn main() {
    // let file_path = "../inputs/aoc_07.txt";
    let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let camel_card_to_bid: Vec<(Vec<char>, i32)> = parse_input(&raw_input);
    let task01_solution = task01(&camel_card_to_bid);
    // println!("task01 solution = {}", task01_solution);
    // let task02_solution = task02(&time_to_dist);
    // println!("task02 solution = {}", task02_solution);
}
