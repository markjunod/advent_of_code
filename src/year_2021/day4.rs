use lazy_static::lazy_static;

use crate::timer::time_millis;

#[derive(Copy, Clone)]
struct BingoEntry {
    num: u32,
    marked: bool
}

lazy_static! {
    static ref BINGO_INPUT: Vec<&'static str> = include_str!("inputs/day4.txt")
        .trim()
        .split("\n\n")
        .collect();

    static ref BINGO_NUMBERS: Vec<u32> = BINGO_INPUT[0].trim().split(",").map(|n| n.parse::<u32>().unwrap()).collect();

    static ref BINGO_BOARDS: Vec<Vec<Vec<BingoEntry>>> = BINGO_INPUT[1..].iter()
        .map(|board_str| board_str.split("\n")
            .map(|row_str| row_str.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap()).map(|num| BingoEntry { num, marked: false }).collect())
            .collect())
        .collect();
}

pub fn run() {
    let part1_millis = time_millis(|| run_part1());

    println!("2021 - Day 4 - Part 1: Took {} millis", part1_millis);

    let part2_millis = time_millis(|| run_part2());

    println!("2021 - Day 4 - Part 2: Took {} millis", part2_millis);
}

fn run_part1() {
    let mut winner_score = -1;
    let mut cur_bingo_num_idx = 0;
    let mut boards = BINGO_BOARDS.clone();
    while winner_score < 0 {
        boards.iter_mut().for_each(|board| mark_board(BINGO_NUMBERS[cur_bingo_num_idx], board));

        boards.iter().for_each(|board| {
            if is_winner(board) {
                winner_score = calc_score(BINGO_NUMBERS[cur_bingo_num_idx], board);
            }
        });

        cur_bingo_num_idx += 1;
    }

    println!("2021 - Day 4 - Part 1: Winning score is {}", winner_score);
}

fn run_part2() {
    let mut cur_bingo_num_idx = 0;
    let mut boards = BINGO_BOARDS.clone();
    while boards.len() > 1 {
        boards.iter_mut().for_each(|board| mark_board(BINGO_NUMBERS[cur_bingo_num_idx], board));

        boards = boards.into_iter().filter(|board| !is_winner(board)).collect();

        cur_bingo_num_idx += 1;
    }

    let mut last_winning_board = &mut boards[0];
    while !is_winner(last_winning_board) {
        mark_board(BINGO_NUMBERS[cur_bingo_num_idx], last_winning_board);

        cur_bingo_num_idx += 1;
    }

    let last_score = calc_score(BINGO_NUMBERS[cur_bingo_num_idx - 1], &last_winning_board);

    println!("2021 - Day 4 - Part 2: Last winning score is {}", last_score);
}

fn mark_board(num: u32, board: &mut Vec<Vec<BingoEntry>>) {
    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|entry| {
            if entry.num == num {
                entry.marked = true;
                return;
            }
        });
    });
}

fn calc_score(num: u32, board: &Vec<Vec<BingoEntry>>) -> i32 {
    let sum: u32 = board.iter().flatten()
        .filter(|entry| !entry.marked)
        .map(|entry| entry.num)
        .sum();

    (num * sum) as i32
}

fn is_winner(board: &Vec<Vec<BingoEntry>>) -> bool {
    is_row_winner(board) || is_col_winner(board)
}

fn is_row_winner(board: &Vec<Vec<BingoEntry>>) -> bool {
    for row in board.iter() {
        if row.iter().fold(true, |all_marked, entry| all_marked && entry.marked) {
            return true;
        }
    }

    false
}

fn is_col_winner(board: &Vec<Vec<BingoEntry>>) -> bool {
    for i in 0..board.len() {
        if board.iter().fold(true, |all_marked, row| all_marked && row[i].marked) {
            return true;
        }
    }

    false
}
