use crate::timer::time_millis;

mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;

pub fn run_all() {
    let total_millis = time_millis(|| {
        day1::run();
        // day2::run();
        // day3::run();
        // day4::run();
        // day5::run();
        // day6::run();
        // day7::run();
    });

    println!("Running all days implemented for 2022 took {} millis", total_millis);
}

pub fn run_day(day: u32) {
    match day {
        1 => day1::run(),
        // 2 => day2::run(),
        // 3 => day3::run(),
        // 4 => day4::run(),
        // 5 => day5::run(),
        // 6 => day6::run(),
        // 7 => day7::run(),
        n if n > 25 => println!("Days greater than 25 are not an option"),
        n => println!("Day {} is not implemented yet", n),
    }
}
