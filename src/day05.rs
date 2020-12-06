use std::collections::HashSet;

use itertools::{iproduct, Itertools};

fn parse_row(row_str: &str) -> u8 {
    let mut row = 0u8;
    row_str.chars().for_each(|c| {
        row = row << 1;
        match c {
            'B' => row += 1,
            _ => return,
        }
    });
    row
}

fn parse_col(col_str: &str) -> u8 {
    let mut col = 0u8;
    col_str.chars().for_each(|c| {
        col = col << 1;
        match c {
            'R' => col += 1,
            _ => return,
        }
    });
    col
}

pub fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let row = parse_row(&line[..7]) as u32;
            let col = parse_col(&line[7..]) as u32;
            row * 8 + col
        })
        .max()
        .expect("Day 5 Part 1 Failed")
}

pub fn part2(input: String) -> u32 {
    let mut seat_ids: Vec<_> = input
        .lines()
        .map(|line| {
            let row = parse_row(&line[..7]) as u32;
            let col = parse_col(&line[7..]) as u32;

            row * 8 + col
        })
        .collect();

    seat_ids.sort_unstable();
    seat_ids
        .iter()
        .tuple_windows()
        .filter_map(|(a, b)| if b - a > 1 { Some(a + 1) } else { None })
        .next()
        .unwrap()
}

#[test]
fn day05_test() {
    dbg!(parse_row("FBFBBFF"));
    dbg!(parse_col("RLR"));
}
