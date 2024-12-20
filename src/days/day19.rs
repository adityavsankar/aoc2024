use super::DayResult;
use crate::utils::bench::time_execution;
use std::fs;

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

#[derive(Debug, Clone, Default, PartialEq)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 5],
    is_end: bool,
}

impl TrieNode {
    fn byte_to_index(b: u8) -> usize {
        usize::from((b % 9) % 5)
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for b in word.bytes() {
            let idx = TrieNode::byte_to_index(b);
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }
        node.is_end = true;
    }

    fn search_prefix<F: FnMut(usize)>(&self, s: &str, start: usize, mut on_match: F) {
        let mut node = self;
        for (i, b) in s.bytes().skip(start).enumerate() {
            let idx = TrieNode::byte_to_index(b);
            if let Some(next_node) = &node.children[idx] {
                node = next_node;
                if node.is_end {
                    on_match(start + i + 1);
                }
            } else {
                break;
            }
        }
    }
}

impl<'a> FromIterator<&'a str> for TrieNode {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut root = TrieNode::default();
        for word in iter {
            root.insert(word);
        }
        root
    }
}

fn parse(input: &str) -> (TrieNode, Vec<String>) {
    let input = input.replace('\r', "");
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect();
    let designs = designs.lines().map(String::from).collect();
    (towels, designs)
}

fn count_arrangements(design: &str, towels: &TrieNode) -> u64 {
    let n = design.len();
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for i in 0..n {
        towels.search_prefix(design, i, |end| {
            dp[end] += dp[i];
        });
    }

    dp[n]
}

fn part1(towels: &TrieNode, designs: &[String]) -> String {
    let possible_design_count = designs
        .iter()
        .filter(|design| count_arrangements(design, towels) != 0)
        .count();
    format!("{possible_design_count}")
}

fn part2(towels: &TrieNode, designs: &[String]) -> String {
    let total_design_arrangements: u64 = designs
        .iter()
        .map(|design| count_arrangements(design, towels))
        .sum();
    format!("{total_design_arrangements}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";

    #[test]
    fn test_parse() {
        let (towels, designs) = parse(INPUT);

        assert_eq!(
            towels,
            ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]
                .into_iter()
                .collect()
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
