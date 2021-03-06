use itertools::Itertools;

fn parse(input: &str) -> Vec<(usize, usize, char, &str)> {
    input
        .lines()
        .filter_map(|line| {
            let (range, c, passwd) = line.split(' ').collect_tuple()?;

            let (min, max) = range.split('-').collect_tuple()?;
            let (min, max): (usize, usize) = (min.parse().ok()?, max.parse().ok()?);

            let c = c.chars().next()?;

            Some((min, max, c, passwd))
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: String) -> usize {
    let input = parse(&input); //.expect("Day 02 Part 1 -- Failed to Parse");
    input
        .into_iter()
        .filter(|(min, max, c, passwd)| {
            let count = passwd.chars().filter(|x| x == c).count();
            count >= *min && count <= *max
        })
        .count()
}

pub fn part2(input: String) -> usize {
    let input = parse(&input);

    input
        .into_iter()
        .filter(|(first, second, c, passwd)| {
            let first = passwd.chars().nth(*first - 1).unwrap() == *c;
            let second = passwd.chars().nth(*second - 1).unwrap() == *c;

            first ^ second
        })
        .count()
}
