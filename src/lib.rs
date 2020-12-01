// Days
pub mod day01;

pub fn noop(_inp: String) -> Box<dyn std::fmt::Debug> {
    Box::new(())
}

pub type DayFn = fn(String) -> Box<dyn std::fmt::Debug>;

pub enum Part {
    Part1,
    Part2,
}

macro_rules! aoc {
    ($day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::part1(input)), |input| Box::new([<day $day>]::part2(input)))
        }
    };
    (same:$day: expr) => {
        paste::item! {
            (|input| Box::new([<day $day>]::solutions(input, Part::Part1)), |input| Box::new([<day $day>]::solutions(input, Part::Part2)))
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
        _ => {
            eprintln!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
