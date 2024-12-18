use super::DayResult;
use crate::utils::bench::time_execution;
use regex::{Match, Regex};
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/03.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let instructions = parsed.result;
    let part1 = time_execution(|| part1(&instructions));
    let part2 = time_execution(|| part2(&instructions));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Vec<Match> {
    Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)")
        .unwrap()
        .find_iter(input)
        .collect()
}

fn mul(instruction: &str) -> u64 {
    let (a, b) = instruction[4..instruction.len() - 1]
        .split_once(',')
        .unwrap();
    let (a, b) = (
        a.parse::<u64>()
            .expect("Input should only contain positive integers"),
        b.parse::<u64>()
            .expect("Input should only contain positive integers"),
    );
    a * b
}

fn part1(instructions: &[Match]) -> String {
    let ans: u64 = instructions
        .iter()
        .map(|instruction| {
            let instruction = instruction.as_str();
            if instruction.len() > 7 {
                mul(instruction)
            } else {
                0
            }
        })
        .sum();
    format!("{ans}")
}

fn part2(instructions: &[Match]) -> String {
    let mut enabled = true;

    let ans: u64 = instructions
        .iter()
        .map(|instruction| {
            let instruction = instruction.as_str();
            match instruction.len() {
                4 => {
                    enabled = true;
                    0
                }
                7 => {
                    enabled = false;
                    0
                }
                _ if enabled => mul(instruction),
                _ => 0,
            }
        })
        .sum();
    format!("{ans}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse() {
        let matches = parse(INPUT2);

        assert_eq!(matches.len(), 6);
        assert_eq!(matches[0].as_str(), "mul(2,4)");
        assert_eq!(matches[1].as_str(), "don't()");
        assert_eq!(matches[2].as_str(), "mul(5,5)");
        assert_eq!(matches[3].as_str(), "mul(11,8)");
        assert_eq!(matches[4].as_str(), "do()");
        assert_eq!(matches[5].as_str(), "mul(8,5)");
    }

    #[test]
    fn test_part1() {
        let instructions = parse(INPUT1);
        let result = part1(&instructions);
        assert_eq!(result, "161");
    }

    #[test]
    fn test_part2() {
        let instructions = parse(INPUT2);
        let enabled_result = part2(&instructions);
        assert_eq!(enabled_result, "48");
    }
}
