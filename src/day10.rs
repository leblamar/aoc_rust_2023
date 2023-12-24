use std::fs::read_to_string;

pub fn main() {
    println!("It's day 10 !!!");
    let map = &mut Map::create_map();

    part1(map);
    part2(map);
}

#[derive(Debug)]
struct Map {
    pipes: Vec<Vec<Pipe>>,
    start: (usize, usize),
    all_nodes: Vec<(usize, usize)>
}

#[derive(Debug, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    West,
    South
}

impl Map {
    fn create_map() -> Map {
        let mut pipes = read_to_string("src/day10_input.txt")
            .unwrap()
            .lines()
            .map(Map::parse_row)
            .collect::<Vec<Vec<Pipe>>>();

        let mut start = (0, 0);
        'first: for j in 0..pipes.len() {
            for i in 0..pipes[0].len() {
                if let Pipe::Start = pipes[j][i] {
                    start = (j, i);
                    break 'first;
                }
            }
        }

        let start_pipe = Map::resolve_start(start, &pipes);
        pipes[start.0][start.1] = start_pipe;

        Map {
            pipes,
            start,
            all_nodes: vec![]
        }
    }

    fn resolve_start(start: (usize, usize), pipes: &Vec<Vec<Pipe>>) -> Pipe {
        let north = if start.0 == 0 { false } 
            else { pipes[start.0 - 1][start.1].connect_south() };
        let west = if start.1 == 0 { false } 
            else { pipes[start.0][start.1 - 1].connect_east() };
        let south = pipes[start.0 + 1][start.1].connect_north();
        let east = pipes[start.0][start.1 + 1].connect_west();

        match (north, south, east, west) {
            (true, true, false, false) => Pipe::Vertical,
            (true, false, true, false) => Pipe::NorthEast,
            (true, false, false, true) => Pipe::NorthWest,
            (false, true, true, false) => Pipe::SouthEast,
            (false, true, false, true) => Pipe::SouthWest,
            (false, false, true, true) => Pipe::Horizontal,
            other => panic!("Should not append: {:?}", other)
        }
    }

    fn parse_row(line: &str) -> Vec<Pipe> {
        line.chars()
            .map(Pipe::from)
            .collect::<Vec<Pipe>>()
    }

    fn get_start(&self) -> Pipe {
        self.pipes[self.start.0][self.start.1].clone()
    }

    fn get_next_from(
        &self, 
        cur_idx: (usize, usize), 
        cur_dir: Dir
    ) -> ((usize, usize), Dir) {
        let next_idx = match cur_dir {
            Dir::North => (cur_idx.0 - 1, cur_idx.1),
            Dir::South => (cur_idx.0 + 1, cur_idx.1),
            Dir::East => (cur_idx.0, cur_idx.1 + 1),
            Dir::West => (cur_idx.0, cur_idx.1 - 1) 
        };

        let next_dir = self.pipes[next_idx.0][next_idx.1].get_next_dir(cur_dir.get_opp_dir());

        (next_idx, next_dir)
    }

    fn walk_from_start(&mut self) {
        let mut cur_idx = self.start;
        let mut is_back = false;
        let mut cur_dir = self.get_start().get_dirs().0;
        self.all_nodes.push(cur_idx.clone());

        while !is_back {
            (cur_idx, cur_dir) = self.get_next_from(cur_idx, cur_dir);

            is_back = self.start.eq(&cur_idx);

            if !is_back {
                self.all_nodes.push(cur_idx.clone());
            }
        }
    }

    fn resolve_shoelace_points(&mut self) {
        if self.all_nodes.len() == 0 {
            self.walk_from_start();
        }

        self.all_nodes = self.all_nodes
            .iter()
            .filter(|(y, x)| self.pipes[*y][*x].shoelace_filter())
            .map(|&pos| pos)
            .collect::<Vec<(usize, usize)>>();
    }

    fn compute_segment((&(y_i, x_i), &(y_i1, x_i1)): (&(usize, usize), &(usize, usize))) -> i32 {
        (y_i + y_i1) as i32 * (x_i as i32 - x_i1 as i32)
    }

    fn compute_shoelace_formula(&mut self) -> i32 {
        let nb_points = self.all_nodes.len();
        // Only used for performance purposes to avoid having all ponits but only vertices
        self.resolve_shoelace_points();

        let all_nodes_1 = self.all_nodes
            .iter()
            .filter(|(y, x)| self.pipes[*y][*x].shoelace_filter())
            .cycle()
            .skip(1)
            .take(self.all_nodes.len());

        let segment_sum = self.all_nodes.iter()
            .filter(|(y, x)| self.pipes[*y][*x].shoelace_filter())
            .zip(all_nodes_1)
            .map(Map::compute_segment)
            .sum::<i32>();

        let area = segment_sum.abs() / 2;

        area - (nb_points as i32 / 2) + 1
    }
}

