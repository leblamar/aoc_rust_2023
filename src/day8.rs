use std::fs::read_to_string;
use std::collections::HashMap;

pub fn main() {
    println!("It's day 8 !!!");
    let network = Network::create_network().unwrap();

    part1(&network);
    part2(&network);
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
    map: HashMap<String, usize>
}

#[derive(Debug)]
enum Instruction {
    Right,
    Left
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
    is_final: bool
}

impl Network {
    fn create_network() -> Option<Network> {
        let file = read_to_string("src/day8_input.txt")
            .unwrap();
        let lines: Vec<&str> = file.lines()
            .collect();

        let instructions = Instruction::parse_instructions(lines[0]);
        let nodes = Node::parse_nodes(&lines[2..]);
        let map = Network::create_map(&nodes);

        Some(Network { 
            instructions,
            nodes,
            map
        })
    }

    fn get_aaa_idx(&self) -> usize {
        self.nodes
            .iter()
            .position(|node| node.id == "AAA")
            .unwrap()
    }

    fn get_zzz_idx(&self) -> usize {
        self.nodes
            .iter()
            .position(|node| node.id == "ZZZ")
            .unwrap()
    }

    fn get_a_idxs(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|&node| node.id.ends_with('A'))
            .map(|node| *self.map.get(&node.id).unwrap())
            .collect::<Vec<usize>>()
    }

    fn create_map(nodes: &Vec<Node>) -> HashMap<String, usize> {
        nodes.iter()
            .enumerate()
            .map(|(idx, node)| (node.id.clone(), idx))
            .collect::<HashMap<String, usize>>()
    }

    fn next_idx(&self, cur_instruction: usize, cur_idx: usize) -> usize {
        let node = &self.nodes[cur_idx];
        let instruction = &self.instructions[cur_instruction];
        let next_id = &instruction.get_next(node);
        *self.map.get(next_id).unwrap()
    }

    fn found_cycle_and_offset(&self, init_idx: usize) -> (u32, u32) {
        let nb_instruction = self.instructions.len();
        let mut cur_instruction = 0;
        let mut cur_idx = init_idx;
        let mut first_cycle = 0;
        let mut step = 0;
        while step < 1_000_000 {
            step += 1;
            cur_idx = self.next_idx(cur_instruction, cur_idx);

            if self.nodes[cur_idx].is_final {
                if first_cycle != 0 {
                    break;
                }
                first_cycle = step;
            }

            cur_instruction = (cur_instruction + 1) % nb_instruction;
        }
        let cycle = step - first_cycle;
        let offset = first_cycle - cycle;

        (cycle, offset)
    }
}

impl Instruction {
    fn parse_instructions(line: &str) -> Vec<Instruction> {
        line.chars()
            .map(Instruction::parse_instruction)
            .collect::<Vec<Instruction>>()
    }

    fn parse_instruction(inst: char) -> Instruction {
        match inst {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            other => panic!("This character is unknown as instruction: {}", other)
        }
    }

    fn get_next(&self, node: &Node) -> String {
        match self {
            Instruction::Right => node.right.clone(),
            Instruction::Left => node.left.clone()
        }
    }
}

impl Node {
    fn parse_nodes(nodes: &[&str]) -> Vec<Node> {
        nodes.iter()
            .filter_map(Node::parse_node)
            .collect::<Vec<Node>>()
    }

    fn parse_node(&node: &&str) -> Option<Node> {
        let (id, equiv) = node.split_once(" = (")?;

        let (left, right) = equiv[..(equiv.len() - 1)].split_once(", ")?;

        let is_final = id.ends_with('Z');

        Some(Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            is_final
        })
    }
}

fn part1(network: &Network) {
    let nb_instruction = network.instructions.len();
    let mut result = 0;
    let mut cur_idx = network.get_aaa_idx();
    let zzz_idx = network.get_zzz_idx();
    let mut cur_instruction = 0;
    while cur_idx != zzz_idx {
        cur_idx = network.next_idx(cur_instruction, cur_idx);
        result += 1;
        cur_instruction = (cur_instruction + 1) % nb_instruction;
    }

    println!("Part 1: {}", result);
}

fn part2(network: &Network) {
    let cur_idxs = network.get_a_idxs();

    let cycles_offsets = cur_idxs.iter()
        .map(|&cur_idx| network.found_cycle_and_offset(cur_idx))
        .collect::<Vec<(u32, u32)>>();

    if !cycles_offsets.iter().all(|&(_, offset)| offset == 0) {
        panic!("In the original solution all cycles were the same equals to the initial cycle, that was just luck, but not this time :'(")
    }

    let result = cycles_offsets.iter()
        .map(|&(cycle, _)| cycle as u64)
        .reduce(|acc, cycle| lcm(acc, cycle))
        .unwrap();

    println!("Part 2: {}", result);
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while a % b > 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }

    b
}