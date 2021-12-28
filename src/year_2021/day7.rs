use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref INIT_CRAB_POSITIONS: Vec<u64> = include_str!("inputs/day7.txt")
        .trim()
        .split(",")
        .map(|timer_str| timer_str.parse::<u64>().unwrap())
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2021 - Day 7 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2021 - Day 7 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut crab_positions = INIT_CRAB_POSITIONS.clone();
    crab_positions.sort_unstable();

    // find the median position and then calculate the distance each crab has to move
    let median_pos = calc_median(&crab_positions);
    let total_moved = crab_positions.iter().fold(0, |moved, &crab_pos| {
        if crab_pos < median_pos {
            moved + median_pos - crab_pos
        } else {
            moved + crab_pos - median_pos
        }
    });
    println!("2021 - Day 7 - Part 1: {} total fuel used to move crabs", total_moved);
}

fn run_part2() {
    let mut crab_positions = INIT_CRAB_POSITIONS.clone();
    crab_positions.sort_unstable();

    let mut total_fuel = 0;
    let mut lower_idx = 0;
    let mut upper_idx = crab_positions.len() - 1;
    let mut cost_of_lower_move = 1;
    let mut cost_of_upper_move = 1;

    while crab_positions[lower_idx] == crab_positions[lower_idx + 1] {
        lower_idx += 1;
        cost_of_lower_move += 1;
    }

    while crab_positions[upper_idx] == crab_positions[upper_idx - 1] {
        upper_idx -= 1;
        cost_of_upper_move += 1;
    }

    let mut amount_moved: Vec<u64> = vec![0; crab_positions.len()];

    while crab_positions[lower_idx] + amount_moved[lower_idx] != crab_positions[upper_idx] - amount_moved[upper_idx] {
        if cost_of_lower_move < cost_of_upper_move {
            total_fuel += cost_of_lower_move;
            cost_of_lower_move = 0;
            for i in 0..(lower_idx + 1) {
                amount_moved[i] += 1;
                cost_of_lower_move += amount_moved[i] + 1;
            }

            while crab_positions[lower_idx] + amount_moved[lower_idx] == crab_positions[lower_idx + 1] {
                lower_idx += 1;
                cost_of_lower_move += 1;
            }
        } else {
            total_fuel += cost_of_upper_move;
            cost_of_upper_move = 0;
            for i in upper_idx..crab_positions.len() {
                amount_moved[i] += 1;
                cost_of_upper_move += amount_moved[i] + 1;
            }

            while crab_positions[upper_idx] - amount_moved[upper_idx] == crab_positions[upper_idx - 1] {
                upper_idx -= 1;
                cost_of_upper_move += 1;
            }
        }
    }

    println!("2021 - Day 7 - Part 2: {} total fuel used to move crabs", total_fuel);
}

fn calc_median(positions: &Vec<u64>) -> u64 {
    if positions.len() % 2 == 1 {
        return positions[(positions.len() - 1) / 2];
    }

    let high_middle = positions.len() / 2;
    (positions[high_middle] + positions[high_middle - 1]) / 2
}

fn calc_geo_sum(n: u64) -> u64 {
    // this is the closed form of 1 + 2 + 3 + ... + n
    n * (n + 1) / 2
}
