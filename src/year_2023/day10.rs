use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref PIPE_MAP: PipeMap = PipeMap::from(include_str!("inputs/day10.txt"));
}

fn find_start_position(pipes: &Vec<Vec<Pipe>>) -> Position {
    for (y, pipe_row) in pipes.iter().enumerate() {
        for (x, pipe) in pipe_row.iter().enumerate() {
            if pipe == &Pipe::Start {
                return Position { x, y };
            }
        }
    }

    panic!("Unable to find Start pipe");
}

fn find_start_pipe(pipes: &Vec<Vec<Pipe>>, start_position: &Position) -> Pipe {
    let Position { x, y } = start_position.clone();
    let north = pipes.get((y as i32 - 1) as usize).and_then(|row| row.get(x)).unwrap_or(&Pipe::None);
    let east = pipes.get(y).and_then(|row| row.get(x+1)).unwrap_or(&Pipe::None);
    let south = pipes.get(y+1).and_then(|row| row.get(x)).unwrap_or(&Pipe::None);
    let west = pipes.get(y).and_then(|row| row.get((x as i32 - 1) as usize)).unwrap_or(&Pipe::None);
    match (north, east, south, west) {
        (_, _, Pipe::NS | Pipe::NW | Pipe::NE, Pipe::EW | Pipe::NE | Pipe::SE) => Pipe::SW,
        (_, Pipe::EW | Pipe::NW | Pipe::SW, _, Pipe::EW | Pipe::NE | Pipe::SE) => Pipe::EW,
        (Pipe::NS | Pipe::SE | Pipe::SW, _, _, Pipe::EW | Pipe::NE | Pipe::SE) => Pipe::NW,
        (_, Pipe::EW | Pipe::NW | Pipe::SW, Pipe::NS | Pipe::NW | Pipe::NE, _) => Pipe::SE,
        (Pipe::NS | Pipe::SE | Pipe::SW, _, Pipe::NS | Pipe::NW | Pipe::NE, _) => Pipe::NS,
        (Pipe::NS | Pipe::SE | Pipe::SW, Pipe::EW | Pipe::NW | Pipe::SW, _, _) => Pipe::NE,
        _ => panic!("Start position {:?} has too many neighbors", start_position),
    }
}

struct PipeMap {
    start_position: Position,
    start_pipe: Pipe,
    loop_positions: HashSet<Position>,
    pipes: Vec<Vec<Pipe>>,
}

impl From<&str> for PipeMap {
    fn from(value: &str) -> Self {
        println!("PipeMap from str");
        let pipes = value.trim()
            .split('\n')
            .into_iter()
            .map(|line| {
                line.chars().map(Pipe::from).collect::<Vec<_>>()
            })
            .collect();

        let start_position = find_start_position(&pipes);
        let start_pipe = find_start_pipe(&pipes, &start_position);
        let loop_positions = HashSet::new();

        let mut pipe_map = PipeMap { start_position, start_pipe, loop_positions, pipes };
        pipe_map.travel_loop();

        pipe_map
    }
}

impl PipeMap {
    fn travel_loop(&mut self) {
        let mut previous = self.start_position;
        let (mut current, _) = self.get_start_moves();
        self.loop_positions.insert(self.start_position);
        while current != self.start_position {
            self.loop_positions.insert(current);
            let next = self.get_next_position(&current, &previous);
            previous = current;
            current = next;
        }
    }

    fn get_start_moves(&self) -> (Position, Position) {
        match self.start_pipe {
            Pipe::EW => (self.start_position.move_east(), self.start_position.move_west()),
            Pipe::NE => (self.start_position.move_north(), self.start_position.move_east()),
            Pipe::NS => (self.start_position.move_north(), self.start_position.move_south()),
            Pipe::NW => (self.start_position.move_north(), self.start_position.move_west()),
            Pipe::SE => (self.start_position.move_south(), self.start_position.move_east()),
            Pipe::SW => (self.start_position.move_south(), self.start_position.move_west()),
            _ => panic!("Invalid start pipe {:?}", self.start_pipe),
        }
    }

