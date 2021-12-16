use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref EXPENSES: Vec<u32> = include_str!("inputs/day1.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2020 - Day 1 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2020 - Day 1 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    for i in 0..(EXPENSES.len() - 1) {
        for j in (i + 1)..EXPENSES.len() {
            let e1 = EXPENSES[i];
            let e2 = EXPENSES[j];

            if e1 + e2 == 2020 {
                println!("2020 - Day 1 - Part 1: {} + {} = 2020", e1, e2);
                println!("2020 - Day 1 - Part 1: {} * {} = {}", e1, e2, e1 * e2);
                break;
            }
        }
    }
}

fn run_part2() {
    for i in 0..(EXPENSES.len() - 2) {
        for j in (i + 1)..(EXPENSES.len() - 1) {
            for k in (j + 1)..EXPENSES.len() {
                let e1 = EXPENSES[i];
                let e2 = EXPENSES[j];
                let e3 = EXPENSES[k];

                if e1 + e2 + e3 == 2020 {
                    println!("2020 - Day 1 - Part 2: {} + {} + {} = 2020", e1, e2, e3);
                    println!("2020 - Day 1 - Part 2: {} * {} * {} = {}", e1, e2, e3, e1 * e2 * e3);
                    break;
                }
            }
        }
    }
}
