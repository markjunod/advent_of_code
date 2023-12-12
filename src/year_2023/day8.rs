use std::collections::HashMap;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref DESERT_MAP: DesertMap = DesertMap::from(include_str!("inputs/day8.txt").trim());
}

struct DesertMap {
    directions: Vec<u8>,
    neighbors_by_node: HashMap<&'static str, NodeNeighbors>,
}

impl DesertMap {
    fn next_node(&self, direction: char, current_node: &str) -> &str {
        let current_node_neighbors = self.neighbors_by_node.get(current_node).unwrap();
        if direction == 'L' {
            current_node_neighbors.left
        } else {
            current_node_neighbors.right
        }
    }
}

impl From<&'static str> for DesertMap {
    fn from(value: &'static str) -> Self {
        let parts = value.split("\n\n").collect::<Vec<_>>();
        let directions = parts[0].trim().as_bytes().to_vec();
        let neighbors_by_node = parts[1].trim().split("\n").fold(HashMap::new(), |mut accum, line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            accum.insert(node, NodeNeighbors { left, right });
            accum
        });
        
        DesertMap { directions, neighbors_by_node }
    }
}

#[derive(Debug)]
struct NodeNeighbors {
    left: &'static str,
    right: &'static str,
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 8 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 8 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut steps = 0;
    let mut current_node = "AAA";
    while current_node != "ZZZ" {
        let next_direction_index = steps % DESERT_MAP.directions.len();
        let next_direction = DESERT_MAP.directions[next_direction_index] as char;
        current_node = DESERT_MAP.next_node(next_direction, current_node);

        steps += 1;
    }

    info!("2023 - Day 8 - Part 1: Steps required to reach ZZZ {}", steps);
}

fn run_part2() {
    let start_nodes = DESERT_MAP.neighbors_by_node.keys().filter(|node| node.ends_with('A')).map(|node| *node).collect::<Vec<_>>();
    let cycle_lengths = start_nodes.into_iter().map(|node| {
        let mut steps = 0;
        let mut current_node = node.clone();
        while !current_node.ends_with('Z') {
            let next_direction_index = steps % DESERT_MAP.directions.len();
            let next_direction = DESERT_MAP.directions[next_direction_index] as char;
            current_node = DESERT_MAP.next_node(next_direction, current_node);

            steps += 1;
        }
        steps as u64
    }).collect::<Vec<_>>();

    let prime_factor_maps = cycle_lengths.into_iter().map(prime_factors).collect();
    let merged_prime_factors = merge_prime_factor_maps(&prime_factor_maps);
    let total_steps = multiply_prime_factors(merged_prime_factors);

    info!("2023 - Day 8 - Part 2: Steps required to complete all paths {}", total_steps);
}

fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut remainder = n;
    let mut current_factor = 2;
    let mut factors = HashMap::new();
    while remainder != 1 {
        if remainder % current_factor == 0 {
            while remainder % current_factor == 0 {
                factors.entry(current_factor).and_modify(|counter| *counter += 1).or_insert(1);
                remainder /= current_factor;
            }
            current_factor = 2;
        } else {
            if current_factor == 2 {
                current_factor += 1;
            } else {
                current_factor += 2;
            }
        }
    }

    factors
}

fn merge_prime_factor_maps(prime_factor_maps: &Vec<HashMap<u64, u64>>) -> HashMap<u64, u64> {
    let mut merged_prime_factors = HashMap::new();
    for prime_factors in prime_factor_maps {
        for (prime, count) in prime_factors {
            let current_count = merged_prime_factors.get(prime).unwrap_or(&0);
            merged_prime_factors.insert(*prime, *current_count.max(count));
        }
    }

    merged_prime_factors
}

fn multiply_prime_factors(prime_factors: HashMap<u64, u64>) -> u64 {
    prime_factors.iter().fold(1, |acc, (prime, count)| {
        acc * prime.pow(*count as u32)
    })
}
