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
}

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => aoc!(01),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
