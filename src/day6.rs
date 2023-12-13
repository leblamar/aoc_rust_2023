use std::fs::read_to_string;

pub fn main() {
    println!("It's day 6 !!!");

    part1();
    part2();
}

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64
}

impl Race {
    fn create_races_1() -> Vec<Race> {
        let race_list = read_to_string("src/day6_input.txt")
            .unwrap()
            .lines()
            .map(|line| line.split(" ")
                .filter_map(|val| val.parse().ok())
                .collect::<Vec<u64>>()
            )
            .collect::<Vec<Vec<u64>>>();
        
        let races = race_list[0].iter()
            .zip(race_list[1].iter())
            .map(Race::create_race)
            .collect::<Vec<Race>>();

        races
    }

    fn create_races_2() -> Vec<Race> {
        let race_list = read_to_string("src/day6_input.txt")
            .unwrap()
            .lines()
            .map(|line| line.replace(" ", "")
                .split(":")
                .filter_map(|val| val.parse().ok())
                .collect::<Vec<u64>>()
            )
            .collect::<Vec<Vec<u64>>>();
        
        let races = race_list[0].iter()
            .zip(race_list[1].iter())
            .map(Race::create_race)
            .collect::<Vec<Race>>();

        races
    }

    fn create_race((&time, &dist): (&u64, &u64)) -> Race {
        Race { time, dist }
    }
    
    fn count_nb_opti(&self) -> usize {
        (0..self.time)
            .filter(|i| i * (self.time - i) > self.dist)
            .count()
    }

    fn count_ultra_opti(&self) -> u64 {
        let delta = self.time.pow(2) as i64 - 4 * self.dist as i64;
        if delta < 0 {
            return 0;
        } else if delta == 0 {
            return 1;
        }

        let d1 = (self.time - (delta as f64).sqrt() as u64) / 2;
        let d2 = (self.time + (delta as f64).sqrt() as u64) / 2;
        d2 - d1 + 1
    }
}

fn part1() {
    let races = Race::create_races_1();
    let result: usize = races.iter()
        .map(Race::count_nb_opti)
        .product();

    println!("Part 1: {}", result);
}

fn part2() {
    let races = Race::create_races_2();
    let result: u64 = races.iter()
        .map(Race::count_ultra_opti)
        .product();

    println!("Part 2: {}", result);
}