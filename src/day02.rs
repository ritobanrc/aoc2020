use anyhow::{anyhow, Result};
use itertools::Itertools;

fn parse(input: &str) -> Result<Vec<(usize, usize, char, &str)>> {
    Ok(input
        .lines()
        .map(|line| {
            let (range, c, passwd) = line
                .split(' ')
                .collect_tuple()
                .ok_or(anyhow!("Failed to split input line"))?;

            let (min, max) = range
                .split('-')
                .collect_tuple()
                .ok_or(anyhow!("Failed to parse range."))?;

            let (min, max): (usize, usize) = (min.parse()?, max.parse()?);

            let c = c
                .chars()
                .next()
                .ok_or(anyhow!("Failed to prase character"))?;

            Ok((min, max, c, passwd))
        })
        .collect::<Result<Vec<_>>>()?)
}

// Parser combinator approach
fn parse2(input: &str) -> Vec<(usize, usize, char, String)> {
    use combine::{
        many1,
        parser::char::{digit, letter, space},
        token, Parser,
    };
    input
        .lines()
        .map(|line| {
            (
                many1::<String, _, _>(digit()),
                token('-'),
                many1::<String, _, _>(digit()),
                space(),
                letter(),
                token(':'),
                space(),
                many1::<String, _, _>(letter()),
            )
                .map(|(min, _, max, _, c, _, _, passwd)| {
                    let min: usize = str::parse(&min).unwrap();
                    let max: usize = str::parse(&max).unwrap();
                    (min, max, c, passwd)
                })
                .parse(line)
                .unwrap()
                .0
        })
        .collect()
}

pub fn part1(input: String) -> usize {
    let input = parse2(&input); //.expect("Day 02 Part 1 -- Failed to Parse");
    input
        .into_iter()
        .filter(|(min, max, c, passwd)| {
            let count = passwd.chars().filter(|x| x == c).count();
            count >= *min && count <= *max
        })
        .count()
}

pub fn part2(input: String) -> usize {
    let input = parse(&input).expect("Day 02 Part 2 -- Failed to Parse");

    input
        .into_iter()
        .filter(|(first, second, c, passwd)| {
            let first = passwd.chars().nth(*first - 1).unwrap() == *c;
            let second = passwd.chars().nth(*second - 1).unwrap() == *c;

            first ^ second
        })
        .count()
}
