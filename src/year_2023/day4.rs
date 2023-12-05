use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref CARDS: Vec<Card> = include_str!("inputs/day4.txt")
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .map(Card::from)
        .collect();
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers_you_have: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let parts = value.split(':').collect::<Vec<&str>>();
        let numbers = parts[1].split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers[0].trim().split_ascii_whitespace().map(|wn| wn.parse::<u32>().unwrap()).collect();
        let numbers_you_have = numbers[1].trim().split_ascii_whitespace().map(|wn| wn.parse::<u32>().unwrap()).collect();
        
        Card { winning_numbers, numbers_you_have }
    }
}

impl Card {
    fn count_winning_numbers_you_have(&self) -> u32 {
        self.numbers_you_have.iter().fold(0, |accum, n| {
            if self.winning_numbers.contains(n) {
                accum + 1
            } else {
                accum
            }
        })
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 4 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 4 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let scores = CARDS.iter().map(|card| {
        let count = card.count_winning_numbers_you_have();
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    });

    let sum: u32 = scores.sum();

    info!("2023 - Day 4 - Part 1: Sum of card scores {}", sum);
}

fn run_part2() {
    let mut counts = vec![1; CARDS.len()];
    CARDS.iter().enumerate().for_each(|(idx, card)| {
        for i in 1..=card.count_winning_numbers_you_have() as usize {
            counts[idx + i] += counts[idx];
        }
    });

    let card_count: u32 = counts.iter().sum();
    info!("2023 - Day 4 - Part 2: Total number of cards {}", card_count);
}
