use lazy_static::lazy_static;
use log::{error, info};

use crate::timer::time_millis;

lazy_static! {
    static ref CALIBRATION_LINES: Vec<String> = include_str!("inputs/day1.txt")
        .trim()
        .split("\n")
        .map(|lines| {
            lines.trim().to_string()
        })
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 1 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 1 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut sum = 0;
    CALIBRATION_LINES.iter().for_each(|line| {
        let calibration_value = 10 * find_first_digit(line) + find_last_digit(line);
        sum += calibration_value;
    });

    info!("2023 - Day 1 - Part 1: Sum of calibration values {}", sum);
}

fn find_first_digit(str: &str) -> u32 {
    for c in str.chars() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
    }
    error!("Unable to find first digit in {}", str);
    0
}

fn find_last_digit(str: &str) -> u32 {
    for c in str.chars().rev() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
    }
    error!("Unable to find last digit in {}", str);
    0
}

fn run_part2() {
    let mut sum = 0;
    CALIBRATION_LINES.iter().for_each(|line| {
        let calibration_value = 10 * find_first_digit_char_or_word(line) + find_last_digit_char_or_word(line);
        sum += calibration_value;
    });

    info!("2023 - Day 1 - Part 2: New sum of calibration values {}", sum);
}

fn find_first_digit_char_or_word(str: &str) -> u32 {
    let str_bytes = str.as_bytes();
    for (i, c) in str.char_indices() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }

        if str.len() - i >= 5 {
            match (str_bytes[i] as char, str_bytes[i + 1] as char, str_bytes[i + 2] as char, str_bytes[i + 3] as char, str_bytes[i + 4] as char) {
                ('t', 'h', 'r', 'e', 'e') => return 3,
                ('s', 'e', 'v', 'e', 'n') => return 7,
                ('e', 'i', 'g', 'h', 't') => return 8,
                _ => (),
            };
        }
        if str.len() - i >= 4 {
            match (str_bytes[i] as char, str_bytes[i + 1] as char, str_bytes[i + 2] as char, str_bytes[i + 3] as char) {
                ('z', 'e', 'r', 'o') => return 0,
                ('f', 'o', 'u', 'r') => return 4,
                ('f', 'i', 'v', 'e') => return 5,
                ('n', 'i', 'n', 'e') => return 9,
                _ => (),
            };
        }
        if str.len() - i >= 3 {
            match (str_bytes[i] as char, str_bytes[i + 1] as char, str_bytes[i + 2] as char) {
                ('o', 'n', 'e') => return 1,
                ('t', 'w', 'o') => return 2,
                ('s', 'i', 'x') => return 6,
                _ => (),
            };
        }
    }
    error!("Unable to find first digit in {}", str);
    0
}

fn find_last_digit_char_or_word(str: &str) -> u32 {
    let str_bytes = str.as_bytes();
    for (i, c) in str.char_indices().rev() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }

        if i >= 5 {
            match (str_bytes[i - 4] as char, str_bytes[i - 3] as char, str_bytes[i - 2] as char, str_bytes[i - 1] as char, str_bytes[i] as char) {
                ('t', 'h', 'r', 'e', 'e') => return 3,
                ('s', 'e', 'v', 'e', 'n') => return 7,
                ('e', 'i', 'g', 'h', 't') => return 8,
                _ => (),
            };
        }
        if i >= 4 {
            match (str_bytes[i - 3] as char, str_bytes[i - 2] as char, str_bytes[i - 1] as char, str_bytes[i] as char) {
                ('z', 'e', 'r', 'o') => return 0,
                ('f', 'o', 'u', 'r') => return 4,
                ('f', 'i', 'v', 'e') => return 5,
                ('n', 'i', 'n', 'e') => return 9,
                _ => (),
            };
        }
        if i >= 3 {
            match (str_bytes[i - 2] as char, str_bytes[i - 1] as char, str_bytes[i] as char) {
                ('o', 'n', 'e') => return 1,
                ('t', 'w', 'o') => return 2,
                ('s', 'i', 'x') => return 6,
                _ => (),
            };
        }
    }
    error!("Unable to find last digit in {}", str);
    0
}
