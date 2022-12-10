use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref INDIVIDUAL_CALORIES: Vec<Vec<u32>> = include_str!("inputs/day1.txt")
        .trim()
        .split("\n\n")
        .map(|lines| {
            lines.trim().split("\n").map(|line| line.trim().parse::<u32>().unwrap()).collect()
        })
        .collect();

    static ref CALORIES: Vec<u32> = INDIVIDUAL_CALORIES.iter().map(|single_elf_calories| single_elf_calories.iter().sum()).collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2022 - Day 1 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2022 - Day 1 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let max_calories = CALORIES.iter().max();

    match max_calories {
        Some(max) => println!("2022 - Day 1 - Part 1: {} total calories carried by the top elf", max),
        None => println!("2022 - Day 1 - Part 1: Unable to parse a max value from the calorie sums"),
    };
}

fn run_part2() {
    let mut top_3_calories: [u32; 3] = [0, 0, 0];

    CALORIES.iter().for_each(|calories| {
        if *calories > top_3_calories[0] {
            top_3_calories[0] = *calories;
        }
        if *calories > top_3_calories[1] {
            top_3_calories[0] = top_3_calories[1];
            top_3_calories[1] = *calories;
        }
        if *calories > top_3_calories[2] {
            top_3_calories[1] = top_3_calories[2];
            top_3_calories[2] = *calories;
        }
    });

    let sum = top_3_calories[0] + top_3_calories[1] + top_3_calories[2];

    println!("2022 - Day 1 - Part 2: {} total calories carried by the top 3 elves", sum);
}
