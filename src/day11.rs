use crate::Part;
use itertools::iproduct;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#',
            }
        )
    }
}

// This literally only exists for the Display impl
struct Map(Vec<Vec<Seat>>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for seat in row.iter() {
                write!(f, "{}", seat)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn count_neighbors<T: PartialEq>(map: &[Vec<T>], y: usize, x: usize, val: &T) -> usize {
    let mut total = 0;
    for (ny, nx) in iproduct!(y.saturating_sub(1)..=y + 1, x.saturating_sub(1)..=x + 1) {
        if ny == y && nx == x || ny >= map.len() || nx >= map[0].len() {
            continue;
        }

        //let ny = ny as usize;
        //let nx = nx as usize;

        if &map[ny][nx] == val {
            total += 1
        }
    }
    total
}

fn count_visible_neighbors(map: &[Vec<Seat>], y: usize, x: usize) -> usize {
    let mut total = 0;
    let on_grid = |y, x| y >= 0 && y < map.len() as isize && x >= 0 && x < map[0].len() as isize;

    for dy in -1..=1 {
        'dx: for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut current = (y as isize + dy, x as isize + dx);
            if !on_grid(current.0, current.1) {
                continue;
            }

            while map[current.0 as usize][current.1 as usize] == Seat::Floor {
                current = (current.0 + dy, current.1 + dx);
                if !on_grid(current.0, current.1) {
                    continue 'dx;
                }
            }

            match map[current.0 as usize][current.1 as usize] {
                Seat::Empty => continue,
                Seat::Occupied => total += 1,
                Seat::Floor => unreachable!(),
            }
        }
    }
    total
}

pub fn solutions(input: String, part: Part) -> usize {
    let mut map: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    '#' => Seat::Occupied,
                    'L' => Seat::Empty,
                    _ => panic!("Unrecognized char {:?}", c),
                })
                .collect()
        })
        .collect();

    let mut first = true;
    let mut next = map.clone();

    while first || map != next {
        map = next.clone();
        for (y, row) in map.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                let (neighbors, threshold) = match part {
                    Part::Part1 => (count_neighbors(&map, y, x, &Seat::Occupied), 4),
                    Part::Part2 => (count_visible_neighbors(&map, y, x), 5),
                };

                match (seat, neighbors) {
                    (Seat::Empty, 0) => next[y][x] = Seat::Occupied,
                    (Seat::Occupied, n) if n >= threshold => next[y][x] = Seat::Empty,
                    _ => {}
                }
            }
        }
        first = false;
    }

    map.into_iter()
        .flatten()
        .filter(|x| x == &Seat::Occupied)
        .count()
}

pub fn part2(_input: String) {}

#[test]
fn day11_test() {
    let input = "
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
        .trim();

    assert_eq!(37, solutions(input.to_owned(), Part::Part1));
    assert_eq!(26, solutions(input.to_owned(), Part::Part2));
}
