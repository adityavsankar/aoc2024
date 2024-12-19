use super::DayResult;
use crate::utils::bench::time_execution;
use rayon::prelude::*;
use std::{collections::HashSet, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/19.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (towels, designs) = parsed.result;
    let part1 = time_execution(|| part1(&towels, &designs));
    let part2 = time_execution(|| part2(&towels, &designs));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let input = input.replace('\r', "");
    let (towels, designs) = input
        .split_once("\n\n")
        .expect("Towels and Designs should be separated by a blank line");
    let towels = towels.split(", ").map(String::from).collect();
    let designs = designs.lines().map(String::from).collect();
    (towels, designs)
}

fn solve(design: &str, towels: &HashSet<String>) -> u64 {
    let n = design.len();
    let k = towels
        .iter()
        .map(String::len)
        .max()
        .expect("There should be at least one towel");
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..n {
        if dp[i] == 0 {
            continue;
        }
        for j in i + 1..=n.min(i + k) {
            if towels.contains(&design[i..j]) {
                dp[j] += dp[i];
            }
        }
    }

    dp[n]
}

fn part1(towels: &HashSet<String>, designs: &[String]) -> String {
    let possible_design_count = designs
        .par_iter()
        .filter(|design| solve(design, towels) != 0)
        .count();
    format!("{possible_design_count}")
}

fn part2(towels: &HashSet<String>, designs: &[String]) -> String {
    let total_design_arrangements: u64 =
        designs.par_iter().map(|design| solve(design, towels)).sum();
    format!("{total_design_arrangements}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;

    const INPUT: &'static str =
        "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";

    #[test]
    fn test_parse() {
        let (towels, designs) = parse(INPUT);

        assert_eq!(
            towels,
            HashSet::from([
                "r".into(),
                "wr".into(),
                "b".into(),
                "g".into(),
                "bwu".into(),
                "rb".into(),
                "gb".into(),
                "br".into()
            ])
        );

        assert_eq!(designs.len(), 8);
        assert_eq!(
            designs,
            vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]
        )
    }

    #[test]
    fn test_part1() {
        let (towels, designs) = parse(INPUT);
        let possible_design_count = part1(&towels, &designs);
        assert_eq!(possible_design_count, "6");
    }

    #[test]
    fn test_part2() {
        let (towels, designs) = parse(INPUT);
        let total_design_arrangements = part2(&towels, &designs);
        assert_eq!(total_design_arrangements, "16");
    }
}
