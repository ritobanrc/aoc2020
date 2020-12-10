use std::collections::HashMap;

use itertools::Itertools;

pub fn part1(input: String) -> usize {
    let mut adapters: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort_unstable();

    let mut ones = adapters
        .iter()
        .tuple_windows()
        .filter(|(a, b)| **b - **a == 1)
        .count();
    let mut threes = adapters
        .iter()
        .tuple_windows()
        .filter(|(a, b)| **b - **a == 3)
        .count();

    if adapters[0] == 1 {
        ones += 1;
    } else if adapters[0] == 3 {
        threes += 1
    }

    threes += 1; // from the final adapter to the device
    ones * threes
}

type Graph<'a> = HashMap<u32, &'a [u32]>;

pub fn part2(input: String) -> usize {
    let mut adapters: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();

    let start = adapters[0];
    let end = adapters[adapters.len() - 1];

    let graph: HashMap<u32, &[u32]> = adapters
        .iter()
        .enumerate()
        .filter_map(|(i, &a)| {
            // This is the last one that we could jump to
            let last = adapters[i..]
                .iter()
                .enumerate()
                .take_while(|(_i, x)| **x - a <= 3)
                .map(|(i, _x)| i)
                .last()
                .unwrap();

            Some((a, &adapters[i + 1..i + last + 1]))
        })
        .collect();

    let mut memo = HashMap::new();

    fn count_possibilites(
        last: u32,
        device: u32,
        graph: &Graph,
        memo: &mut HashMap<u32, usize>,
    ) -> usize {
        if let Some(result) = memo.get(&last) {
            return *result;
        }

        if last == device {
            return 1; // we're there!
        }
        let total = graph[&last]
            .iter()
            .map(|next| count_possibilites(*next, device, graph, memo))
            .sum();

        memo.insert(last, total);

        total
    }

    count_possibilites(start, end, &graph, &mut memo)
}

#[test]
fn day10_test() {
    let input = "
16
10
15
5
1
11
7
19
6
12
4
        "
    .trim();

    assert_eq!(35, part1(input.to_owned()));
    //assert_eq!(8, part2(input.to_owned()));

    let input2 = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
        "
    .trim();

    assert_eq!(19208, part2(input2.to_owned()));
}
