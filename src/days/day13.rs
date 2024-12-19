use super::DayResult;
use crate::utils::{bench::time_execution, grid::Coord};
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/13.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let vals = parsed.result;
    let part1 = time_execution(|| part1(&vals));
    let part2 = time_execution(|| part2(&vals));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

type Pair = Coord;

fn parse(input: &str) -> Vec<Pair> {
    input
        .lines()
        .filter(|line| line.len() > 1)
        .map(|line| {
            let (_, right) = line
                .split_once(": ")
                .expect("Label and values should be colon separated");
            let (x, y) = right
                .split_once(", ")
                .expect("X and Y components should be comma separated");
            Pair::new(
                y[2..].parse().expect("Y component should be an integer"),
                x[2..].parse().expect("X component should be an integer"),
            )
        })
        .collect()
}

fn solve(vals: &[Pair], is_part2: bool) -> isize {
    vals.chunks(3)
        .filter_map(|q| {
            let a_button = q[0];
            let b_button = q[1];
            let prize = if is_part2 {
                q[2] + Pair::new(10_000_000_000_000, 10_000_000_000_000)
            } else {
                q[2]
            };
            let det = a_button.x() * b_button.y() - b_button.x() * a_button.y();
            let det1 = prize.x() * b_button.y() - b_button.x() * prize.y();
            if det1 % det != 0 {
                None
            } else {
                let a_presses = det1 / det;
                let b_presses = (prize.x() - a_presses * a_button.x()) / b_button.x();
                Some(3 * a_presses + b_presses)
            }
        })
        .sum()
}

fn part1(vals: &[Pair]) -> String {
    let total_min_tokens = solve(vals, false);
    format!("{total_min_tokens}")
}

fn part2(vals: &[Pair]) -> String {
    let total_min_tokens = solve(vals, true);
    format!("{total_min_tokens}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";

    #[test]
    fn test_parse() {
        let stones = parse(INPUT);
        assert_eq!(stones.len(), 12);
        assert_eq!(
            stones,
            vec![
                Pair::new(34, 94),
                Pair::new(67, 22),
                Pair::new(5400, 8400),
                Pair::new(66, 26),
                Pair::new(21, 67),
                Pair::new(12176, 12748),
                Pair::new(86, 17),
                Pair::new(37, 84),
                Pair::new(6450, 7870),
                Pair::new(23, 69),
                Pair::new(71, 27),
                Pair::new(10279, 18641)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let stones = parse(INPUT);
        let total_min_tokens = part1(&stones);
        assert_eq!(total_min_tokens, "480");
    }

    #[test]
    fn test_part2() {
        let stones = parse(INPUT);
        let total_min_tokens = part2(&stones);
        assert_eq!(total_min_tokens, "875318608908");
    }
}
