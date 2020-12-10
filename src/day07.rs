use std::collections::HashMap;

use itertools::Itertools;

type Color<'a> = (&'a str, &'a str);
type Rules<'a> = HashMap<Color<'a>, Vec<(u32, Color<'a>)>>;

fn parse(input: &str) -> Rules {
    input
        .lines()
        .filter_map(|line| {
            let (color, contains) = line.split(" bags contain ").collect_tuple()?;
            let color = color.split(' ').collect_tuple::<(_, _)>()?;
            let contains: Vec<_> = contains
                .split(", ")
                .filter_map(|desc| {
                    if desc == "no other bags." {
                        return None;
                    }
                    let (count, qualifier, color, _bags) = desc.split(' ').collect_tuple()?;
                    Some((count.parse::<u32>().ok()?, (qualifier, color)))
                })
                .collect();

            Some((color, contains))
        })
        .collect()
}

fn contains_gold<'a>(
    color: Color<'a>,
    rules: &Rules<'a>,
    memo: &mut HashMap<Color<'a>, bool>,
) -> bool {
    if let Some(result) = memo.get(&color) {
        return *result;
    }

    if color == ("shiny", "gold") {
        return true;
    }

    for inside in rules[&color].iter() {
        if contains_gold(inside.1, rules, memo) {
            memo.insert(color, true);
            return true;
        }
    }

    memo.insert(color, false);
    return false;
}

pub fn part1(input: String) -> usize {
    let rules = parse(&input);
    let mut memo = HashMap::new();
    rules
        .keys()
        .filter(|&&color| color != ("shiny", "gold") && contains_gold(color, &rules, &mut memo))
        .count()
}

fn count_inside<'a>(color: Color<'a>, rules: &Rules<'a>) -> u32 {
    rules[&color]
        .iter()
        .map(|(count, color)| count * count_inside(*color, rules))
        .sum::<u32>()
        + 1
}

pub fn part2(input: String) -> u32 {
    let rules = parse(&input);
    count_inside(("shiny", "gold"), &rules) - 1
}

#[test]
fn day07_test() {
    let input = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"
    .trim();

    assert_eq!(4, part1(input.to_owned()));
    dbg!(part2(input.to_owned()));
}
