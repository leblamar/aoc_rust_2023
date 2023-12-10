use std::fs::read_to_string;
use std::collections::HashSet;

pub fn main() {
    println!("It's day 4 !!!");
    let cards = read_to_string("src/day4_input.txt")
        .unwrap()
        .lines()
        .filter_map(Card::parse_card)
        .collect::<Vec<Card>>();

    part1(&cards);
    part2(&cards);
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    numbers: Vec<usize>
}

impl Card {
    fn parse_card(line: &str) -> Option<Card> {
        let (win_str, num_str) = line.split_once(": ")?.1.split_once(" | ")?;

        let winning_numbers = win_str.split(" ")
            .filter_map(|num_str| num_str.parse().ok())
            .collect::<HashSet<usize>>();

        let numbers = num_str.split(" ")
            .filter_map(|num_str| num_str.parse().ok())
            .collect::<Vec<usize>>();

        Some(Card {
            winning_numbers,
            numbers
        })
    }
}

fn part1(cards: &Vec<Card>) {
    let result: u32 = cards.iter()
        .map(|card| card.numbers
            .iter()
            .filter(|&num| card.winning_numbers.contains(num))
            .count()
        ).filter(|&val| val != 0)
        .map(|val| 1 << val - 1)
        .sum();

    println!("Part 1: {}", result);
}


fn part2(cards: &Vec<Card>) {
    let mut result_list: Vec<usize> = vec![1; cards.len()];
    for (idx, card) in cards.iter().enumerate() {
        let cur_res = result_list[idx];
        card.numbers
            .iter()
            .filter(|&num| card.winning_numbers.contains(num))
            .enumerate()
            .for_each(|(rel_idx, _)| result_list[idx + rel_idx + 1] += cur_res)
    }

    let result: usize = result_list.iter().sum();

    println!("Part 2: {}", result);
}