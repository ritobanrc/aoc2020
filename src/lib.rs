// Days
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;

pub fn noop(_inp: String) -> Box<dyn std::fmt::Debug> {
    Box::new(())
}

pub type DayFn = fn(String) -> Box<dyn std::fmt::Debug>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

macro_rules! aoc {
    (with_enum:$day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::solutions(input, Part::Part1)), |input| Box::new([<day $day>]::solutions(input, Part::Part2)))
        }
    };
    (with_enum:$day: expr, $ans1: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::solutions(input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| Box::new([<day $day>]::solutions(input, Part::Part2)))
    };
    (with_enum:$day: expr, $ans1: expr, $ans2: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::solutions(input, Part::Part1); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans = [<day $day>]::solutions(input, Part::Part2); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        })
    };


    ($day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::part1(input)), |input| Box::new([<day $day>]::part2(input)))
        }
    };
    ($day: expr, $ans1: expr) => {
        paste::item! {
            (|input| {
                let ans =[<day $day>]::part1(input);
                assert_eq!(ans, $ans1);
                Box::new(ans)
            }, |input| Box::new([<day $day>]::part2(input)))
        }
    };
    ($day: expr, $ans1: expr, $ans2: expr) => {
        (|input| {
            paste::item! { let ans = [<day $day>]::part1(input); }
            assert_eq!(ans, $ans1);
            Box::new(ans)
        }, |input| {
            paste::item! { let ans =[<day $day>]::part2(input); }
            assert_eq!(ans, $ans2);
            Box::new(ans)
        })
    };
}

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => aoc!(01, 539851, 212481360),
        2 => aoc!(02, 524, 485),
        3 => aoc!(03, 156, 3521829480),
        4 => aoc!(04, 210, 131),
        5 => aoc!(05, 908, 619),
        6 => aoc!(06, 6534, 3402),
        7 => aoc!(07, 316, 11310),
        8 => aoc!(08, 2051, 2304),
        9 => aoc!(with_enum: 09, 1930745883, 268878261),
        10 => aoc!(10, 2482, 96717311574016),
        11 => aoc!(with_enum: 11, 2386, 2091),
        12 => aoc!(with_enum: 12, 2847, 29839),
        13 => aoc!(13, 207, 530015546283687),
        14 => aoc!(14, 3059488894985, 2900994392308),
        15 => aoc!(with_enum: 15, 240, 505),
        16 => aoc!(16, 28884, 1001849322119),
        17 => aoc!(17, 386),
        _ => {
            eprintln!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
