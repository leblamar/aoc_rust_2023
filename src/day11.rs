use std::fs::read_to_string;
use std::collections::HashSet;
use itertools::Itertools;

pub fn main() {
    println!("It's day 11 !!!");
    let universe = &mut Universe::create_universe();

    part1(universe);
    part2(universe);
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
    max_x: usize,
    max_y: usize
}

impl Universe {
    fn create_universe() -> Self {
        let galaxies = read_to_string("src/day11_input.txt")
            .unwrap()
            .lines()
            .enumerate()
            .flat_map(Universe::parse_row)
            .collect::<Vec<(usize, usize)>>();

        let max_x = galaxies.iter()
            .map(|&(_, x)| x)
            .max()
            .unwrap();

        let max_y = galaxies.iter()
            .map(|&(y, _)| y)
            .max()
            .unwrap();

        let x_values = galaxies
            .iter()
            .map(|&(_, x)| x)
            .collect::<HashSet<usize>>();
        let y_values = galaxies
            .iter()
            .map(|&(y, _)| y)
            .collect::<HashSet<usize>>();
         
        let empty_rows = (0..=max_y)
            .filter(|y| !y_values.contains(y))
            .collect::<Vec<usize>>();
        let empty_columns = (0..=max_x)
            .filter(|x| !x_values.contains(x))
            .collect::<Vec<usize>>();

        Universe {
            galaxies,
            empty_rows,
            empty_columns,
            max_x,
            max_y
        }
    }

    fn parse_row((y, line): (usize, &str)) -> Vec<(usize, usize)> {
        line.chars()
            .enumerate()
            .filter(|&(_, char)| char == '#')
            .map(|(x, _)| (y, x))
            .collect::<Vec<(usize, usize)>>()
    }

    fn expanded_universe(&self, mult_factor: usize) -> Universe {
        let factor = mult_factor - 1;
        let expended_max_y = self.max_y + self.empty_rows.len();
        let expended_max_x = self.max_x + self.empty_columns.len();
        let mut expended_galaxies: Vec<(usize, usize)> = Vec::with_capacity(self.galaxies.len());
        for &(y, x) in self.galaxies.iter() {
            let expended_y = y + self.empty_rows.iter()
                .take_while(|&&row| row < y)
                .count() * factor;
            let expended_x = x + self.empty_columns.iter()
                .take_while(|&&column| column < x)
                .count() * factor;
            expended_galaxies.push((expended_y, expended_x));
        }

        Universe { 
            galaxies: expended_galaxies, 
            empty_rows: vec![], 
            empty_columns: vec![], 
            max_x: expended_max_x, 
            max_y: expended_max_y 
        }
    }

    fn compute_distances_sum(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|pair| (*pair[0], *pair[1]))
            .map(|(pos_1, pos_2)| distance_between(pos_1, pos_2))
            .sum()
    }
}

fn distance_between((y_1, x_1): (usize, usize), (y_2, x_2): (usize, usize)) -> usize {
    (y_2 as i32 - y_1 as i32).abs() as usize 
        + (x_2 as i32 - x_1 as i32).abs() as usize
}

fn part1(universe: &mut Universe) {
    let expended_universe = universe.expanded_universe(2);

    let result = expended_universe.compute_distances_sum();

    println!("Part 1: {}", result);
}

fn part2(universe: &Universe) {
    let expended_universe = universe.expanded_universe(1_000_000);

    let result = expended_universe.compute_distances_sum();

    println!("Part 2: {}", result);
}