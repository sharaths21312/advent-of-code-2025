use std::time::Instant;
use crate::utils::{point::Pair, solution::{FullSolution, Solution}, stringfuncs::StrExt};

type Input = (Vec<Pair>, Vec<i64>);

fn parse_input(input: &str) -> Input {
    let mut ranges = vec![];
    let mut ids = vec![];

    let splits = input.clean_split("\n\n");
    for line in splits[0].clean_split("\n") {
        let elts = line.clean_split("-");
        ranges.push(Pair::new(
            elts[0].parse().unwrap(),
            elts[1].parse().unwrap()
        ));
    }
    
    for line in splits[1].clean_split("\n") {
        ids.push(line.parse().unwrap());
    }

    (ranges, ids)
}

fn part1(input: &Input) -> Solution {
    
    let mut ids = input.1.clone();
    let mut fresh: Vec<i64> = vec![];

    for &pair in (&input.0).into_iter() {
        let (low, high) = pair.into();
        let mut idx = 0;
        while idx < ids.len() {
            if ids[idx] <= high && ids[idx] >= low {
                fresh.push(ids[idx]);
                ids.swap_remove(idx);
            } else {
                idx += 1;
            }
        }
    }

    Solution::Usize(fresh.len())
}

fn part2(input: &Input) -> Solution {
    let mut input = input.0.clone();
    input.sort_by_key(|i| i.low);

    let (start, mut covered) = input[0].into();
    let mut count = covered - start + 1;
    for i in &input[1..] {
        if i.low > covered {
            count += i.high - i.low + 1;
            covered = i.high;
        } else if i.high <= covered {
            continue;
        } else {
            count += i.high - covered;
            covered = i.high;
        }
    }

    Solution::I64(count)
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