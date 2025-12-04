use std::time::Instant;
use crate::utils::solution::{Solution, FullSolution};

type Input = Vec<i64>;

fn parse_input(input: &str) -> Input {
    let output: Vec<i64> = vec![];
    output
}

fn part1(input: &Input) -> Solution {
    
    Solution::I64(0)
}

fn part2(input: &Input) -> Solution {
    
    Solution::I64(0)
}



pub fn solve(input: &str) -> FullSolution {
    let start = Instant::now();
    let input = parse_input(input);
    let part1_s = part1(&input);
    let time1 = start.elapsed();
    let start = Instant::now();
    let part2_s = part2(&input);
    let time2 = start.elapsed();
    FullSolution { part1: part1_s, part2: part2_s, time1, time2 }
}