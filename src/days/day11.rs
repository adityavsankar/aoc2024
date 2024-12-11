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
    fn count(n: usize, blinks: u8, cache: &mut HashMap<(usize, u8), usize>) -> usize {
        if blinks == 0 {
            return 1;
        }
        if let Some(&result) = cache.get(&(n, blinks)) {
            return result;
        }

        let result = if n == 0 {
            count(1, blinks - 1, cache)
        } else {
            let len = n.ilog10() + 1;
            if len % 2 == 0 {
                let mid_idx = 10_usize.pow(len / 2);
                let left = n / mid_idx;
                let right = n % mid_idx;
                count(left, blinks - 1, cache) + count(right, blinks - 1, cache)
            } else {
                count(n * 2024, blinks - 1, cache)
            }
        };

        cache.insert((n, blinks), result);
        result
    }
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| count(stone, blinks, &mut cache))
        .sum()
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
        let result = part1(&stones);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part2() {
        let stones = parse(INPUT);
        let result = part2(&stones);
        assert_eq!(result, 65601038650482);
    }
}
