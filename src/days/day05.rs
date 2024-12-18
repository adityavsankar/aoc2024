use super::DayResult;
use crate::utils::bench::time_execution;
use std::{cmp::Ordering, collections::HashSet, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/05.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (rules, updates) = parsed.result;
    let part1 = time_execution(|| part1(&rules, &updates));
    let part2 = time_execution(|| part2(&rules, updates));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    let input = input.replace('\r', "");
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Rules and updates should be separated by a line");
    let rules = rules
        .lines()
        .map(|rule| {
            let (a, b) = rule
                .split_once('|')
                .expect("Rules should be pipe delimited");
            (
                a.parse::<u8>()
                    .expect("Page numbers should be positive integers"),
                b.parse::<u8>()
                    .expect("Page numbers should be positive integers"),
            )
        })
        .collect();
    let updates = updates
        .lines()
        .map(|rule| {
            rule.split(',')
                .map(|page| {
                    page.parse::<u8>()
                        .expect("Page numbers should be positive integers")
                })
                .collect()
        })
        .collect();
    (rules, updates)
}

fn part1(rules: &HashSet<(u8, u8)>, updates: &[Vec<u8>]) -> String {
    let correct_order_total: u64 = updates
        .iter()
        .filter(|update| update.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|update| u64::from(update[update.len() / 2]))
        .sum();
    format!("{correct_order_total}")
}

fn part2(rules: &HashSet<(u8, u8)>, updates: Vec<Vec<u8>>) -> String {
    let incorrect_order_total: u64 = updates
        .into_iter()
        .filter(|update| !update.is_sorted_by(|&a, &b| rules.contains(&(a, b))))
        .map(|mut update| {
            update.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            u64::from(update[update.len() / 2])
        })
        .sum();
    format!("{incorrect_order_total}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\r\n\r\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    #[test]
    fn test_parse() {
        let (rules, updates) = parse(INPUT);
        assert_eq!(rules.len(), 21);
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(updates[1], vec![97, 61, 53, 29, 13]);
        assert_eq!(updates[2], vec![75, 29, 13]);
        assert_eq!(updates[3], vec![75, 97, 47, 61, 53]);
        assert_eq!(updates[4], vec![61, 13, 29]);
        assert_eq!(updates[5], vec![97, 13, 75, 29, 47]);
        assert_eq!(
            rules,
            HashSet::from([
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13)
            ])
        )
    }

    #[test]
    fn test_part1() {
        let (rules, updates) = parse(INPUT);
        let correct_order_total = part1(&rules, &updates);
        assert_eq!(correct_order_total, "143");
    }

    #[test]
    fn test_part2() {
        let (rules, updates) = parse(INPUT);
        let incorrect_order_total = part2(&rules, updates);
        assert_eq!(incorrect_order_total, "123");
    }
}
