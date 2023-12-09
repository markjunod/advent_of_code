use log::info;

use crate::timer::time_millis;

const TEST_TIMES: [u64; 3] = [7, 15, 30];
const TEST_RECORDS: [u64; 3] = [9, 40, 200];

const TIMES: [u64; 4] = [59, 70, 78, 78];
const RECORDS: [u64; 4] = [430, 1218, 1213, 1276];

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    info!("2023 - Day 6 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    info!("2023 - Day 6 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let times = TIMES;
    let records = RECORDS;
    let winning_ways = (0..times.len()).map(|i| {
        count_winning_ways(times[i], records[i])
    });

    let product = winning_ways.product::<u64>();

    info!("2023 - Day 6 - Part 1: Product of winning ways {}", product);
}

fn run_part2() {
    let total_time:u64 = 59_707_878; // test = 71_539, problem = 59_707_878
    let record: u64 = 430_121_812_131_276; // test = 940_200, problem = 430_121_812_131_276

    let winning_ways = count_winning_ways(total_time, record);

    info!("2023 - Day 6 - Part 2: Total ways to win {}", winning_ways);
}

fn count_winning_ways(total_time: u64, record: u64) -> u64 {
    let mut time = 1;
    while time * (total_time - time) < record {
        time += 1;
    }

    // have the min time needed to win
    // it's symmetrical, so total_time - winning_time will win as well
    // that range contains all the ways to win
    let max_time = total_time - time;
    max_time - time + 1
} 
