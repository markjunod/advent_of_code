use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref COMMANDS: Vec<&'static str> = include_str!("inputs/day2.txt")
        .trim()
        .split("\n")
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2021 - Day 2 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2021 - Day 2 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut horizontal = 0;
    let mut depth = 0;

    COMMANDS.iter().for_each(|command| {
        let command_parts: Vec<&'static str> = command.trim().split(" ").collect();
        let units = command_parts[1].parse::<u32>().unwrap();
        match command_parts[0] {
            "forward" => horizontal += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => panic!("{} is not a valid command", command),
        }
    });

    info!("2021 - Day 2 - Part 1: {} forward, {} depth, yields {}", horizontal, depth, horizontal * depth);
}

fn run_part2() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    COMMANDS.iter().for_each(|command| {
        let command_parts: Vec<&'static str> = command.trim().split(" ").collect();
        let units = command_parts[1].parse::<u32>().unwrap();
        match command_parts[0] {
            "forward" => {
                horizontal += units;
                depth += aim * units;
            },
            "down" => aim += units,
            "up" => aim -= units,
            _ => panic!("{} is not a valid command", command),
        }
    });

    info!("2021 - Day 2 - Part 2: {} forward, {} depth, {} aim, yields {}", horizontal, depth, aim, horizontal * depth);
}
