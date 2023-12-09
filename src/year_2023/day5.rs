use lazy_static::lazy_static;
use log::info;

use crate::timer::time_millis;

lazy_static! {
    static ref ALMANAC: Almanac = Almanac::from(include_str!("inputs/day5.txt").trim());
}

#[derive(Debug, Clone, Default)]
struct Almanac {
    seeds: Vec<i64>,
    seed_ranges: Vec<Range>,
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
                        almanac.seeds.clone().chunks(2).for_each(|chunk| almanac.seed_ranges.push(Range { min: chunk[0], length: chunk[1] }));
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
struct Range {
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

fn run_part2() {
    let locals = ALMANAC.seed_ranges.iter().flat_map(|seed_range| {
        let mut seed = seed_range.min;
        let mut locals = Vec::<i64>::new();
        let mut count = 0;
        while count < 5 && seed < seed_range.min + seed_range.length {
            let increasing_range = get_seed_local_with_range(seed);
            locals.push(increasing_range.min);
            seed += increasing_range.length;
            count += 1;
        }

        locals
    });

    let min_local = locals.min().unwrap();

    info!("2023 - Day 5 - Part 2: Lowest location number {}", min_local);
}

fn get_seed_local_with_range(seed: i64) -> Range {
    let soil_range = get_destination_with_range(seed, &ALMANAC.seed_to_soil);
    let fertilizer_range = get_destination_with_range(soil_range.min, &ALMANAC.soil_to_fertilizer);
    let water_range = get_destination_with_range(fertilizer_range.min, &ALMANAC.fertilizer_to_water);
    let light_range = get_destination_with_range(water_range.min, &ALMANAC.water_to_light);
    let temp_range = get_destination_with_range(light_range.min, &ALMANAC.light_to_temp);
    let humid_range = get_destination_with_range(temp_range.min, &ALMANAC.temp_to_humidity);
    let local_range = get_destination_with_range(humid_range.min, &ALMANAC.humidity_to_local);

    Range {
        min: local_range.min,
        length: vec![soil_range.length, fertilizer_range.length, water_range.length, light_range.length, temp_range.length, humid_range.length, local_range.length].into_iter().min().unwrap(),
    }
}

fn get_destination_with_range(source: i64, ranges: &Vec<AlmanacMapRange>) -> Range {
    let mut min_next_range = i64::MAX;
    for range in ranges {
        if source >= range.source_min && source < range.source_min + range.length {
            let min = source + (range.destination_min - range.source_min);
            let length = range.destination_min + range.length - min;
            return Range { min, length};
        } else if source < range.source_min && range.source_min < min_next_range {
            min_next_range = range.source_min;
        }
    }

    Range { min: source, length: min_next_range - source }
}
