use std::fs::read_to_string;

pub fn main() {
    println!("It's day 1 !!!");
    let file = read_to_string("src/day1_input.txt")
        .unwrap();

    part1(&file);
    part2(&file);
}

fn part1(file: &String) {
    let result = file.lines()
        .map(read_line_1)
        .sum::<u32>();

    println!("Part 1: {}", result);
}

fn read_line_1(line: &str) -> u32 {
    let first = line.chars()
        .map(|char| char.to_digit(10))
        .find(Option::is_some)
        .unwrap()
        .unwrap();
    let second = line.chars()
        .rev()
        .map(|char| char.to_digit(10))
        .find(Option::is_some)
        .unwrap()
        .unwrap();
    first * 10 + second
}

fn part2(file: &String) {
    let result = file.lines()
        .map(|line| read_line_2(line))
        .sum::<u32>();

    println!("Part 2: {}", result);
}

fn find_number(window: &[char], is_back: bool) -> Option<u32> {
    if window.len() < 3 {
        return if !is_back { window.iter().find_map(|char| char.to_digit(10)) }
            else { window.iter().rev().find_map(|char| char.to_digit(10)) };
    }

    let last_idx = window.len() - 1;
    let idx = if !is_back { 0 } else { last_idx };
    if window[idx].is_numeric() {
        return window[idx].to_digit(10);
    }

    let string_window = if !is_back { String::from_iter(window[..3].iter()) }
        else { String::from_iter(window[(window.len() - 3)..].iter()) };
    if string_window.contains("one") {
        return Some(1);
    } else if string_window.contains("two") {
        return Some(2);
    } else if string_window.contains("six") {
        return Some(6);
    }
    
    if window.len() < 4 {
        return if !is_back { window.iter().find_map(|char| char.to_digit(10)) }
            else { window.iter().rev().find_map(|char| char.to_digit(10)) };
    }

    let string_window = if !is_back { String::from_iter(window[..4].iter()) }
        else { String::from_iter(window[(window.len() - 4)..].iter()) };
    if string_window.contains("four") {
        return Some(4);
    } else if string_window.contains("five") {
        return Some(5);
    } else if string_window.contains("nine") {
        return Some(9);
    }

    if window.len() < 5 {
        return if !is_back { find_number(&window[1..], is_back) }
            else { find_number(&window[..last_idx], is_back)};
    }
    
    let string_window = String::from_iter(window.iter());
    if string_window.contains("three") {
        return Some(3);
    } else if string_window.contains("seven") {
        return Some(7);
    } else if string_window.contains("eight") {
        return Some(8);
    }

    if !is_back { find_number(&window[1..], is_back) }
        else { find_number(&window[..last_idx], is_back)}
}

fn read_line_2(line: &str) -> u32 {
    let max_window = line.len().min(5);
    let first = line.chars()
        .collect::<Vec<char>>()
        .windows(max_window)
        .find_map(|window| find_number(window, false))
        .unwrap();

    let second = line.chars()
        .collect::<Vec<char>>()
        .windows(max_window)
        .rev()
        .find_map(|window| find_number(window, true))
        .unwrap();

    first * 10 + second
}