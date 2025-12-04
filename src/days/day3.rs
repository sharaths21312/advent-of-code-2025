use std::time::Instant;
use crate::utils::solution::{Solution, FullSolution};

type Input = Vec<Vec<i64>>;

fn parse_input<'a>(input: &str) -> Input {
    let output = input.trim().split('\n').map(|s| s.chars().map(|c| c.to_digit(10).unwrap().into()).collect()).collect();
    output
}

fn part1(input: &Input) -> Solution {
    let mut joltage = 0;
    for line in input {
        let mut ones = 0;
        let mut tens = 0;
        for &next in line {
            if ones > tens {
                tens = ones;
                ones = next;
            } else if next > ones {
                ones = next;
            }
        }
        joltage += tens * 10 + ones;
    }
    Solution::I64(joltage)
}

fn part2(input: &Input) -> Solution {
    
    Solution::I64(input
        .into_iter()
        .map(|line| {
            let mut digits = [0; 13];
            for &digit in line {
                digits[12] = digit;
                for i in 0..12 {
                    if digits[i] < digits[i + 1] {
                        digits.copy_within(i + 1.., i);
                        break;
                        // If a digit is lower at any point, delete it and shift the rest ahead
                    }
                }
            }
            return digits[..12].iter().fold(0, |left, &right| 10 * left + right);
        })
        .sum())
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