use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref PASSWORD_POLICIES: Vec<&'static str> = include_str!("inputs/day2.txt")
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2020 - Day 2 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2020 - Day 2 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut valid_passwords = 0;

    for i in 0..PASSWORD_POLICIES.len() {
        let (min, max, c, password) = parse_password_policy(PASSWORD_POLICIES[i]);

        let c_count = password.chars().fold(0u8, |sum, pc| {
            if pc == c {
                sum + 1
            } else {
                sum
            }
        });

        if min <= c_count && c_count <= max {
            valid_passwords += 1;
        }
    }

    info!("2020 - Day 2 - Part 1: Found {} valid passwords", valid_passwords);
}

fn run_part2() {
    let mut valid_passwords = 0;

    for i in 0..PASSWORD_POLICIES.len() {
        let (min, max, c, password) = parse_password_policy(PASSWORD_POLICIES[i]);

        let password_slice = password.get((min as usize - 1)..(max as usize)).unwrap();

        if password_slice.starts_with(c) || password_slice.ends_with(c) {
            if !(password_slice.starts_with(c) && password_slice.ends_with(c)) {
                valid_passwords += 1;
            }
        }
    }

    info!("2020 - Day 2 - Part 2: Found {} valid passwords", valid_passwords);
}

fn parse_password_policy(policy: &str) -> (u8, u8, char, &str) {
    let policy_and_password: Vec<&str> = policy.split(": ").collect();
    let range_and_char: Vec<&str> = policy_and_password[0].split(' ').collect();
    let range: Vec<&str> = range_and_char[0].split('-').collect();
    let min: u8 = range[0].parse().unwrap();
    let max: u8 = range[1].parse().unwrap();
    let c = range_and_char[1].chars().next().unwrap();
    let password = policy_and_password[1];

    (min, max, c, password)
}
