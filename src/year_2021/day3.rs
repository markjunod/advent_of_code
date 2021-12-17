use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref REPORT: Vec<u32> = include_str!("inputs/day3.txt")
        .trim()
        .split("\n")
        .map(|line| u32::from_str_radix(line.trim(), 2).unwrap())
        .collect();
}

const REPORT_ENTRY_LENGTH: u32 = 12;

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2021 - Day 3 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2021 - Day 3 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut current_bit = 1;

    let mut gamma = 0;
    let mut epsilon = 0;
    for _ in 0..REPORT_ENTRY_LENGTH {
        if find_most_common_bit(current_bit, &REPORT, true) {
            gamma += current_bit;
        } else {
            epsilon += current_bit;
        }

        current_bit <<= 1;
    }

    println!("2021 - Day 3 - Part 1: {} gamma rate, {} epsilon rate, {} power consumption", gamma, epsilon, gamma * epsilon);
}

fn run_part2() {
    let o2_entry = find_o2_entry();
    let co2_entry = find_co2_entry();

    println!("2021 - Day 3 - Part 2: {} O2 rating, {} CO2 rating, {} life support", o2_entry, co2_entry, o2_entry * co2_entry);
}

fn find_o2_entry() -> u32 {
    let mut current_bit = 1 << (REPORT_ENTRY_LENGTH - 1);
    let mut o2_entries = REPORT.clone();
    for _ in 0..REPORT_ENTRY_LENGTH {
        if find_most_common_bit(current_bit, &o2_entries, true) {
            o2_entries = o2_entries.iter().filter(|&&entry| entry & current_bit == current_bit).map(|e| *e).collect();
        } else {
            o2_entries = o2_entries.iter().filter(|&&entry| entry & current_bit == 0).map(|e| *e).collect();
        }

        if o2_entries.len() == 1 {
            break;
        }

        current_bit >>= 1;
    }

    o2_entries[0]
}

fn find_co2_entry() -> u32 {
    let mut current_bit = 1 << (REPORT_ENTRY_LENGTH - 1);
    let mut co2_entries = REPORT.clone();
    for _ in 0..REPORT_ENTRY_LENGTH {
        if find_most_common_bit(current_bit, &co2_entries, true) {
            co2_entries = co2_entries.iter().filter(|&&entry| entry & current_bit == 0).map(|e| *e).collect();
        } else {
            co2_entries = co2_entries.iter().filter(|&&entry| entry & current_bit == current_bit).map(|e| *e).collect();
        }

        if co2_entries.len() == 1 {
            break;
        }

        current_bit >>= 1;
    }

    co2_entries[0]
}

fn find_most_common_bit(current_bit: u32, entries: &Vec<u32>, tie_breaker_bit: bool) -> bool {
    let mut zero_bit_count = 0;
    let mut one_bit_count = 0;
    for entry in entries.iter() {
        if *entry & current_bit == 0 {
            zero_bit_count += 1;
        } else {
            one_bit_count += 1;
        }
    }

    if one_bit_count == zero_bit_count {
        tie_breaker_bit
    } else if one_bit_count > zero_bit_count {
        true
    } else {
        false
    }
}
