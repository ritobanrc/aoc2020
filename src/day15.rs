use std::collections::BTreeMap;

use crate::Part;

pub fn solutions(input: String, part: Part) -> u32 {
    let starting_nums: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut last_spoken = BTreeMap::new();
    for (i, n) in starting_nums.iter().enumerate() {
        last_spoken.insert(*n, i + 1);
    }

    let mut next_number = 0; // since it's the first time

    let end = match part {
        Part::Part1 => 2020,
        Part::Part2 => 30000000,
    };

    for turn in starting_nums.len() + 1..end {
        let prev = *last_spoken.get(&next_number).unwrap_or(&turn);
        last_spoken.insert(next_number, turn);

        next_number = (turn - prev) as u32;
    }

    next_number
}

#[test]
fn day15_test() {
    let input = "1,3,2";
    assert_eq!(1, solutions(input.to_owned(), Part::Part1));
}
