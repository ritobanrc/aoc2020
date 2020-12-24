use std::collections::{HashMap, HashSet};

use euclid::{default::Vector3D, vec3};
use itertools::iproduct;

pub fn part1(input: String) -> usize {
    // Thanks to u/SuperSmurfen for coming up with this idea, will definitely use again in the future
    let mut active_cells: HashSet<Vector3D<_>> = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, active) in row.chars().enumerate() {
            match active {
                '#' => active_cells.insert(vec3(x as i64, y as i64, 0)),
                _ => continue,
            };
        }
    }

    println!("Init: Active Cells: {:?}", active_cells);

    for _cycle in 0..6 {
        let mut neighbors = HashMap::new();
        for active in active_cells.iter() {
            for (dx, dy, dz) in iproduct!(-1..=1, -1..=1, -1..=1) {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }

                neighbors
                    .entry(vec3(active.x + dx, active.y + dy, active.z + dz))
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }
        active_cells = neighbors
            .into_iter()
            .filter(|&(pos, count)| {
                if active_cells.contains(&pos) {
                    count == 2 || count == 3
                } else {
                    count == 3
                }
            })
            .map(|(pos, _count)| pos)
            .collect();
    }

    active_cells.len()
}

pub fn part2(_input: String) {}

#[test]
fn day17_test() {
    let input = "
.#.
..#
###"
    .trim();

    assert_eq!(112, part1(input.to_string()));
}
