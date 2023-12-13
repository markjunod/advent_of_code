use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref OASIS_OUTPUT: Vec<Vec<Vec<i64>>> = include_str!("inputs/day9.txt").trim()
        .split('\n')
        .into_iter()
        .map(|line| {
            let history = line.trim().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<i64>>();
            complete_differences(history)
        })
        .collect();
}

fn complete_differences(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences = vec![history];
    let mut idx = 0;
    while differences[idx].iter().any(|n| n != &0) {
        let mut difference = Vec::<i64>::new();
        for i in 0..(differences[idx].len() - 1) {
            difference.push(differences[idx][i + 1] - differences[idx][i]);
        }
        differences.push(difference);
        idx += 1;
    }

    differences
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 9 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 9 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let predicted_values = OASIS_OUTPUT.iter().map(|differences| {
        future_difference(differences)
    });
    let sum = predicted_values.sum::<i64>();

    info!("2023 - Day 9 - Part 1: Sum of predicted future values {}", sum);
}

fn future_difference(differences: &Vec<Vec<i64>>) -> i64 {
    let last_idx = differences.len() - 1;
    let mut future_values = Vec::<i64>::new();
    future_values.push(0);
    for i in (0..last_idx).rev() {
        let n1 = *differences[i].last().unwrap();
        let n2 = *future_values.last().unwrap();
        future_values.push(n1 + n2);
    }

    *future_values.last().unwrap()
}

fn run_part2() {
    let predicted_values = OASIS_OUTPUT.iter().map(|differences| {
        past_difference(differences)
    });
    let sum = predicted_values.sum::<i64>();

    info!("2023 - Day 9 - Part 2: Sum of predicted past values {}", sum);
}

fn past_difference(differences: &Vec<Vec<i64>>) -> i64 {
    let last_idx = differences.len() - 1;
    let mut past_values = Vec::<i64>::new();
    past_values.push(0);
    for i in (0..last_idx).rev() {
        let n1 = *differences[i].first().unwrap();
        let n2 = *past_values.last().unwrap();
        past_values.push(n1 - n2);
    }

    *past_values.last().unwrap()
}
