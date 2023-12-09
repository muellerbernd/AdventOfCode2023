use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

const RELATIVE_STRENGTHS: [char; 13] = [
    // 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn parse_input(raw_input: &String) -> Vec<(Vec<usize>, i32)> {
    let v: Vec<(Vec<usize>, i32)> = raw_input
        .lines()
        .map(|l| {
            let camel_cards_raw: Vec<&str> = l.split(" ").collect();
            (
                camel_cards_raw[0]
                    .chars()
                    .map(|c| RELATIVE_STRENGTHS.iter().position(|&r| r == c).unwrap() + 1)
                    .collect(),
                camel_cards_raw[1].parse::<i32>().unwrap(),
            )
        })
        .collect();
    v
}

fn task01(camel_card_to_bid: &Vec<(Vec<usize>, i32)>) -> i32 {
    let mut all_hands: BTreeMap<i32, Vec<(Vec<usize>, i32)>> = BTreeMap::new();
    for (hand, bid) in camel_card_to_bid {
        let letter_counts: HashMap<usize, i32> = hand.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        let hand_type: i32 = match letter_counts.len() {
            5 => {
                // println!("High card");
                0
            }
            4 => {
                // println!("1 pair");
                1
            }
            3 => match letter_counts.values().max().unwrap() {
                3 => {
                    // println!("three of a kind");
                    3
                }
                _ => {
                    // println!("2 Pair");
                    2
                }
            },
            2 => match letter_counts.values().max().unwrap() {
                4 => {
                    // println!("Four of a kind");
                    5
                }
                _ => {
                    // println!("Full house");
                    4
                }
            },
            1 => {
                // println!("Five of a kind");
                6
            }
            _ => {
                println!("default");
                -1
            }
        };
        match all_hands.get_mut(&hand_type) {
            Some(v) => v.push((hand.to_vec(), *bid)),
            // _=> println!("not found")
            _ => {
                let mut v: Vec<(Vec<usize>, i32)> = Vec::new();
                v.push((hand.to_vec(), *bid));
                all_hands.insert(hand_type, v);
            }
        };
    }
    for (_, v) in all_hands.iter_mut().rev() {
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
    all_hands
        .values()
        .flatten()
        .enumerate()
        .map(|(i, v)| v.1 * (i + 1) as i32)
        .sum()
}

// fn task02(time_to_dist: &Vec<Vec<usize>>) -> i32 {
//     0
// }
fn main() {
    let file_path = "../inputs/aoc_07.txt";
    // let file_path = "sample_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let camel_card_to_bid: Vec<(Vec<usize>, i32)> = parse_input(&raw_input);
    let task01_solution = task01(&camel_card_to_bid);
    println!("task01 solution = {}", task01_solution);
    // let task02_solution = task02(&time_to_dist);
    // println!("task02 solution = {}", task02_solution);
}
