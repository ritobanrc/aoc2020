use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|l| l.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(None, |acc, line| {
                    let chars: HashSet<_> = line.chars().collect();
                    if let Some(acc) = acc {
                        Some(chars.intersection(&acc).copied().collect())
                    } else {
                        Some(chars)
                    }
                })
                .unwrap()
                .len()
        })
        .sum()
}
