pub fn part1(input: String) -> u64 {
    let earliest_time: u64 = input.lines().next().unwrap().parse().unwrap();
    let busses: Vec<u64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse().ok())
        .collect();

    for time in earliest_time.. {
        for bus in busses.iter() {
            if time % bus == 0 {
                return (time - earliest_time) * bus;
            }
        }
    }
    0
}

pub fn part2(input: String) -> isize {
    let busses: Vec<(isize, isize)> = input
        .lines()
        .last()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, c)| {
            let m = c.parse().ok()?;
            let i = m - i as isize;
            Some((i, m))
        })
        .collect();

    //dbg!(busses);
    //     i      c
    // t = 0 (mod 7)
    // t = -1 (mod 13)
    // t = -4 (mod 59)
    // ...
    chinese_remainder_theorem(&busses)
}

fn chinese_remainder_theorem(nums: &[(isize, isize)]) -> isize {
    let prod: isize = nums.iter().map(|(_, c)| c).product();

    nums.iter()
        .map(|(ai, ni)| {
            let bi = prod / ni;
            let invmod = invmod(bi, *ni);
            ai * bi * invmod
        })
        .sum::<isize>()
        % prod
}

fn extended_euclid(mut x: isize, mut y: isize) -> (isize, isize) {
    let (mut x0, mut x1, mut y0, mut y1) = (1, 0, 0, 1);

    while y > 0 {
        let q = x / y;
        let r = x % y;

        x = y;
        y = r;

        let next_x1 = x0 - q * x1;
        x0 = x1;
        x1 = next_x1;

        let next_y1 = y0 - q * y1;
        y0 = y1;
        y1 = next_y1;
    }

    (x0, y0)
}

fn invmod(a: isize, m: isize) -> isize {
    let (x, _y) = extended_euclid(a, m);

    let result = x % m;
    if result < 0 {
        result + m
    } else {
        result
    }
}

#[test]
fn day13_test() {
    let input = "
939
7,13,x,x,59,x,31,19"
        .trim();

    assert_eq!(295, part1(input.to_owned()));

    assert_eq!(39, chinese_remainder_theorem(&[(0, 3), (3, 4), (4, 5)]));

    assert_eq!(3417, part2("17,x,13,19".to_string()));
}
