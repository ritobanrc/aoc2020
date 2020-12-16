use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug)]
struct Rule<'a> {
    property: &'a str,
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

fn parse_nearby_and_rules(input: &str) -> (Vec<Vec<u32>>, Vec<Rule>) {
    let nearby_tickets: Vec<Vec<u32>> = input
        .split("\n\n")
        .filter(|line| line.starts_with("nearby tickets:"))
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    let rules: Vec<Rule> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (property, rule) = line.split(':').collect_tuple().unwrap();
            let (first, second) = rule.split("or").collect_tuple().unwrap();

            let (first_min, first_max) = first
                .trim()
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let (snd_min, snd_max) = second
                .trim()
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Rule {
                property,
                first: RangeInclusive::new(first_min, first_max),
                second: RangeInclusive::new(snd_min, snd_max),
            }
        })
        .collect();

    (nearby_tickets, rules)
}

pub fn part1(input: String) -> u32 {
    let (nearby_tickets, rules) = parse_nearby_and_rules(&input);

    nearby_tickets
        .iter()
        .flatten()
        .filter(|value| {
            !rules
                .iter()
                .any(|rule| rule.first.contains(value) || rule.second.contains(value))
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    let (nearby_tickets, rules) = parse_nearby_and_rules(&input);

    let valid_tickets: Vec<_> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            !ticket.is_empty()
                && ticket.iter().all(|value| {
                    rules
                        .iter()
                        .any(|rule| rule.first.contains(value) || rule.second.contains(value))
                })
        })
        .collect();

    let your_ticket: Vec<u64> = input
        .split("\n\n")
        .filter(|line| line.starts_with("your ticket:"))
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut possibilites = vec![vec![true; valid_tickets[0].len()]; rules.len()];

    for ticket in valid_tickets.iter() {
        for (i, value) in ticket.iter().enumerate() {
            for (idx, property) in rules.iter().enumerate() {
                if possibilites[idx][i]
                    && !property.first.contains(value)
                    && !property.second.contains(value)
                {
                    possibilites[idx][i] = false;
                }
            }
        }
    }

    if cfg!(feature = "visualize") {
        for (n, property) in rules.iter().enumerate() {
            for i in 0..20 {
                if possibilites[n][i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("        {:?}", property.property);
            println!();
        }
    }

    let sorted_possibilities: Vec<_> = possibilites
        .iter()
        .zip(rules)
        .sorted_by_key(|(options, _name)| options.iter().filter(|x| **x == true).count())
        .collect();

    if cfg!(feature = "visualize") {
        println!();

        for (options, rule) in sorted_possibilities.iter() {
            for i in 0..20 {
                if options[i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("        {:?}", rule.property);
            println!();
        }
    }

    let mut taken = vec![false; valid_tickets[0].len()];
    let mut product = 1;

    for (options, rule) in sorted_possibilities.iter() {
        let location = options
            .into_iter()
            .zip(&taken)
            .map(|(a, b)| a ^ b)
            .find_position(|x| *x == true)
            .unwrap()
            .0;

        taken[location] = true;

        if rule.property.starts_with("departure") {
            product *= your_ticket[location];
        }
    }

    product
}

#[test]
fn day16_test() {
    let input = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
        .trim();

    assert_eq!(71, part1(input.to_string()));

    let input = "
class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
        .trim();

    dbg!(part2(input.to_string()));
}
