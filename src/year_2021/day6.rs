use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref INIT_FISH_COUNTS: Vec<u64> = include_str!("inputs/day6.txt")
        .trim()
        .split(",")
        .map(|timer_str| timer_str.parse::<usize>().unwrap())
        .fold(vec![0; 9], |mut fish_counts, timer| {
            fish_counts[timer] += 1;
            fish_counts
        });
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2021 - Day 6 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2021 - Day 6 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut fish_counts = INIT_FISH_COUNTS.clone();

    populate_lantern_fish(80, &mut fish_counts);

    info!("2021 - Day 6 - Part 1: {} lantern fish after 80 days", fish_counts.iter().sum::<u64>());
}

fn run_part2() {
    let mut fish_counts = INIT_FISH_COUNTS.clone();

    populate_lantern_fish(256, &mut fish_counts);

    info!("2021 - Day 6 - Part 2: {} lantern fish after 256 days", fish_counts.iter().sum::<u64>());
}

fn populate_lantern_fish(days: u32, fish_counts: &mut Vec<u64>) {
    for _ in 0..days {
        let new_fish = fish_counts[0];
        for i in 0..8 {
            fish_counts[i] = fish_counts[i + 1];
        }
        fish_counts[6] += new_fish;
        fish_counts[8] = new_fish;
    }
}