    fn get_pipe(&self, position: &Position) -> Pipe {
        if position == &self.start_position {
            return self.start_pipe;
        }

        self.pipes[position.y][position.x]
    }

    fn get_next_position(&self, current: &Position, previous: &Position) -> Position {
        match self.get_pipe(current) {
            Pipe::EW if previous == &current.move_east() => current.move_west(),
            Pipe::EW => current.move_east(),
            Pipe::NS if previous == &current.move_north() => current.move_south(),
            Pipe::NS => current.move_north(),
            Pipe::NE if previous == &current.move_north() => current.move_east(),
            Pipe::NE => current.move_north(),
            Pipe::NW if previous == &current.move_north() => current.move_west(),
            Pipe::NW => current.move_north(),
            Pipe::SE if previous == &current.move_south() => current.move_east(),
            Pipe::SE => current.move_south(),
            Pipe::SW if previous == &current.move_south() => current.move_west(),
            Pipe::SW => current.move_south(),
            _ => panic!("Unable to move out of position {:?}", current),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_north(&self) -> Position {
        Position { x: self.x, y: self.y.checked_sub(1).unwrap_or(self.y) }
    }

    fn move_south(&self) -> Position {
        Position { x: self.x, y: self.y + 1 }
    }

    fn move_west(&self) -> Position {
        Position { x: self.x.checked_sub(1).unwrap_or(self.x), y: self.y }
    }

    fn move_east(&self) -> Position {
        Position { x: self.x + 1, y: self.y }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Pipe {
    NS,
    NW,
    NE,
    EW,
    SW,
    SE,
    Start,
    None
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'S' => Pipe::Start,
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '.' => Pipe::None,
            _ => panic!("{} can't be mapped to a Pipe", value),
        }
    }
}

impl Pipe {
    fn is_turn(&self) -> bool {
        match self {
            Pipe::EW | Pipe::NS | Pipe::None => false,
            _ => true,
        }
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 10 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 10 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let steps = PIPE_MAP.loop_positions.len() / 2;

    info!("2023 - Day 10 - Part 1: Furthest steps from start {}", steps);
}

fn run_part2() {
    let mut count = 0;
    for y in 1..(PIPE_MAP.pipes.len() - 1) {
        let mut is_inside = false;
        let mut previous_turn: Option<Pipe> = None;
        for x in 0..(PIPE_MAP.pipes[y].len() - 1) {
            let position = Position { x, y };
            if is_inside && !PIPE_MAP.loop_positions.contains(&position) {
                count += 1;
            } else if flip_is_inside(&position, &previous_turn) {
                is_inside = !is_inside;
            }
            
            let pipe = if PIPE_MAP.loop_positions.contains(&position) { PIPE_MAP.get_pipe(&position) } else { Pipe::None };
            if previous_turn.is_none() && pipe.is_turn() {
                previous_turn = Some(pipe);
            } else if previous_turn.is_some() && pipe.is_turn() {
                previous_turn = None;
            }
        }
    }

    info!("2023 - Day 10 - Part 2: Total area inside loop {}", count);
}

// if outside, only way to get inside is to hit a NS pipe, or NE/SE pipe, followed by EW pipes, then a SW/NW
// if inside, only way to get outside is the same
fn flip_is_inside(position: &Position, previous_turn: &Option<Pipe>) -> bool {
    if PIPE_MAP.loop_positions.contains(position) {
        match PIPE_MAP.get_pipe(position) {
            Pipe::NS => true,
            Pipe::NW if previous_turn.unwrap() == Pipe::SE => true,
            Pipe::SW if previous_turn.unwrap() == Pipe::NE => true,
            _ => false
        }
    } else {
        false
    }
}
