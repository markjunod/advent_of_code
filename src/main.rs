use clap::ArgMatches;
use clap::clap_app;

pub mod timer;
mod year_2020;

fn main() {
    let cli_arg_matches = cli_args();

    let subcommand = cli_arg_matches.subcommand_matches("run_all");

    match subcommand {
        Some(subcommand_arg_matches) => run_all_for_year(subcommand_arg_matches.value_of("year")),
        None => run_day_for_year(cli_arg_matches.value_of("year"), cli_arg_matches.value_of("day")),
    };
}

fn run_all_for_year(year: Option<&str>) {
    match parse_to_number(year) {
        2020 => year_2020::run_all(),
        _ => println!("Only year 2020 has a run_all command implemented"),
    };
}

fn run_day_for_year(year: Option<&str>, day: Option<&str>) {
    match parse_to_number(year) {
        2020 => year_2020::run_day(parse_to_number(day)),
        _ => println!("Only year 2020 has any days implemented"),
    };
}

fn parse_to_number(opt_str: Option<&str>) -> u32 {
    match opt_str.unwrap_or("-1").parse() {
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
            (@arg year: -y --year +takes_value +required possible_values(&["2020", "2021"]) "The Advent of Code year to run")
            (@arg day: -d --day +takes_value +required "The day of the chosen year to run")
        )
        (@subcommand run_all =>
            (about: "Runs all problems sequentially for a given year")
            (@arg year: -y --year +takes_value +required possible_values(&["2020", "2021"]) "The Advent of Code year to run all problems for")
        )
    ).get_matches()
}
