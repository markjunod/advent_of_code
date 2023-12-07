use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref ALMANAC: Almanac = Almanac::from(include_str!("inputs/day5.txt").trim());
}

#[derive(Debug, Clone, Default)]
struct Almanac {
    seeds: Vec<i64>,
    seed_ranges: Vec<SeedRange>,
    seed_to_soil: Vec<AlmanacMapRange>,
    soil_to_fertilizer: Vec<AlmanacMapRange>,
    fertilizer_to_water: Vec<AlmanacMapRange>,
    water_to_light: Vec<AlmanacMapRange>,
    light_to_temp: Vec<AlmanacMapRange>,
    temp_to_humidity: Vec<AlmanacMapRange>,
    humidity_to_local: Vec<AlmanacMapRange>,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut almanac = Almanac::default();
        let mut current_map = "";
        value.split('\n').filter(|line| !line.is_empty()).for_each(|line| {
            if line > "a" {
                current_map = &line[0..5];
            } else {
                match current_map {
                    "seeds" => {
                        almanac.seeds = line.trim().split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
                        almanac.seeds.clone().chunks(2).for_each(|chunk| almanac.seed_ranges.push(SeedRange { min: chunk[0], length: chunk[1] }));
                    },
                    "seed-" => almanac.seed_to_soil.push(AlmanacMapRange::from(line.trim())),
                    "soil-" => almanac.soil_to_fertilizer.push(AlmanacMapRange::from(line.trim())),
                    "ferti" => almanac.fertilizer_to_water.push(AlmanacMapRange::from(line.trim())),
                    "water" => almanac.water_to_light.push(AlmanacMapRange::from(line.trim())),
                    "light" => almanac.light_to_temp.push(AlmanacMapRange::from(line.trim())),
                    "tempe" => almanac.temp_to_humidity.push(AlmanacMapRange::from(line.trim())),
                    "humid" => almanac.humidity_to_local.push(AlmanacMapRange::from(line.trim())),
                    _ => panic!("Unknown map type {}", current_map),
                };
            }
        });
        
        almanac
    }
}

#[derive(Debug, Clone, Default)]
struct SeedRange {
    min: i64,
    length: i64,
}

#[derive(Debug, Clone, Default)]
struct AlmanacMapRange {
    source_min: i64,
    destination_min: i64,
    length: i64,
}

impl From<&str> for AlmanacMapRange {
    fn from(value: &str) -> Self {
        let ranges_vec = value.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
        AlmanacMapRange {
            source_min: ranges_vec[1],
            destination_min: ranges_vec[0],
            length: ranges_vec[2],
        }
    }
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 5 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 5 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let locals = ALMANAC.seeds.iter().map(|&seed| get_seed_local(seed));

    let min_local = locals.min().unwrap();

    info!("2023 - Day 5 - Part 1: Lowest location number {}", min_local);
}

fn run_part2() {
    let locals = ALMANAC.seed_ranges.iter().flat_map(|seed_range| {
        (seed_range.min..(seed_range.min + seed_range.length)).map(get_seed_local)
    });

    let min_local = locals.min().unwrap();

    info!("2023 - Day 5 - Part 2: Lowest location number {}", min_local);
}

fn get_seed_local(seed: i64) -> i64 {
    let soil = get_destination(seed, &ALMANAC.seed_to_soil);
    let fertilizer = get_destination(soil, &ALMANAC.soil_to_fertilizer);
    let water = get_destination(fertilizer, &ALMANAC.fertilizer_to_water);
    let light = get_destination(water, &ALMANAC.water_to_light);
    let temp = get_destination(light, &ALMANAC.light_to_temp);
    let humid = get_destination(temp, &ALMANAC.temp_to_humidity);
    get_destination(humid, &ALMANAC.humidity_to_local)
}

fn get_destination(source: i64, ranges: &Vec<AlmanacMapRange>) -> i64 {
    for range in ranges {
        if source >= range.source_min && source < range.source_min + range.length {
            return source + (range.destination_min - range.source_min);
        }
    }

    source
}
