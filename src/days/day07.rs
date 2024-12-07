use super::DayResult;
use crate::utils::time_execution;
use rayon::prelude::*;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/input07.txt").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let calibration_equations = parsed.result;
    let part1 = time_execution(|| part1(&calibration_equations));
    let part2 = time_execution(|| part2(&calibration_equations));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line
                .split_once(": ")
                .expect("Input should contain test value and numbers separated by ': '");
            let test_value = test_value
                .parse()
                .expect("Test value should be a positive integer");
            let numbers = numbers
                .split(' ')
                .map(|num| num.parse().expect("Numbers should be positive integers"))
                .collect();
            (test_value, numbers)
        })
        .collect()
}

pub fn part1(calibration_equations: &[(usize, Vec<usize>)]) -> usize {
    solve(calibration_equations, false)
}

pub fn part2(calibration_equations: &[(usize, Vec<usize>)]) -> usize {
    solve(calibration_equations, true)
}

fn solve(calibration_equations: &[(usize, Vec<usize>)], is_part2: bool) -> usize {
    fn is_possible(acc: usize, arr: &[usize], i: usize, target: usize, is_part2: bool) -> bool {
        if i >= arr.len() {
            acc == target
        } else if acc > target {
            false
        } else {
            is_possible(acc + arr[i], arr, i + 1, target, is_part2)
                || is_possible(acc * arr[i], arr, i + 1, target, is_part2)
                || if is_part2 {
                    is_possible(
                        acc * 10_usize.pow(arr[i].ilog10() + 1) + arr[i],
                        arr,
                        i + 1,
                        target,
                        is_part2,
                    )
                } else {
                    false
                }
        }
    }

    calibration_equations
        .par_iter()
        .filter_map(|(test_value, numbers)| {
            if is_possible(numbers[0], numbers, 1, *test_value, is_part2) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn test_parse() {
        let calibration_equations = parse(INPUT);
        assert_eq!(calibration_equations.len(), 9);
        assert_eq!(calibration_equations[0], (190, vec![10, 19]));
        assert_eq!(calibration_equations[1], (3267, vec![81, 40, 27]));
        assert_eq!(calibration_equations[2], (83, vec![17, 5]));
        assert_eq!(calibration_equations[3], (156, vec![15, 6]));
        assert_eq!(calibration_equations[4], (7290, vec![6, 8, 6, 15]));
        assert_eq!(calibration_equations[5], (161011, vec![16, 10, 13]));
        assert_eq!(calibration_equations[6], (192, vec![17, 8, 14]));
        assert_eq!(calibration_equations[7], (21037, vec![9, 7, 18, 13]));
        assert_eq!(calibration_equations[8], (292, vec![11, 6, 16, 20]));
    }

    #[test]
    fn test_part1() {
        let calibration_equations = parse(INPUT);
        let result = part1(&calibration_equations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part2() {
        let calibration_equations = parse(INPUT);
        let result = part2(&calibration_equations);
        assert_eq!(result, 11387);
    }
}
