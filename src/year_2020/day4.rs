use std::collections::HashSet;

use lazy_static::lazy_static;
use log::info;
use regex::Regex;

use crate::timer::time_millis;

lazy_static! {
    static ref PASSPORTS: Vec<&'static str> = include_str!("inputs/day4.txt").trim().split("\n\n").collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2020 - Day 4 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2020 - Day 4 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let valid_passports = PASSPORTS.iter().filter(|passport| passport_contains_all_fields(passport)).count();

    info!("2020 - Day 4 - Part 1: {} valid passports", valid_passports);
}

fn run_part2() {
    let valid_passports = PASSPORTS.iter().filter(|passport| is_passport_valid(passport)).count();

    info!("2020 - Day 4 - Part 2: {} valid passports", valid_passports);
}

fn is_passport_valid(passport: &str) -> bool {
    if passport_contains_all_fields(passport) {
        let (byr, iyr, eyr, hgt, hcl, ecl, pid) = split_passport_into_fields(passport);

        is_byr_valid(byr) 
            && is_iyr_valid(iyr) 
            && is_eyr_valid(eyr) 
            && is_hgt_valid(hgt) 
            && is_hcl_valid(hcl)
            && is_ecl_valid(ecl)
            && is_pid_valid(pid)
    } else {
        false
    }
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn passport_contains_all_fields(passport: &str) -> bool {
    REQUIRED_FIELDS.iter().fold(true, |is_valid, field| is_valid && passport.contains(field))
}

fn split_passport_into_fields(passport: &str) -> (&str, &str, &str, &str, &str, &str, &str) {
    let fields = passport.split_whitespace();

    let mut tuple = ("", "", "", "", "", "", "");
    fields.for_each(|field| match field {
        _ if field.starts_with("byr") => tuple.0 = &field[4..],
        _ if field.starts_with("iyr") => tuple.1 = &field[4..],
        _ if field.starts_with("eyr") => tuple.2 = &field[4..],
        _ if field.starts_with("hgt") => tuple.3 = &field[4..],
        _ if field.starts_with("hcl") => tuple.4 = &field[4..],
        _ if field.starts_with("ecl") => tuple.5 = &field[4..],
        _ if field.starts_with("pid") => tuple.6 = &field[4..],
        _ => {},
    });

    tuple
}

const MIN_BYR: u32 = 1920;
const MAX_BYR: u32 = 2002;
fn is_byr_valid(byr: &str) -> bool {
    match byr.parse() {
        Ok(num) => MIN_BYR <= num && num <= MAX_BYR,
        Err(_) => false,
    }
}

const MIN_IYR: u32 = 2010;
const MAX_IYR: u32 = 2020;
fn is_iyr_valid(iyr: &str) -> bool {
    match iyr.parse() {
        Ok(num) => MIN_IYR <= num && num <= MAX_IYR,
        Err(_) => false,
    }
}

const MIN_EYR: u32 = 2020;
const MAX_EYR: u32 = 2030;
fn is_eyr_valid(eyr: &str) -> bool {
    match eyr.parse() {
        Ok(num) => MIN_EYR <= num && num <= MAX_EYR,
        Err(_) => false,
    }
}

const MIN_HGT_CM: u32 = 150;
const MAX_HGT_CM: u32 = 193;
const MIN_HGT_IN: u32 = 59;
const MAX_HGT_IN: u32 = 76;
fn is_hgt_valid(hgt: &str) -> bool {
    if hgt.len() < 4 {
        false
    } else if hgt.ends_with("cm") {
        match hgt[0..(hgt.len() - 2)].parse() {
            Ok(num) => MIN_HGT_CM <= num && num <= MAX_HGT_CM,
            Err(_) => false,
        }
    } else if hgt.ends_with("in") {
        match hgt[0..(hgt.len() - 2)].parse() {
            Ok(num) => MIN_HGT_IN <= num && num <= MAX_HGT_IN,
            Err(_) => false,
        }
    } else {
        false
    }
}

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new(r"#[0-9a-f]").unwrap();
}
fn is_hcl_valid(hcl: &str) -> bool {
    HCL_REGEX.is_match(hcl)
}

lazy_static! {
    static ref VALID_ECLS: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
}
fn is_ecl_valid(ecl: &str) -> bool {
    VALID_ECLS.contains(ecl)
}

lazy_static! {
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}
fn is_pid_valid(pid: &str) -> bool {
    PID_REGEX.is_match(pid)
}
