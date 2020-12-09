use crate::Part;

pub fn solutions(input: String, part: Part) -> u64 {
    let input: Vec<_> = input.lines().map(|s| s.parse().unwrap()).collect();

    let part1 = input
        .windows(26)
        .find_map(|nums| {
            let to_check = nums[nums.len() - 1];
            for x in &nums[..25] {
                for y in &nums[..25] {
                    if x + y == to_check {
                        return None;
                    }
                }
            }

            return Some(to_check);
        })
        .unwrap();

    if part == Part::Part1 {
        return part1;
    }

    for len in 2.. {
        for start in 0..input.len() - len {
            let section = &input[start..start + len];
            if section.iter().sum::<u64>() == part1 {
                return section.iter().max().unwrap() + section.iter().min().unwrap();
            }
        }
    }

    0
}
