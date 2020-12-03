use euclid::{vec2, Point2D};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

fn parse_input(input: String) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Square::Open,
                    '#' => Square::Tree,
                    c => panic!("Unexpected token: {:?}", c),
                })
                .collect()
        })
        .collect()
}

fn count_slope(map: &[Vec<Square>], right: usize, down: usize) -> usize {
    let mut pos: Point2D<usize, usize> = Point2D::zero();
    let mut count = 0;

    while pos.y < map.len() {
        if map[pos.y][pos.x % map[0].len()] == Square::Tree {
            count += 1;
        }
        pos += vec2(right, down);
    }

    count
}

pub fn part1(input: String) -> usize {
    count_slope(&parse_input(input), 3, 1)
}

pub fn part2(input: String) -> usize {
    let map = parse_input(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| count_slope(&map, *right, *down))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test() {
        let input = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
        .trim();

        dbg!(part1(input.to_string()));
    }
}
