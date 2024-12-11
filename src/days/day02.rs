use super::DayResult;
use crate::utils::bench::time_execution;
use itertools::Itertools;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/02.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let reports = parsed.result;
    let part1 = time_execution(|| part1(&reports));
    let part2 = time_execution(|| part2(reports));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    let lines = input.lines();
    lines
        .map(|line| {
            line.split(' ')
                .map(|num| {
                    num.parse::<usize>()
                        .expect("Input should only contain positive integers")
                })
                .collect()
        })
        .collect()
}

fn is_safe(report: &[usize]) -> bool {
    let monotonic = report.is_sorted() || report.is_sorted_by(|a, b| a >= b);
    let safe = report
        .windows(2)
        .all(|w| (1..=3).contains(&w[1].abs_diff(w[0])));
    monotonic && safe
}

pub fn part1(reports: &[Vec<usize>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

pub fn part2(reports: Vec<Vec<usize>>) -> usize {
    reports
        .into_iter()
        .map(|report| {
            let k = report.len() - 1;
            report.into_iter().combinations(k).any(|r| is_safe(&r))
        })
        .filter(|&is_safe| is_safe)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_parse() {
        let reports = parse(INPUT);
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(reports[1], vec![1, 2, 7, 8, 9]);
        assert_eq!(reports[2], vec![9, 7, 6, 2, 1]);
        assert_eq!(reports[3], vec![1, 3, 2, 4, 5]);
        assert_eq!(reports[4], vec![8, 6, 4, 4, 1]);
        assert_eq!(reports[5], vec![1, 3, 6, 7, 9]);
    }

    #[test]
    fn test_part1() {
        let reports = parse(INPUT);
        let result = part1(&reports);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let reports = parse(INPUT);
        let result = part2(reports);
        assert_eq!(result, 4);
    }
}
