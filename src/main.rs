// Credit to Repl.it Advent of Code Template
use std::env;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};

use aoc2020::{get_day, noop};

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn main() -> Result<()> {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }
    // Parse day as number
    day = day.trim().to_string();
    let day_num: u32 = day
        .parse()
        .map_err(|e| anyhow!("Invalid day number {:?}, Err: {:?}", day, e))?;

    // Read input file
    let cwd = env::current_dir().context("Failed to get current_dir")?;
    let filename = cwd.join("input").join(format!("day{:02}.txt", day_num));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(&filename)
        .context(format!("Error while reading input from {:?}", filename))?;

    println!("---------------------");

    // Get corresponding function
    let to_run = get_day(day_num);
    // Time it
    if to_run.0 != noop {
        println!("Day {} - Part 1: ", day_num);
        let part1_start = Instant::now();
        println!("    {:?}", to_run.0(input.clone()));
        let part1_dur = part1_start.elapsed();
        println!("    Time: {}", fmt_dur(part1_dur));
    }

    println!("---------------------");

    if to_run.1 != noop {
        println!("Day {} - Part 2: ", day_num);
        let part2_start = Instant::now();
        println!("    {:?}", to_run.1(input.clone()));
        let part2_dur = part2_start.elapsed();
        println!("    Time: {}", fmt_dur(part2_dur));
    }

    println!("---------------------");

    Ok(())
}
