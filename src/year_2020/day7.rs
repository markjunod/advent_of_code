use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref RULES: Vec<&'static str> = include_str!("inputs/day7.txt").trim().split("\n").collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2020 - Day 7 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2020 - Day 7 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    println!("2020 - Day 7 - Part 1: ");
}

fn run_part2() {
    println!("2020 - Day 7 - Part 2: ");
}