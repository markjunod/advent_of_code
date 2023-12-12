use std::{collections::HashMap, cmp::Ordering};

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref CARD_HANDS: Vec<CardHand> = include_str!("inputs/day7.txt").trim().split('\n').map(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        CardHand {
            hand: parts[0],
            hand_type: HandType::from(parts[0]),
            bid: parts[1].parse::<u64>().unwrap(),
            wild: None,
        }
    }).collect();
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let card_counts = count_chars(value);
        match card_counts.len() {
            1 => HandType::Five,
            2 if card_counts.values().min().unwrap() == &1 => HandType::Four,
            2 => HandType::FullHouse,
            3 if card_counts.values().max().unwrap() == &3 => HandType::Three,
            3 => HandType::TwoPair,
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => panic!("More than 5 cards in hand {}", value),
        }
    }
}

fn count_chars(str: &str) -> HashMap<char, u8> {
    let mut counts = HashMap::new();
    for c in str.chars() {
        counts.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
    }
    counts
}

#[derive(Eq, Clone, Debug)]
struct CardHand {
    hand: &'static str,
    hand_type: HandType,
    bid: u64,
    wild: Option<char>,
}

impl CardHand {
    fn make_card_wild(&mut self, wild: char) {
        self.wild = Some(wild);
        let num_wilds = self.hand.chars().fold(0, |acc, c| if c == wild { acc + 1 } else { acc });
        match &self.hand_type {
            HandType::Four if num_wilds > 0 => self.hand_type = HandType::Five,
            HandType::FullHouse if num_wilds > 0 => self.hand_type = HandType::Five,
            HandType::Three if num_wilds > 0 => self.hand_type = HandType::Four,
            HandType::TwoPair if num_wilds == 2 => self.hand_type = HandType::Four,
            HandType::TwoPair if num_wilds == 1 => self.hand_type = HandType::FullHouse,
            HandType::Pair if num_wilds > 0 => self.hand_type = HandType::Three,
            HandType::HighCard if num_wilds > 0 => self.hand_type = HandType::Pair,
            _ => (),
        };
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            let mut first_not_equal = self.hand.chars().zip(other.hand.chars()).skip_while(|(sc, oc)| sc == oc);
            match first_not_equal.next() {
                None => Ordering::Equal,
                Some((sc, oc)) if self.wild.is_some() => compare_cards_with_wild(sc, oc, self.wild.unwrap()),
                Some((sc, oc)) => compare_cards(sc, oc),
            }
        }
    }
}

fn compare_cards_with_wild(c1: char, c2: char, wild: char) -> Ordering {
    match (c1, c2) {
        (_, _) if c1 == c2 => Ordering::Equal,
        (_, _) if c1 == wild => Ordering::Less,
        (_, _) if c2 == wild => Ordering::Greater,
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('J', _) => Ordering::Greater,
        (_, 'J') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        (c1, c2) => c1.cmp(&c2),
    }
}

fn compare_cards(c1: char, c2: char) -> Ordering {
    match (c1, c2) {
        (_, _) if c1 == c2 => Ordering::Equal,
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('J', _) => Ordering::Greater,
        (_, 'J') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        (c1, c2) => c1.cmp(&c2),
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 7 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 7 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let sum = score_card_hands(&mut CARD_HANDS.to_vec());

    info!("2023 - Day 7 - Part 1: Total winnings {}", sum);
}

fn run_part2() {
    let mut card_hands = CARD_HANDS.to_vec();
    card_hands.iter_mut().for_each(|card_hand| card_hand.make_card_wild('J'));
    let sum = score_card_hands(&mut card_hands);

    info!("2023 - Day 7 - Part 2: Total winnings with wild {}", sum);
}

fn score_card_hands(card_hands: &mut Vec<CardHand>) -> u64 {
    card_hands.sort();
    card_hands.into_iter().enumerate().map(|(idx, hand)| {
        (idx as u64 + 1) * hand.bid
    }).sum()
}
