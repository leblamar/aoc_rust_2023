use std::fs::read_to_string;

pub fn main() {
    println!("It's day 9 !!!");
    let sequences = Sequence::create_sequences();

    part1(&sequences);
    part2(&sequences);
}

#[derive(Debug)]
struct Sequence {
    values: Vec<i32>
}

impl Sequence {
    fn create_sequences() -> Vec<Sequence> {
        read_to_string("src/day9_input.txt")
            .unwrap()
            .lines()
            .map(Sequence::parse_sequence)
            .collect::<Vec<Sequence>>()
    }

    fn parse_sequence(line: &str) -> Sequence {
        let values = line.split(' ')
            .filter_map(|value| value.parse().ok())
            .collect::<Vec<i32>>();

        Sequence {
            values
        }
    }

    fn right_resolve(&self) -> i32 {
        let mut are_all_zeros = false;

        let mut right_values = vec![];
        let mut seqs = self.values.clone();
        while !are_all_zeros {
            right_values.push(seqs[seqs.len() - 1]);

            seqs = seqs.iter()
                .zip(seqs[1..].iter())
                .map(|(&left, &right)| right - left)
                .collect::<Vec<i32>>();
            
            are_all_zeros = seqs.iter().all(|&val| val == 0);
        }

        right_values.iter().sum()
    }

    fn left_resolve(&self) -> i32 {
        let mut are_all_zeros = false;

        let mut left_values = vec![];
        let mut seqs = self.values.clone();
        while !are_all_zeros {
            left_values.push(seqs[0]);

            seqs = seqs.iter()
                .zip(seqs[1..].iter())
                .map(|(&left, &right)| right - left)
                .collect::<Vec<i32>>();
            
            are_all_zeros = seqs.iter().all(|&val| val == 0);
        }

        left_values.into_iter()
            .rev()
            .reduce(|acc, val| val - acc)
            .unwrap()
    }
}

fn part1(sequences: &Vec<Sequence>) {
    let result = sequences.iter()
        .map(Sequence::right_resolve)
        .sum::<i32>();

    println!("Part 1: {}", result);
}

fn part2(sequences: &Vec<Sequence>) {
    let result = sequences.iter()
        .map(Sequence::left_resolve)
        .sum::<i32>();

    println!("Part 2: {}", result);
}