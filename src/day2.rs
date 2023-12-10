use std::fs::read_to_string;

pub fn main() {
    println!("It's day 2 !!!");
    let games = read_to_string("src/day2_input.txt")
        .unwrap()
        .lines()
        .filter_map(Game::parse_game)
        .collect::<Vec<Game>>();

    part1(&games);
    part2(&games);
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>
}

impl Game {
    fn parse_game(line: &str) -> Option<Game> {
        let (game_name, rounds_str) = line.split_once(": ")?;
        let id: usize = game_name.split_once(" ")?.1.parse().unwrap();
        let rounds = rounds_str.split("; ")
            .filter_map(Round::parse_round)
            .collect::<Vec<Round>>();

        Some(Game {
            id,
            rounds
        })
    }

    fn has_enough_blues(&self, max_blues: usize) -> bool {
        self.rounds.iter()
            .all(|round| round.blues <= max_blues)
    }

    fn has_enough_reds(&self, max_reds: usize) -> bool {
        self.rounds.iter()
            .all(|round| round.reds <= max_reds)
    }
    
    fn has_enough_greens(&self, max_greens: usize) -> bool {
        self.rounds.iter()
            .all(|round| round.greens <= max_greens)
    }

    fn max_blues(&self) -> usize {
        self.rounds.iter()
            .map(|round| round.blues)
            .max()
            .unwrap()
    }

    fn max_reds(&self) -> usize {
        self.rounds.iter()
            .map(|round| round.reds)
            .max()
            .unwrap()
    }
    
    fn max_greens(&self) -> usize {
        self.rounds.iter()
            .map(|round| round.greens)
            .max()
            .unwrap()
    }
}

#[derive(Debug)]
struct Round {
    blues: usize,
    greens: usize,
    reds: usize
}

impl Round {
    fn parse_round(round_str: &str) -> Option<Round> {
        let mut round = Round {
            blues: 0,
            greens: 0,
            reds: 0
        };
        for color_and_num in round_str.split(", ") {
            let (num_str, color) = color_and_num.split_once(" ")?;
            let num = num_str.parse().unwrap();

            match color {
                "blue" => round.blues = num,
                "red" => round.reds = num,
                "green" => round.greens = num,
                unk => panic!("Unknown color: {}", unk)
            }
        }

        Some(round)
    }
}


fn part1(games: &Vec<Game>) {
    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;
    let result = games.iter()
        .filter(|game| game.has_enough_blues(max_blues))
        .filter(|game| game.has_enough_reds(max_reds))
        .filter(|game| game.has_enough_greens(max_greens))
        .map(|game| game.id)
        .sum::<usize>();

    println!("Part 1: {}", result);
}


fn part2(games: &Vec<Game>) {
    let result = games.iter()
        .map(|game| game.max_blues() * game.max_greens() * game.max_reds())
        .sum::<usize>();

    println!("Part 2: {}", result);
}