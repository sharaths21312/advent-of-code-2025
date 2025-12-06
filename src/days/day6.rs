use std::{fmt::format, time::Instant};
use crate::utils::{solution::{FullSolution, Solution}, stringfuncs::StrExt};

// type Input = Vec<i64>;

#[derive(Debug)]
struct Input<'a> {
    numbers: Vec<Vec<i64>>,
    ops: Vec<&'a str>
}

fn parse_input(input: &str) -> Input<'_> {
    let numbers: Vec<Vec<i64>> =  input.clean_split("\n")
        .into_iter()
        .filter(|line| !line.starts_with('+') && !line.starts_with('*'))
        .map(|line| {
            line.clean_split(" ").into_iter()
                .map(|num| num.parse().unwrap())
                .collect()
        }).collect();

    let ops = input.lines().filter(|line| line.starts_with('+') || line.starts_with('*')).next().unwrap().clean_split(" ");

    Input { numbers, ops }
}

fn part1(input: &Input) -> Solution {
    let mut solution = 0;

    for col in 0..input.numbers[0].len() {
        let op = input.ops[col];
        let mut subtotal = match op {
            "*" => 1,
            "+" => 0,
            _ => panic!()
        };
        for row in 0..input.numbers.len() {
            subtotal = match op {
                "*" => subtotal * input.numbers[row][col],
                "+" => subtotal + input.numbers[row][col],
                _ => panic!()
            }
        }
        solution += subtotal;
    }

    Solution::I64(solution)
}

fn part2(input: &str) -> Solution {
    let ops = whitespace_aware_split(input.lines().filter(|line| line.starts_with('+') || line.starts_with('*')).next().unwrap());
    let numbers: Vec<Vec<_>> = input.trim().lines().filter(|line| !(line.starts_with('+') || line.starts_with('*'))).map(|line| line.chars().collect()).collect();
    let mut current_idx = 0;
    let mut total = 0;
    for (op, gap) in ops {
        let mut subtotal = match op {
            '*' => 1,
            '+' => 0,
            _ => panic!()
        };
        for idx in current_idx..=(current_idx+gap) {
            let mut nums = vec![];
            let idx = idx as usize;
            let num = numbers.iter().fold(String::new(), |c, a| {
                format!("{}{}", c, a[idx])
            });
            let num = num.trim();
            if num != "" {
                nums.push(num);
                let num: i64 = num.parse().unwrap();
                subtotal = match op {
                    '*' => subtotal * num,
                    '+' => subtotal + num,
                    _ => panic!()
                }
            }
        }
        total += subtotal;
        current_idx += gap + 1;
    }
    Solution::I64(total)
}

fn whitespace_aware_split(input: &str) -> Vec<(char, i64)> {
    let mut output = vec![];
    let mut last_char = input.chars().next().unwrap();
    let mut space = 0;
    for char in input.chars().skip(1) {
        if char.is_whitespace() {
            space += 1;
        } else {
            output.push((last_char, space));
            space = 0;
            last_char = char;
        }
    }
    output.push((last_char, space));
    output
}

pub fn solve(input_str: &str) -> FullSolution {
    let start = Instant::now();
    let input = parse_input(input_str);
    let part1_s = part1(&input);
    let time1 = start.elapsed();
    let start = Instant::now();
    let part2_s = part2(&input_str);
    let time2 = start.elapsed();
    FullSolution { part1: part1_s, part2: part2_s, time1, time2 }
}