use log::{error, info, warn};
use crate::timer::time_millis;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

pub fn run_all() {
    let total_millis = time_millis(|| {
        day1::run();
        day2::run();
        day3::run();
        day4::run();
        day5::run();
        day6::run();
        day7::run();
        day8::run();
        day9::run();
        day10::run();
        day11::run();
    });

    info!("Running all days implemented for 2023 took {} millis", total_millis);
}

pub fn run_day(day: u32) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        11 => day11::run(),
        n if n > 25 => error!("Days greater than 25 are not an option"),
        n => warn!("Day {} is not implemented yet", n),
    }
}