impl From<char> for Pipe {
    fn from(pipe: char) -> Self {
        match pipe {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            other => panic!("Bizarre ce caractère: {}", other)
        }
    }
}

impl Pipe {
    fn connect_north(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::NorthEast => true,
            Pipe::NorthWest => true,
            _ => false
        }
    }

    fn connect_south(&self) -> bool {
        match self {
            Pipe::Vertical => true,
            Pipe::SouthEast => true,
            Pipe::SouthWest => true,
            _ => false
        }
    }

    fn connect_east(&self) -> bool {
        match self {
            Pipe::Horizontal => true,
            Pipe::NorthEast => true,
            Pipe::SouthEast => true,
            _ => false
        }
    }

    fn connect_west(&self) -> bool {
        match self {
            Pipe::Horizontal => true,
            Pipe::NorthWest => true,
            Pipe::SouthWest => true,
            _ => false
        }
    }

    fn get_dirs(&self) -> (Dir, Dir) {
        match self {
            Pipe::Horizontal => (Dir::East, Dir::West),
            Pipe::Vertical => (Dir::North, Dir::South),
            Pipe::NorthEast => (Dir::North, Dir::West),
            Pipe::NorthWest => (Dir::North, Dir::West),
            Pipe::SouthEast => (Dir::South, Dir::East),
            Pipe::SouthWest => (Dir::South, Dir::West),
            other => panic!("Should not happen: {:?}", other)
        }
    }

    fn get_next_dir(&self, cur_dir: Dir) -> Dir {
        match cur_dir {
            Dir::North => match self {
                Pipe::Vertical => Dir::South,
                Pipe::NorthEast => Dir::East,
                Pipe::NorthWest => Dir::West,
                other => panic!("Should not happen : {:?}, {:?}", other, cur_dir)
            },
            Dir::South => match self {
                Pipe::Vertical => Dir::North,
                Pipe::SouthEast => Dir::East,
                Pipe::SouthWest => Dir::West,
                other => panic!("Should not happen : {:?}, {:?}", other, cur_dir)
            },
            Dir::East => match self {
                Pipe::Horizontal => Dir::West,
                Pipe::NorthEast => Dir::North,
                Pipe::SouthEast => Dir::South,
                other => panic!("Should not happen : {:?}, {:?}", other, cur_dir)
            },
            Dir::West => match self {
                Pipe::Horizontal => Dir::East,
                Pipe::SouthWest => Dir::South,
                Pipe::NorthWest => Dir::North,
                other => panic!("Should not happen : {:?}, {:?}", other, cur_dir)
            }
        }
    }

    fn shoelace_filter(&self) -> bool {
        match self {
            Pipe::Horizontal => false,
            Pipe::Vertical => false,
            Pipe::Ground => false,
            Pipe::Start => false,
            _ => true
        }
    }
}

impl Dir {
    fn get_opp_dir(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East
        }
    }
}

fn part1(map: &mut Map) {
    map.walk_from_start();

    let result = map.all_nodes.len() / 2;

    println!("Part 1: {}", result);
}

fn part2(map: &mut Map) {
    let result = map.compute_shoelace_formula();

    println!("Part 2: {}", result);
}