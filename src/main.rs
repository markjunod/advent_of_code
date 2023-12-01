use advent_of_code::year_2023;
use clap::ArgMatches;
use clap::clap_app;
use log::{error, warn};

use advent_of_code::year_2020;
use advent_of_code::year_2021;
use advent_of_code::year_2022;

fn main() {
    env_logger::init();

    let cli_arg_matches = cli_args();

    let run_day_args = cli_arg_matches.subcommand_matches("run_day");
    let run_year_args = cli_arg_matches.subcommand_matches("run_year");

    if run_day_args.is_some() {
        let year = run_day_args.unwrap().value_of("year");
        let day = run_day_args.unwrap().value_of("day");
        return run_day_for_year(year, day);
    }

    if run_year_args.is_some() {
        let year = run_year_args.unwrap().value_of("year");
        return run_all_for_year(year);
    }

    error!("No valid subcommand entered. Use one of 'run_day' or 'run_all' (see --help for all options)");
}

fn run_all_for_year(year: Option<&str>) {
    match parse_to_number(year) {
        2020 => year_2020::run_all(),
        2021 => year_2021::run_all(),
        2022 => year_2022::run_all(),
        2023 => year_2023::run_all(),
        y => warn!("Only years 2020-2023 have a run_all command implemented: given year {}", y),
    };
}

fn run_day_for_year(year: Option<&str>, day: Option<&str>) {
    match parse_to_number(year) {
        2020 => year_2020::run_day(parse_to_number(day)),
        2021 => year_2021::run_day(parse_to_number(day)),
        2022 => year_2022::run_day(parse_to_number(day)),
        2023 => year_2023::run_day(parse_to_number(day)),
        y => warn!("Only years 2020-2023 have any days implemented: given year {}", y),
    };
}

fn parse_to_number(opt_str: Option<&str>) -> u32 {
    match opt_str.unwrap_or("0").parse() {
        Ok(num) => num,
        Err(e) => panic!("Unable to parse {:?} as a number: {:?}", opt_str, e),
    }
}

fn cli_args() -> ArgMatches<'static> {
    clap_app!(app =>
        (version: "0.1.0")
        (author: "Mark Junod <mark.junod@gmail.com>")
        (about: "Solves Advent of Code problems")
        (@subcommand run_day =>
            (@arg year: -y --year +takes_value +required possible_values(&["2020", "2021", "2022", "2023"]) "The Advent of Code year to run")
            (@arg day: -d --day +takes_value +required "The day of the chosen year to run")
        )
        (@subcommand run_year =>
            (about: "Runs all problems sequentially for a given year")
            (@arg year: -y --year +takes_value +required possible_values(&["2020", "2021", "2022", "2023"]) "The Advent of Code year to run all problems for")
        )
    ).get_matches()
}
