use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref DEPTHS: Vec<u32> = include_str!("inputs/day1.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2021 - Day 1 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2021 - Day 1 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut increased_count = 0;
    for i in 1..DEPTHS.len() {
        if DEPTHS[i] > DEPTHS[i - 1] {
            increased_count += 1;
        }
    }

    println!("2021 - Day 1 - Part 1: {} increasing depths", increased_count);
}

fn run_part2() {
    let mut increased_count = 0;
    let window_depths: Vec<u32> = DEPTHS.as_slice().windows(3).map(|window| window.iter().sum()).collect();
    for i in 1..window_depths.len() {
        if window_depths[i] > window_depths[i - 1] {
            increased_count += 1;
        }
    }

    println!("2021 - Day 1 - Part 2: {} increasing windows", increased_count);
}
