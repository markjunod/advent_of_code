use std::{collections::{HashMap, HashSet}, hash::Hash};

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref ENGINE_SCHEMATIC: EngineSchematic = include_str!("inputs/day3.txt")
        .trim()
        .chars()
        .fold( EngineSchematicBuilder::default(), |mut builder, c| {
            if c.is_ascii_digit() {
                let mut current_number = builder.current_number.unwrap_or_default();
                current_number.push(c);
                builder.current_number = Some(current_number);
            } else if builder.current_number.is_some() {
                let length = builder.current_number.as_ref().unwrap().len();
                let number = Number { id: builder.current_id, value: builder.current_number.as_ref().unwrap().parse().unwrap() };
                for i in (builder.current_coordinate.x - length as i32)..builder.current_coordinate.x {
                    builder.engine_schematic.numbers_by_coords.insert(Coordinate { x: i, y: builder.current_coordinate.y }, number);
                }
                builder.current_id += 1;
                builder.current_number = None;
            }
            
            if c == '\n' {
                builder.current_coordinate.inc_y();
                return builder;
            }

            if c != '.' && !c.is_ascii_digit() {
                builder.engine_schematic.punctuation.push((c, builder.current_coordinate));
            }
            
            builder.current_coordinate.inc_x();
            builder
        })
        .build();
}

struct EngineSchematicBuilder {
    engine_schematic: EngineSchematic,
    current_coordinate: Coordinate,
    current_number: Option<String>,
    current_id: u32,
}

impl Default for EngineSchematicBuilder {
    fn default() -> Self {
        EngineSchematicBuilder { 
            engine_schematic: EngineSchematic::default(),
            current_coordinate: Coordinate::default(),
            current_number: None,
            current_id: 0,
        }
    }
}

impl EngineSchematicBuilder {
    fn build(self) -> EngineSchematic {
        self.engine_schematic
    }
}

#[derive(Debug)]
struct EngineSchematic {
    numbers_by_coords: HashMap<Coordinate, Number>,
    punctuation: Vec<(char, Coordinate)>,
}

impl Default for EngineSchematic {
    fn default() -> Self {
        EngineSchematic {
            numbers_by_coords: HashMap::new(),
            punctuation: Vec::new(),
        }
    }
}

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn inc_x(&mut self) {
        self.x += 1;
    }

    fn inc_y(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn neighbors(&self) -> Vec<Coordinate> {
        let mut neighbors = Vec::<Coordinate>::new();
        for i in (self.x - 1)..=(self.x + 1) {
            for j in (self.y - 1)..=(self.y + 1) {
                if i >= 0 && j >= 0 && (i != self.x || j != self.y) {
                    neighbors.push(Coordinate { x: i, y: j });
                }
            }
        }
        neighbors
    }
}

#[derive(Copy, Clone, Eq, Debug)]
struct Number {
    id: u32,
    value: u32,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 3 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 3 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut parts = HashSet::<Number>::new();
    ENGINE_SCHEMATIC.punctuation.iter().for_each(|(_, punct_coord)| {
        punct_coord.neighbors().iter().for_each(|neighbor| {
            if ENGINE_SCHEMATIC.numbers_by_coords.contains_key(neighbor) {
                parts.insert(*ENGINE_SCHEMATIC.numbers_by_coords.get(neighbor).unwrap());
            }
        })
    });

    let sum: u32 = parts.iter().map(|part| part.value).sum();

    info!("2023 - Day 3 - Part 1: Sum of engine parts {}", sum);
}

fn run_part2() {
    let sum: u64 = ENGINE_SCHEMATIC.punctuation.iter()
        .filter(|(c, _)| c == &'*')
        .map(|(_, coord)| {
            let neighbors = coord.neighbors().into_iter()
                .filter(|neighbor| ENGINE_SCHEMATIC.numbers_by_coords.contains_key(neighbor))
                .map(|coord| ENGINE_SCHEMATIC.numbers_by_coords.get(&coord).unwrap())
                .collect::<HashSet<_>>();
            if neighbors.len() == 2 {
                neighbors.into_iter().map(|number| number.value as u64).fold(1, |prod, x| prod * x)
            } else {
                0
            }
        })
        .sum();

    info!("2023 - Day 3 - Part 2: Sum of gear ratios {}", sum);
}
