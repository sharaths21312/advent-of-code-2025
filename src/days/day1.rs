use std::time::Instant;

use crate::utils::{point::Point, solution::{Solution, SolutionData}};
use crate::utils::stringfuncs::StrExt;

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    let lines: Vec<_> = input.split('\n').filter(|l| *l != "").collect();
    let output: Vec<i64> = lines.iter().map(|line| {
        if line.starts_with('L') {
            return -line.lstrip_parse("L")
        } else {
            return line.lstrip_parse("R")
        }
    }).collect();
    output
}

fn part1(input: &Input) -> Solution {
    let mut pos = 50;
    let mut count = 0;
    for &step in input {
        pos += step;
        if pos % 100 == 0 {
            count += 1;
        }
    }
    Solution::I64(count)
}

fn part2(input: &Input) -> Solution {
    let mut pos = 50;
    let mut count = 0;
    for &step in input {
        pos += step;
        if pos <= 0 && pos != step {
            count += 1;
        }
        count += pos.abs()/100;
        pos = pos.rem_euclid(100);
    }
    Solution::I64(count)
}



pub fn solve(input: &str) -> SolutionData {
    let start = Instant::now();
    let input = parse_input(input);
    let part1_s = part1(&input);
    let time1 = start.elapsed();
    let start = Instant::now();
    let part2_s = part2(&input);
    let time2 = start.elapsed();
    SolutionData { part1: part1_s, part2: part2_s, time1, time2 }
}