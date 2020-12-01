use std::collections::HashSet;

pub fn part1(input: String) -> u64 {
    let nums: HashSet<u64> = input.lines().map(|x| x.parse().unwrap()).collect();
    for x in nums.iter() {
        let y = 2020 - x;
        if nums.contains(&y) {
            assert_eq!(x + y, 2020);
            return x * y;
        }
    }
    0
}

pub fn part2(input: String) -> u64 {
    let nums: HashSet<u64> = input.lines().map(|x| x.parse().unwrap()).collect();
    for x in nums.iter() {
        for y in nums.iter() {
            if x + y > 2020 {
                continue;
            }
            let z = 2020 - x - y;
            if nums.contains(&z) {
                assert_eq!(x + y + z, 2020);
                return x * y * z;
            }
        }
    }
    0
}
