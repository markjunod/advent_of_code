use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref TREE_MAP: Vec<&'static str> = include_str!("inputs/day3.txt").trim().split("\n").map(|line| line.trim()).collect();
}

const TREE: &str = "#";

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2020 - Day 3 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2020 - Day 3 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    println!("2020 - Day 3 - Part 1: {} trees hit", traverse_map(3, 1));
}

fn run_part2() {
    let r1_d1 = traverse_map(1, 1);
    let r3_d1 = traverse_map(3, 1);
    let r5_d1 = traverse_map(5, 1);
    let r7_d1 = traverse_map(7, 1);
    let r1_d2 = traverse_map(1, 2);

    let product = r1_d1 * r3_d1 * r5_d1 * r7_d1 * r1_d2;

    println!("2020 - Day 3 - Part 2: {}, {}, {}, {}, and {} trees hit. The product is {}", r1_d1, r3_d1, r5_d1, r7_d1, r1_d2, product);
}

fn traverse_map(right: usize, down: usize) -> u32 {
    let mut num_trees = 0;

    let mut current_idx = 0;
    for i in (0..TREE_MAP.len()).step_by(down) {
        let tree_line = TREE_MAP[i];
        current_idx %= tree_line.len();

        if tree_line[current_idx..(current_idx + 1)] == *TREE {
            num_trees += 1;
        }

        current_idx += right;
    }

    num_trees
}
