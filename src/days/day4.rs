use std::time::Instant;
use crate::utils::{grid::Grid, point::Point, solution::{FullSolution, Solution}};

type Input = Grid<char>;

fn parse_input(input: &str) -> Input {
    input.into()
}

fn part1(input: &Input) -> Solution {
    let grid_enumerator = input.enumerate();
    let mut count = 0;

    for (pos, _) in grid_enumerator.filter(|(_, chr)| **chr == '@') {
        let mut surround = 0;
        for y in (pos.y - 1)..=(pos.y + 1) {
            for x in (pos.x - 1)..=(pos.x + 1) {
                if input.get_or(&(x, y).into(), '.') == '@' {
                    surround += 1;
                }
            }
        }
        if surround - 1 < 4 {
            count += 1;
        }
    }
    Solution::I64(count)
}

fn part2(input: &Input) -> Solution {
    let mut input = input.clone();

    let mut count = 0;
    let mut last_run = 1;

    while last_run > 0 {
        last_run = 0;
        for x in 0..input.width() {
            for y in 0..input.height() {
                let pos: Point = (x as i64, y as i64).into();
                if input[&pos] == '.' {
                    continue;
                }

                let mut surround = 0;
                for y in (pos.y - 1)..=(pos.y + 1) {
                    for x in (pos.x - 1)..=(pos.x + 1) {
                        if input.get_or(&(x, y).into(), '.') == '@' {
                            surround += 1;
                        }
                    }
                }
                if surround - 1 < 4 {
                    count += 1;
                    last_run += 1;
                    input[&pos] = '.';
                }
            }
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