use super::DayResult;
use crate::utils::bench::time_execution;
use std::{collections::HashMap, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/11.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let stones = parsed.result;
    let part1 = time_execution(|| part1(&stones));
    let part2 = time_execution(|| part2(&stones));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(' ')
        .map(|stone| {
            stone
                .parse()
                .expect("Input should contain positive integers")
        })
        .collect()
}

fn solve(stones: &[usize], blinks: u8) -> usize {
    let mut stone_counts = HashMap::with_capacity(6000);
    for &stone in stones {
        *stone_counts.entry(stone).or_default() += 1;
    }

    for _ in 0..blinks {
        let mut new_counts = HashMap::with_capacity(6000);
        for (&stone, &count) in stone_counts.iter() {
            if stone == 0 {
                *new_counts.entry(1).or_default() += count;
            } else {
                let len = stone.ilog10() + 1;
                if len % 2 == 0 {
                    let mid_idx = 10_usize.pow(len / 2);
                    let left = stone / mid_idx;
                    let right = stone % mid_idx;
                    *new_counts.entry(left).or_default() += count;
                    *new_counts.entry(right).or_default() += count;
                } else {
                    *new_counts.entry(stone * 2024).or_default() += count;
                }
            }
        }
        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}

pub fn part1(stones: &[usize]) -> usize {
    solve(stones, 25)
}

pub fn part2(stones: &[usize]) -> usize {
    solve(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "125 17";

    #[test]
    fn test_parse() {
        let stones = parse(INPUT);
        assert_eq!(stones.len(), 2);
        assert_eq!(stones[0], 125);
        assert_eq!(stones[1], 17);
    }

    #[test]
    fn test_part1() {
        let stones = parse(INPUT);
        let stone_count = part1(&stones);
        assert_eq!(stone_count, 55312);
    }

    #[test]
    fn test_part2() {
        let stones = parse(INPUT);
        let stone_count = part2(&stones);
        assert_eq!(stone_count, 65601038650482);
    }
}
