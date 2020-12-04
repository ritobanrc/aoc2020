use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn count_valid(input: String, f: impl Fn(HashMap<&str, &str>) -> bool) -> usize {
    input
        .split("\n\n")
        .filter(|passport| {
            let batch: HashMap<_, _> = passport
                .split_whitespace()
                .map(|kv| {
                    let (key, value) = kv.split(':').collect_tuple().unwrap();
                    (key, value)
                })
                .collect();

            f(batch)
        })
        .count()
}

fn check_fields_present(batch: &HashMap<&str, &str>) -> bool {
    let all_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .copied()
        .collect();

    let keys = batch.keys().copied().collect();
    let diff: Vec<_> = all_keys.difference(&keys).collect();
    diff.len() == 0 || (diff.len() == 1 && diff[0] == &"cid")
}

pub fn part1(input: String) -> usize {
    count_valid(input, |batch| check_fields_present(&batch))
}

fn in_range<T: PartialOrd>(x: T, low: T, high: T) -> bool {
    x >= low && x <= high
}

pub fn part2(input: String) -> usize {
    count_valid(input, |batch| {
        if check_fields_present(&batch) {
            let byr = {
                let byr = match batch["byr"].parse() {
                    Ok(x) => x,
                    Err(_) => return false,
                };
                in_range(byr, 1920, 2002)
            };

            let iyr = {
                let iyr = match batch["iyr"].parse() {
                    Ok(x) => x,
                    Err(_) => return false,
                };
                in_range(iyr, 2010, 2020)
            };

            let eyr = {
                let eyr = match batch["eyr"].parse() {
                    Ok(x) => x,
                    Err(_) => return false,
                };
                in_range(eyr, 2020, 2030)
            };

            let hgt = {
                let hgt = batch["hgt"];
                let (hgt, unit) = hgt.split_at(hgt.len() - 2);
                let hgt = match hgt.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => return false,
                };
                match unit {
                    "cm" => hgt >= 150 && hgt <= 193,
                    "in" => hgt >= 59 && hgt <= 76,
                    _ => return false,
                }
            };

            let hcl = {
                let hcl = batch["hcl"];
                &hcl[0..1] == "#" && hcl[1..].len() == 6 && hcl[1..].chars().all(|c| c.is_digit(16))
            };

            let ecl = {
                let ecl = batch["ecl"];
                match ecl {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            };

            let pid = {
                let pid = batch["pid"];
                pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
            };

            byr && iyr && eyr && hgt && hcl && ecl && pid
        } else {
            false
        }
    })
}

#[test]
fn day4_test() {
    let input = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
        .trim();

    assert_eq!(2, part1(input.to_owned()));

    let input = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "
    .trim();

    assert_eq!(4, part2(input.to_owned()));

    let input = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
        .trim();

    assert_eq!(0, part2(input.to_owned()));
}
