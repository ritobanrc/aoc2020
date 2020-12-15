use std::collections::{BTreeMap, HashMap};

pub fn part1(input: String) -> u64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut mem = HashMap::new();
    for line in input.lines() {
        if line.starts_with("mem") {
            let index: usize = line[4..line.find(']').unwrap()].parse().unwrap();
            let mut value: u64 = line[line.find('=').unwrap() + 2..].parse().unwrap();

            for (i, byte) in mask.bytes().rev().enumerate() {
                match byte {
                    b'X' => continue,
                    b'0' => value = value & !(1 << i),
                    b'1' => value = value | (1 << i),
                    _ => panic!("Unrecognized value in mask {:?}", byte),
                }
            }

            mem.insert(index, value);
        } else if line.starts_with("mask") {
            mask = &line[7..];
        }
    }

    mem.values().sum()
}

#[inline(always)]
fn set_0(index: usize, bit: usize) -> usize {
    index & !(1 << bit)
}

#[inline(always)]
fn set_1(index: usize, bit: usize) -> usize {
    index | (1 << bit)
}

pub fn part2(input: String) -> u64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut mem = BTreeMap::new();
    for line in input.lines() {
        if line.starts_with("mem") {
            let mut index: usize = line[4..line.find(']').unwrap()].parse().unwrap();
            let value: u64 = line[line.find('=').unwrap() + 2..].parse().unwrap();

            let mut floating_bits = Vec::new();

            for (i, byte) in mask.bytes().rev().enumerate() {
                match byte {
                    b'0' => continue,
                    b'X' => floating_bits.push(i),
                    b'1' => index = index | (1 << i),
                    _ => panic!("Unrecognized value in mask {:?}", byte),
                }
            }

            let orig_index = index;
            for n in 0..1 << floating_bits.len() {
                let mut index = orig_index;
                for (i, bit) in floating_bits.iter().enumerate() {
                    match (n & (1 << i)) >> i {
                        0 => index = set_0(index, *bit),
                        1 => index = set_1(index, *bit),
                        _ => unreachable!(),
                    }
                }

                mem.insert(index, value);
            }
        } else if line.starts_with("mask") {
            mask = &line[7..];
        }
    }

    mem.values().sum()
}
