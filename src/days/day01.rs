use super::DayResult;
use crate::utils::bench::time_execution;
use std::{collections::HashMap, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/01.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (mut left_list, mut right_list, right_map) = parsed.result;
    let part1 = time_execution(|| part1(&mut left_list, &mut right_list));
    let part2 = time_execution(|| part2(&left_list, &right_map));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>, HashMap<usize, usize>) {
    let mut left_list = vec![];
    let mut right_list = vec![];
    let mut right_map = HashMap::new();

    for line in input.lines() {
        let (left, right) = line
            .split_once("   ")
            .expect("Exactly two numbers should be present on each line separated by three spaces");

        let left = left
            .parse::<usize>()
            .expect("Left half should be a positive integer");
        let right = right
            .parse::<usize>()
            .expect("Right half should be a positive integer");

        left_list.push(left);
        right_list.push(right);
        right_map
            .entry(right)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    (left_list, right_list, right_map)
}

fn part1(left_list: &mut [usize], right_list: &mut [usize]) -> String {
    left_list.sort_unstable();
    right_list.sort_unstable();
    let total_distance: usize = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum();
    format!("{total_distance}")
}

fn part2(left_list: &[usize], right_map: &HashMap<usize, usize>) -> String {
    let similarity_score: usize = left_list
        .iter()
        .map(|left| left * right_map.get(left).unwrap_or(&0))
        .sum();
    format!("{similarity_score}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_parse() {
        let (left_list, right_list, right_map) = parse(INPUT);

        assert_eq!(left_list, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right_list, vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(right_map, HashMap::from([(3, 3), (4, 1), (5, 1), (9, 1)]));
    }

    #[test]
    fn test_part1() {
        let (mut left_list, mut right_list, _) = parse(INPUT);
        let total_distance = part1(&mut left_list, &mut right_list);
        assert_eq!(total_distance, "11");
    }

    #[test]
    fn test_part2() {
        let (left_list, _, right_map) = parse(INPUT);
        let similarity_score = part2(&left_list, &right_map);
        assert_eq!(similarity_score, "31");
    }
}
