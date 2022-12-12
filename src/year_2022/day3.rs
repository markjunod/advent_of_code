use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref RUCKSACKS: Vec<(HashSet<char>, HashSet<char>)> = include_str!("inputs/day3.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .map(|line| {
            let midway = line.len() / 2;
            let part1: HashSet<char> = line[..midway].chars().collect();
            let part2: HashSet<char> = line[midway..].chars().collect();
            (part1, part2)
        })
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2022 - Day 3 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2022 - Day 3 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut priority_sum = 0;

    RUCKSACKS.iter().for_each(|(compartment1, compartment2)| {
        let common: Vec<&char> = compartment1.intersection(compartment2).collect();
        if common.len() != 1 {
            panic!("There's more than one item in both compartments: {:?}", common);
        }

        let common_item = common[0];
        priority_sum += calc_item_score(common_item);
    });

    info!("2022 - Day 3 - Part 1: {} is the sum of the common item priorities", priority_sum);
}

fn run_part2() {
    let mut priority_sum = 0;

    for i in (0..RUCKSACKS.len()).step_by(3) {
        let pack1: HashSet<&char> = RUCKSACKS[i].0.union(&RUCKSACKS[i].1).collect();
        let pack2: HashSet<&char> = RUCKSACKS[i + 1].0.union(&RUCKSACKS[i + 1].1).collect();
        let pack3: HashSet<&char> = RUCKSACKS[i + 2].0.union(&RUCKSACKS[i + 2].1).collect();

        let common_pack1_pack2: HashSet<&char> = pack1.intersection(&pack2).map(|c| *c).collect();
        let common: Vec<&char> = common_pack1_pack2.intersection(&pack3).map(|c| *c).collect();
        if common.len() != 1 {
            panic!("There's more than one item in both compartments: {:?}", common);
        }

        let common_item = common[0];
        priority_sum += calc_item_score(common_item);
    }

    info!("2022 - Day 3 - Part 2: {} is the sum of the badge priorities", priority_sum);
}

// priorities are scored as follows:
// - Lowercase item types a through z have priorities 1 through 26.
// - Uppercase item types A through Z have priorities 27 through 52.
// but ASCII order is A-Z first, then a-z
fn calc_item_score(c: &char) -> u32 {
    if *c >= 'a' {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}
