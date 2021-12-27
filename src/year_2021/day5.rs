use lazy_static::lazy_static;

use crate::timer::time_millis;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Copy, Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point
}

lazy_static! {
    static ref VENT_LINES: Vec<Line> = include_str!("inputs/day5.txt")
        .trim()
        .split("\n")
        .map(|line_str| {
            let point_strs: Vec<&'static str> = line_str.trim().split(" -> ").collect();
            let p1_strs: Vec<&'static str> = point_strs[0].split(",").collect();
            let p2_strs: Vec<&'static str> = point_strs[1].split(",").collect();

            let p1 = Point {
                x: p1_strs[0].parse::<u32>().unwrap(),
                y: p1_strs[1].parse::<u32>().unwrap(),
            };
            let p2 = Point {
                x: p2_strs[0].parse::<u32>().unwrap(),
                y: p2_strs[1].parse::<u32>().unwrap(),
            };

            Line { p1, p2 }
        })
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2021 - Day 5 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2021 - Day 5 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut point_counts = HashMap::new();
    VENT_LINES.iter().for_each(|line: &Line| {
        if line.p1.x == line.p2.x {
            add_hor_line(&mut point_counts, &line);
        } else if line.p1.y == line.p2.y {
            add_vert_line(&mut point_counts, &line);
        }
    });

    point_counts.retain(|_, &mut count| count > 1);

    println!("2021 - Day 5 - Part 1: Found {} dangerous points", point_counts.len());
}

fn run_part2() {
    let mut point_counts = HashMap::new();

    VENT_LINES.iter().for_each(|line: &Line| {
        if line.p1.x == line.p2.x {
            add_hor_line(&mut point_counts, &line);
        } else if line.p1.y == line.p2.y {
            add_vert_line(&mut point_counts, &line);
        } else {
            add_diag_line(&mut point_counts, &line);
        }
    });

    point_counts.retain(|_, &mut count| count > 1);

    println!("2021 - Day 5 - Part 2: Found {} dangerous points", point_counts.len());
}

fn add_vert_line(point_counts: &mut HashMap<Point, i32>, line: &Line) {
    let min = line.p1.x.min(line.p2.x);
    let max = line.p1.x.max(line.p2.x) + 1;
    for n in min..max {
        let point = Point { x: n, y: line.p1.y };
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }
}

fn add_hor_line(point_counts: &mut HashMap<Point, i32>, line: &Line) {
    let min = line.p1.y.min(line.p2.y);
    let max = line.p1.y.max(line.p2.y) + 1;
    for n in min..max {
        let point = Point { x: line.p1.x, y: n };
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }
}

fn add_diag_line(point_counts: &mut HashMap<Point, i32>, line: &Line) {
    let line_length = (line.p1.x as i32 - line.p2.x as i32).abs() as u32;
    let (start_x, start_y, is_asc) = if line.p1.x <= line.p2.x {
        (line.p1.x, line.p1.y, line.p1.y <= line.p2.y)
    } else {
        (line.p2.x, line.p2.y, line.p2.y <= line.p1.y)
    };
    for n in 0..(line_length + 1) {
        let y = if is_asc {
            start_y + n
        } else {
            start_y - n
        };

        let point = Point { x: start_x + n, y };
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }
}
