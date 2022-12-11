use lazy_static::lazy_static;
use log::{error, info};

use crate::timer::time_millis;

const ROCK_SCORE: u64 = 1;
const PAPER_SCORE: u64 = 2;
const SCISSORS_SCORE: u64 = 3;

const WIN_SCORE: u64 = 6;
const DRAW_SCORE: u64 = 3;
const LOSS_SCORE: u64 = 0;


lazy_static! {
    static ref STRATEGY_PAIRS: Vec<(char, char)> = include_str!("inputs/day2.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| {
            let tuple_vec: Vec<char> = line.chars().collect();
            (tuple_vec[0], tuple_vec[2])
        })
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2022 - Day 2 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2022 - Day 2 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let total_score: u64 = STRATEGY_PAIRS
        .iter()
        .map(|pair| {
            match pair {                                        // opp  vs me
                ('A', 'X') => ROCK_SCORE + DRAW_SCORE,          // rock vs rock
                ('A', 'Y') => PAPER_SCORE + WIN_SCORE,          // rock vs paper
                ('A', 'Z') => SCISSORS_SCORE + LOSS_SCORE,      // rock vs scissors
                ('B', 'X') => ROCK_SCORE + LOSS_SCORE,          // paper vs rock
                ('B', 'Y') => PAPER_SCORE + DRAW_SCORE,         // paper vs paper
                ('B', 'Z') => SCISSORS_SCORE + WIN_SCORE,       // paper vs scissors
                ('C', 'X') => ROCK_SCORE + WIN_SCORE,           // scissors vs rock
                ('C', 'Y') => PAPER_SCORE + LOSS_SCORE,         // scissors vs paper
                ('C', 'Z') => SCISSORS_SCORE + DRAW_SCORE,      // scissors vs scissors
                _ => panic!("{:?} is an unknown pair", pair),
            }
        })
        .sum();

    info!("2022 - Day 2 - Part 1: {} total score for strategy guide", total_score);
}

fn run_part2() {
    let total_score: u64 = STRATEGY_PAIRS
        .iter()
        .map(|pair| {
            match pair {
                ('A', 'X') => SCISSORS_SCORE + LOSS_SCORE,    // lose against rock
                ('A', 'Y') => ROCK_SCORE + DRAW_SCORE,        // draw against rock
                ('A', 'Z') => PAPER_SCORE + WIN_SCORE,        // win against rock
                ('B', 'X') => ROCK_SCORE + LOSS_SCORE,        // lose against paper
                ('B', 'Y') => PAPER_SCORE + DRAW_SCORE,       // draw against paper
                ('B', 'Z') => SCISSORS_SCORE + WIN_SCORE,     // win against paper
                ('C', 'X') => PAPER_SCORE + LOSS_SCORE,       // lose against scissors
                ('C', 'Y') => SCISSORS_SCORE + DRAW_SCORE,    // draw against scissors
                ('C', 'Z') => ROCK_SCORE + WIN_SCORE,         // win against scissors
                _ => panic!("{:?} is an unknown pair", pair),
            }
        })
        .sum();

    info!("2022 - Day 2 - Part 2: {} total score for strategy guide", total_score);
}
