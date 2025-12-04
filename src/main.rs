#![allow(dead_code, unused_imports)]
pub mod utils;
pub mod days;

use std::{env::args, fmt::format, fs, path::{self, Path, absolute}, time::Duration};
use utils::solution::Solution;
use days::*;

use crate::utils::solution::FullSolution;

fn main() {
    let args: Vec<String> = args().collect();
    let filename: String;
    let day = &args[1];
    if args.len() > 2 {
        filename = format!("inputs/day{day}_test.txt")
    } else {
        filename = format!("inputs/day{day}.txt")
    }
    let paths = Path::new(&filename);
    if !fs::exists(paths).unwrap_or(false) {
        panic!("File not found")
    }
    let input = fs::read_to_string(paths).unwrap();
    let solution = get_day(&day)(&input);
    let _ = println!("{}\n\n{}\n\nTime elapsed = {}, {}",
        solution.part1, solution.part2,
        time_fmt(&solution.time1), time_fmt(&solution.time2));
}

fn time_fmt(time: &Duration) -> String {
    if time.as_millis() < 10 {
        format!("{}μs", time.as_micros())
    } else {
        format!("{}ms", time.as_millis())
    }
}

fn get_day(day: &str) -> fn(&str) -> FullSolution {
    return match day {
        "1" => day1::solve,
        "2" => day2::solve,
        "3" => day3::solve,
        _ => unimplemented!()
    }
}
