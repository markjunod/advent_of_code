use std::io;

mod days;

fn main() {
    match choose_day() {
        Some(day) => run_day(day),
        None => println!("No valid day was entered. Exiting."),
    }
}

fn choose_day() -> Option<u8> {
    let mut non_number_count = 0;
    
    loop {
        if non_number_count == 2 {
            break None;
        }

        match read_day_number() {
            Some(day) => break Some(day),
            None => non_number_count += 1,
        }
    }
}

fn read_day_number() -> Option<u8> {
    println!("Enter the Advent of Code day number to run (two non-numbers or non-positive numbers in a row will exit): ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to read day number");

    match number.trim().parse() {
        Ok(num) => Some(num),
        Err(e) => {
            println!("{}", e);
            None
        },
    }
}

fn run_day(day: u8) {
    match day {
        1 => days::day1::run(),
        _ if day == 0 || day > 25 => println!("Only days from 1 through 25 are valid"),
        _ => println!("Day {} is not yet implemented", day),
    };
}
