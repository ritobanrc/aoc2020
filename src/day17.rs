use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use euclid::{default::Vector3D, vec3};
use itertools::iproduct;

struct Coord(i64, i64);

impl From<Coord> for Vector3D<i64> {
    fn from(c: Coord) -> Self {
        vec3(c.0, c.1, 0)
    }
}

fn parse_grid<V: From<Coord> + std::hash::Hash + Eq>(input: &str) -> HashSet<V> {
    let mut grid = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, active) in row.chars().enumerate() {
            match active {
                '#' => grid.insert(Coord(x as i64, y as i64).into()),
                _ => continue,
            };
        }
    }
    grid
}

fn solutions<V>(input: String, deltas: impl Iterator<Item = V> + Clone) -> usize
where
    V: From<Coord> + std::hash::Hash + Eq + Add<V, Output = V> + Copy,
{
    // Thanks to u/SuperSmurfen for coming up with this idea, will definitely use again in the future
    let mut active_cells: HashSet<V> = parse_grid(&input);

    for _cycle in 0..6 {
        let mut neighbors = HashMap::new();
        for active in active_cells.iter() {
            for delta in deltas.clone() {
                neighbors
                    .entry(*active + delta)
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

pub fn part1(input: String) -> usize {
    solutions(
        input,
        iproduct!(-1..=1, -1..=1, -1..=1)
            .map(Vector3D::from)
            .filter(|&v| v != Vector3D::zero()),
    )
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Vector4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl From<Coord> for Vector4D {
    fn from(c: Coord) -> Self {
        Vector4D {
            x: c.0,
            y: c.1,
            z: 0,
            w: 0,
        }
    }
}

impl From<(i64, i64, i64, i64)> for Vector4D {
    fn from(c: (i64, i64, i64, i64)) -> Self {
        Vector4D {
            x: c.0,
            y: c.1,
            z: c.2,
            w: c.3,
        }
    }
}

impl Add<Vector4D> for Vector4D {
    type Output = Vector4D;
    fn add(self, other: Vector4D) -> Self {
        Vector4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

pub fn part2(input: String) -> usize {
    solutions(
        input,
        iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .map(Vector4D::from)
            .filter(|&v| v != Vector4D::default()),
    )
}

#[test]
fn day17_test() {
    let input = "
.#.
..#
###"
    .trim();

    assert_eq!(112, part1(input.to_string()));
}
