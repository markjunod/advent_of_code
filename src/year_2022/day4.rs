use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.min <= other.min && other.min <= self.max)
            || (self.min <= other.max && other.max <= self.max)
            || (other.min <= self.min && self.min <= other.max)
            || (other.min <= self.max && self.max <= other.max)
    }
}

lazy_static! {
    static ref ASSIGNMENT_PAIRS: Vec<(Range, Range)> = include_str!("inputs/day4.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| {
            let pair: Vec<&str> = line.split(',').collect();
            let first: Vec<&str> = pair[0].split('-').collect();
            let second: Vec<&str> = pair[1].split('-').collect();

            (
                Range {min: first[0].parse().unwrap(), max: first[1].parse().unwrap()},
                Range {min: second[0].parse().unwrap(), max: second[1].parse().unwrap()},
            )
        })
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2022 - Day 4 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2022 - Day 4 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut contained_pairs = 0;

    ASSIGNMENT_PAIRS.iter().for_each(|(section1, section2)| {
        if section1.contains(section2) || section2.contains(section1) {
            contained_pairs += 1;
        }
    });

    info!("2022 - Day 4 - Part 1: {} number of fully contained assignments", contained_pairs);
}

fn run_part2() {
    let mut overlapping_pairs = 0;

    ASSIGNMENT_PAIRS.iter().for_each(|(section1, section2)| {
        if section1.overlaps(section2) {
            overlapping_pairs += 1;
        }
    });

    info!("2022 - Day 4 - Part 1: {} number of overlapping assignments", overlapping_pairs);
}
