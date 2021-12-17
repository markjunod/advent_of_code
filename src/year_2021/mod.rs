use crate::timer::time_millis;

mod day1;
mod day2;
mod day3;

pub fn run_all() {
    let total_millis = time_millis(|| {
        day1::run();
        day2::run();
        day3::run();
    });

    println!("Running all days implemented for 2020 took {} millis", total_millis);
}

pub fn run_day(day: u32) {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        n if n > 25 => println!("Days greater than 25 are not an option"),
        n => println!("Day {} is not implemented yet", n),
    }
}
