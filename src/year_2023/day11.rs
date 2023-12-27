use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref IMAGE: Vec<Vec<char>> = include_str!("inputs/day11.txt")
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    static ref EMPTY_ROWS: HashSet<usize> = find_empty_rows(&IMAGE);
    static ref EMPTY_COLS: HashSet<usize> = find_empty_cols(&IMAGE);
}

fn find_empty_rows(image: &Vec<Vec<char>>) -> HashSet<usize> {
    let mut empty_rows = HashSet::new();
    for y in 0..image.len() {
        let mut is_empty = true;
        for x in 0..(image[y].len()) {
            if image[y][x] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_rows.insert(y);
        }
    }
    empty_rows
}

fn find_empty_cols(image: &Vec<Vec<char>>) -> HashSet<usize> {
    let mut empty_cols = HashSet::new();
    let row_len = image[0].len();
    for x in 0..row_len {
        let mut is_empty = true;
        for y in 0..image.len() {
            if image[y][x] != '.' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_cols.insert(x);
        }
    }
    empty_cols
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn dist(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 11 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 11 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let galaxies = find_galaxy_positions(2);
    let sum = sum_galaxy_distances(galaxies);

    info!("2023 - Day 11 - Part 1: Sum of distances between galaxies {}", sum);
}

fn run_part2() {
    let galaxies = find_galaxy_positions(1_000_000);
    let sum = sum_galaxy_distances(galaxies);

    info!("2023 - Day 11 - Part 2: Sum of distances between older galaxies {}", sum);
}

fn find_galaxy_positions(expansion_factor: u64) -> Vec<Position> {
    let mut positions = Vec::new();
    let mut y = 0;
    for i in 0..IMAGE.len() {
        if EMPTY_ROWS.contains(&i) {
            y += expansion_factor - 1;
        }
        let mut x = 0;
        for j in 0..IMAGE[i].len() {
            if EMPTY_COLS.contains(&j) {
                x += expansion_factor - 1;
            }

            if IMAGE[i][j] == '#' {
                positions.push(Position { x, y });
            }

            x += 1;
        }
        y += 1;
    }
    positions
}

fn sum_galaxy_distances(galaxies: Vec<Position>) -> u64 {
    let mut sum = 0;
    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            sum += galaxies[i].dist(&galaxies[j]);
        }
    }
    sum
}
