use lazy_static::lazy_static;

use crate::timer::time_millis;

lazy_static! {
    static ref BOARDING_PASSES: Vec<&'static str> = include_str!("inputs/day5.txt").trim().split("\n").map(|line| line.trim()).collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2020 - Day 5 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2020 - Day 5 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut max_seat_id = 0;

    for i in 0..BOARDING_PASSES.len() {
        let seat_id = parse_seat_id(BOARDING_PASSES[i]);

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("2020 - Day 5 - Part 1: The highest seat id is {}", max_seat_id);
}

fn run_part2() {
    let mut seat_ids: Vec<u32> = BOARDING_PASSES.iter().map(|boarding_pass| parse_seat_id(boarding_pass)).collect();

    seat_ids.sort();

    let mut seat_id = 0;
    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            seat_id = seat_ids[i] + 1;
            break;
        }
    }

    println!("2020 - Day 5 - Part 2: My seat id is {}", seat_id);
}

fn parse_seat_id(boarding_pass: &str) -> u32 {
    let chars: Vec<char> = boarding_pass.chars().collect();

    let mut row = 0;
    for i in 0..7 {
        if let 'B' = chars[i] {
            row += 2u32.pow(6 - i as u32);
        }
    }

    let mut seat = 0;
    for i in 7..10 {
        if let 'R' = chars[i] {
            seat += 2u32.pow(9 - i as u32);
        }
    }

    row * 8 + seat
}
