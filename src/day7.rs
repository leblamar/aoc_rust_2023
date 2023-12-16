use std::{fs::read_to_string, cmp::Ordering};

pub fn main() {
    println!("It's day 7 !!!");
    let hands = &mut Hand::create_hands();

    part1(hands);
    part2(hands);
}

#[derive(Debug)]
enum Combi {
    Five,
    Four,
    Full,
    Three,
    TwoTwo,
    Two,
    High
}

impl Combi {
    fn create_combi(cards: &Vec<u32>) -> Combi {
        let mut values = vec![0; 15];
        cards.iter()
            .for_each(|&card| values[card as usize] += 1);

        let mut nb_2 = 0;
        let mut as_3 = false;
        for i in values {
            if i == 5 {
                return Combi::Five;
            } else if i == 4 {
                return Combi::Four;
            } else if i == 3 {
                as_3 = true;
            } else if i == 2 {
                nb_2 += 1;
            }
        }

        if as_3 {
            if nb_2 == 1 {
                return Combi::Full;
            }
            return Combi::Three;
        } else if nb_2 == 2 {
            return Combi::TwoTwo;
        } else if nb_2 == 1 {
            return Combi::Two;
        }
        return Combi::High;
    }

    fn create_combi_2(cards: &Vec<u32>) -> Combi {
        let mut values = vec![0; 15];
        cards.iter()
            .for_each(|&card| values[card as usize] += 1);
        let joker = values[1];

        let mut nb_2 = 0;
        let mut as_3 = false;
        for i in values.iter() {
            if *i == 5 {
                return Combi::Five;
            } else if *i == 4 {
                if joker != 0 {
                    return Combi::Five;
                }
                return Combi::Four;
            } else if *i == 3 {
                as_3 = true;
            } else if *i == 2 {
                nb_2 += 1;
            }
        }

        if as_3 {
            if nb_2 == 1 {
                if joker != 0 {
                    return Combi::Five;
                }
                return Combi::Full;
            }
            if joker != 0 {
                return Combi::Four;
            }
            return Combi::Three;
        } else if nb_2 == 2 {
            if joker == 2 {
                return Combi::Four;
            } else if joker == 1 {
                return Combi::Full;
            }
            return Combi::TwoTwo;
        } else if nb_2 == 1 {
            if joker != 0 {
                return Combi::Three;
            }
            return Combi::Two;
        } else if joker != 0 {
            return Combi::Two;
        }
        return Combi::High;
    }

    fn points(&self) -> u32 {
        match self {
           Combi::Five => 6,
           Combi::Four => 5,
           Combi::Full => 4,
           Combi::Three => 3,
           Combi::TwoTwo => 2,
           Combi::Two => 1,
           Combi::High => 0
        }
    }

    fn cmp(&self, other: &Combi) -> Ordering {
        self.points().cmp(&other.points())
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    combi: Combi,
    bet: u32
}

impl Hand {
    fn create_hands() -> Vec<Hand> {
        read_to_string("src/day7_input.txt")
            .unwrap()
            .lines()
            .filter_map(Hand::parse_hand)
            .collect::<Vec<Hand>>()
    }

    fn parse_hand(line: &str) -> Option<Hand> {
        let (cards_str, bet) = line.split_once(" ")?;
        
        let cards = Hand::parse_cards(cards_str);
        let combi = Combi::create_combi(&cards);

        Some(Hand {
            cards,
            combi,
            bet: bet.parse().ok()?
        })
    }

    fn parse_cards(cards: &str) -> Vec<u32> {
        cards.chars()
            .map(Hand::parse_card)
            .collect::<Vec<u32>>()
    }

    fn parse_card(card: char) -> u32 {
        if let Some(value) = card.to_digit(10) {
            return value;
        }

        match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("This should not be possible: {}", card)
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let comb_compare = self.combi.cmp(&other.combi);

        if let Ordering::Equal = comb_compare {
            return self.cards.cmp(&other.cards);
        }

        comb_compare
    }
    
    fn update_for_part2(&mut self) {
        self.cards
            .iter_mut()
            .filter(|val| **val == 11)
            .for_each(|val| *val = 1);

        self.combi = Combi::create_combi_2(&self.cards);
    }
}

fn part1(hands: &mut Vec<Hand>) {
    hands.sort_by(|hand1, hand2| hand1.cmp(hand2));

    let result: u32 = hands.iter()
        .enumerate()
        .map(|(idx, hand)| hand.bet * (idx + 1) as u32)
        .sum();

    println!("Part 1: {}", result);
}

fn part2(hands: &mut Vec<Hand>) {
    hands.iter_mut()
        .for_each(|hand| hand.update_for_part2());

    hands.sort_by(|hand1, hand2| hand1.cmp(hand2));

    let result: u32 = hands.iter()
        .enumerate()
        .map(|(idx, hand)| hand.bet * (idx + 1) as u32)
        .sum();

    println!("Part 2: {}", result);
}