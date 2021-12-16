use lazy_static::lazy_static;
use std::collections::HashSet;

use crate::timer::time_millis;

lazy_static! {
    static ref ALL_ANSWERS: Vec<&'static str> = include_str!("inputs/day6.txt").trim().split(",").collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2020 - Day 6 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2020 - Day 6 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let sum_of_yesses_counts: u32 = ALL_ANSWERS.iter().map(|answers| count_yesses_in_group(answers)).sum();

    println!("2020 - Day 6 - Part 1: Sum of anyone answering yes is {}", sum_of_yesses_counts);
}

fn run_part2() {
    let sum_of_yesses_counts: u32 = ALL_ANSWERS.iter().map(|answers| count_overlapping_yesses(answers)).sum();

    println!("2020 - Day 6 - Part 2: Sum of everyone answering yes is {}", sum_of_yesses_counts);
}

fn count_yesses_in_group(group_answers: &str) -> u32 {
    let char_set: HashSet<char> = group_answers.chars().filter(|c| !c.is_whitespace()).collect();

    char_set.len() as u32
}

fn count_overlapping_yesses(group_answers: &str) -> u32 {
    let answer_sets: Vec<HashSet<char>> = group_answers.split(char::is_whitespace).filter(|coll| coll.len() > 0).map(|answer| {
        let char_set: HashSet<char> = answer.chars().collect();

        char_set
    }).collect();

    let initial_yesses = answer_sets[0].clone();
    let all_yesses = answer_sets.iter().fold(initial_yesses, |current_yesses, next_yesses| &current_yesses & next_yesses);

    all_yesses.len() as u32
}