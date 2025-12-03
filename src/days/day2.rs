use std::{fmt::{Display, format}, ops::{Range, RangeInclusive}, time::Instant};
use crate::utils::{solution::{Solution, SolutionData}, stringfuncs::StrExt};

type Input = Vec<(i64, i64)>;

fn parse_input(input: &str) -> Input {
    let output = input.trim().split(",")
         .into_iter()
         .map(|inp| {
             let nums: Vec<i64> = inp
                .trim()
                .split('-')
                .map(|x| x.parse().unwrap()).collect();
             (nums[0], nums[1])
         })
         .collect();
    output
}

fn part1(input: &Input) -> Solution {
    let mut output = 0;
    for (low, high) in input {
        for num in *low..=*high {
            let numstr: String = format!("{num}");
            if numstr.len() % 2 != 0 {
                continue;
            }
            let (fst, snd) = numstr.split_at(numstr.len()/2);
            if fst == snd {
                output += num;
            }
        }
    }
    Solution::I64(output)
}

fn part2(input: &Input) -> Solution {
    let mut output = 0;
    for (low, high) in input {
        for num in *low..=*high {
            let numstr: String = format!("{num}");
            for ndigits in 1..=numstr.len()/2 {
                if numstr.len() % ndigits != 0 {
                    continue;
                }

                
                let splits = numstr.interval_split(ndigits);
                
                if splits.iter().all(|&s| s == splits[0]) {
                    output += num;
                    break;
                }
            }
        }
    }
    Solution::I64(output)
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