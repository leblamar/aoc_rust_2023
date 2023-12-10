use std::fs::read_to_string;
use std::collections::HashMap;

pub fn main() {
    println!("It's day 3 !!!");
    let matrix = Matrix::create_matrix();

    part1(&matrix);
    part2(&matrix);
}

#[derive(Debug, Clone)]
enum Cell {
    Number(u32),
    Gear,
    Symbol,
    Dot
}

impl Cell {
    fn parse_cell(token: char) -> Option<Cell> {
        match token {
            '.' => Some(Cell::Dot),
            '*' => Some(Cell::Gear),
            other => other.to_digit(10).map(Cell::Number).or(Some(Cell::Symbol))
        }
    }

    fn parse_cells(line: &str) -> Vec<Cell> {
        line.chars()
            .filter_map(Cell::parse_cell)
            .collect::<Vec<Cell>>()
    }
}

#[derive(Debug)]
struct Matrix {
    cells: Vec<Vec<Cell>>,
    size: (i32, i32)
}

impl Matrix {
    fn create_matrix() -> Matrix {
        let cells = read_to_string("src/day3_input.txt")
            .unwrap()
            .lines()
            .map(Cell::parse_cells)
            .collect::<Vec<Vec<Cell>>>();

        let size = (cells.len() as i32, cells[0].len() as i32);

        Matrix {
            cells,
            size
        }
    }

    fn get_numbers(&self) -> Vec<Numbers> {
        let mut numbers: Vec<Numbers> = vec![];
        for (j, row) in self.cells.iter().enumerate() {
            let mut cur_poss: Vec<(usize, usize)> = vec![];
            let mut cur_value: u32 = 0;
            let mut last_cell: Option<Cell> = None;
            for (i, cell) in row.into_iter().enumerate() {
                if let Cell::Number(val) = cell {
                    cur_poss.push((j, i));
                    cur_value = 10*cur_value + val;
                } else if let Some(Cell::Number(_)) = last_cell {
                    numbers.push(Numbers { poss: cur_poss, value: cur_value });
                    cur_poss = vec![];
                    cur_value = 0;
                }

                last_cell = Some(cell.clone());
            }

            if !cur_poss.is_empty() {
                numbers.push(Numbers { poss: cur_poss, value: cur_value })
            }
        }

        numbers
    }

    fn is_in_matrix(&self, (j, i): (i32, i32)) -> bool {
        0 <= j && 0 <= i
            && j < self.size.0
            && i < self.size.1
    }

    fn is_valid(&self, number: &Numbers) -> bool {
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for &(j, i) in number.poss.iter() {
            for (dj, di) in directions {
                let r_j = j as i32 + dj;
                let r_i = i as i32 + di;
                if self.is_in_matrix((r_j, r_i)) {
                    let adj_cell = &self.cells[r_j as usize][r_i as usize];
                    if let Cell::Symbol = adj_cell {
                        return true
                    } else if let Cell::Gear = adj_cell {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_valid_gears(&self, number: &Numbers) -> Vec<Gear> {
        let mut gears: Vec<Gear> = Vec::new(); 
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for &(j, i) in number.poss.iter() {
            for (dj, di) in directions {
                let r_j = j as i32 + dj;
                let r_i = i as i32 + di;
                if self.is_in_matrix((r_j, r_i)) {
                    let adj_cell = &self.cells[r_j as usize][r_i as usize];
                    if let Cell::Gear = adj_cell {
                        gears.push(Gear { pos: (r_j as usize, r_i as usize) })
                    }
                }
            }
        }

        gears
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Numbers {
    poss: Vec<(usize, usize)>,
    value: u32
}

fn part1(matrix: &Matrix) {
    let result: u32 = matrix.get_numbers()
        .iter()
        .filter(|&number| matrix.is_valid(number))
        .map(|number| number.value)
        .sum();

    println!("Part 1: {}", result);
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Gear {
    pos: (usize, usize)
}

fn part2(matrix: &Matrix) {
    let numbers = matrix.get_numbers();
    let mut gears_to_numbers: HashMap<Gear, Vec<Numbers>> = HashMap::new();

    for number in numbers {
        matrix.get_valid_gears(&number)
            .iter()
            .for_each(|gear| match gears_to_numbers.get_mut(gear) {
                Some(list) => {
                    if !list.contains(&number) {
                        list.push(number.clone());
                    }
                },
                None => { gears_to_numbers.insert(gear.clone(), vec![number.clone()]); }
            });
    }

    let result: u32 = gears_to_numbers.values()
        .filter(|list| list.len() >= 2)
        .map(|list| list.iter().map(|number| number.value).product::<u32>())
        .sum();

    println!("Part 2: {}", result);
}