use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref GAMES: Vec<Game> = include_str!("inputs/day2.txt")
        .trim()
        .split("\n")
        .map(Game::from)
        .collect();
}

struct Game {
    id: u32,
    pulls: Vec<GamePull>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let parts = value.split(':').collect::<Vec<&str>>();
        let id: u32 = parts[0].split(' ').collect::<Vec<&str>>().last().unwrap().parse().unwrap();
        let pulls = parts[1].split(';').map(GamePull::from).collect::<Vec<_>>();
        Game { id, pulls }
    }
}

struct GamePull {
    blue: u32,
    green: u32,
    red: u32,
}

impl From<&str> for GamePull {
    fn from(value: &str) -> Self {
        let mut game_pull = GamePull { blue: 0, green: 0, red: 0 };
        value
            .split(',')
            .map(|cubes_str| cubes_str.trim())
            .for_each(|cubes_str| {
                let count: u32 = cubes_str.split(' ').collect::<Vec<&str>>().first().unwrap().parse().unwrap();
                if cubes_str.ends_with("blue") {
                    game_pull.blue = count;
                }
                if cubes_str.ends_with("green") {
                    game_pull.green = count;
                }
                if cubes_str.ends_with("red") {
                    game_pull.red = count;
                }
            });
        game_pull
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 2 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 2 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut sum = 0;
    GAMES.iter().for_each(|game| {
        if game.pulls.iter().all(|pull| pull.blue <= 14 && pull.green <= 13 && pull.red <= 12) {
            sum += game.id;
        }
    });

    info!("2023 - Day 2 - Part 1: Sum of game ids {}", sum);
}

fn run_part2() {
    let mut sum = 0;
    GAMES.iter().for_each(|game| {
        let blues = game.pulls.iter().map(|pull| pull.blue).max().unwrap();
        let greens = game.pulls.iter().map(|pull| pull.green).max().unwrap();
        let reds = game.pulls.iter().map(|pull| pull.red).max().unwrap();
        let power = blues * greens * reds;
        sum += power; 
    });

    info!("2023 - Day 2 - Part 2: Sum of set powers {}", sum);
}
