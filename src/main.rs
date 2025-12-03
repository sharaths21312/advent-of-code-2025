#![allow(dead_code, unused_imports)]
pub mod utils;
pub mod days;

use std::{env::args, fs, path::{self, Path, absolute}};
use utils::solution::Solution;
use days::*;

use crate::utils::solution::SolutionData;

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
    let _ = println!("{}\n\n{}\n\nTime elapsed = {}ms, {}ms",
        solution.part1, solution.part2,
        solution.time1.as_millis(), solution.time2.as_millis());
}

fn get_day(day: &str) -> fn(&str) -> SolutionData {
    return match day {
        "1" => day1::solve,
        "2" => day2::solve,
        _ => unimplemented!()
    }
}
