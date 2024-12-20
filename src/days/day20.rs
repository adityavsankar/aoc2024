use super::DayResult;
use crate::utils::{bench::time_execution, grid::Grid};
use rayon::prelude::*;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/20.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let racetrack = parsed.result;
    let part1 = time_execution(|| part1(&racetrack, 2, 100));
    let part2 = time_execution(|| part2(&racetrack, 20, 100));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Grid<u8> {
    Grid::from(input)
}

fn solve(racetrack: &Grid<u8>, cheat_time: usize, threshold: usize) -> usize {
    let (height, width) = (racetrack.height(), racetrack.width());
    let start = racetrack
        .position(b'S')
        .expect("Racetrack should have a start");
    let mut path = Vec::new();
    let mut seen = Grid::new(height, width, false);
    let mut program = start;

    loop {
        path.push(program);
        seen[program] = true;
        program = match program
            .orthogonal_neighbours()
            .into_iter()
            .find(|&next| racetrack[next] != b'#' && !seen[next])
        {
            None => break,
            Some(next) => next,
        }
    }

    path.par_iter()
        .enumerate()
        .map(|(i, &a)| {
            path.iter()
                .skip(i)
                .enumerate()
                .filter(|&(j, &b)| {
                    let taxicab_dist = a.taxicab_distance(b);
                    let saved = j - taxicab_dist + 1;
                    taxicab_dist <= cheat_time && saved >= threshold
                })
                .count()
        })
        .sum()
}

fn part1(racetrack: &Grid<u8>, cheat_time: usize, threshold: usize) -> String {
    let ans = solve(racetrack, cheat_time, threshold);
    format!("{ans}")
}

fn part2(racetrack: &Grid<u8>, cheat_time: usize, threshold: usize) -> String {
    let ans = solve(racetrack, cheat_time, threshold);
    format!("{ans}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const INPUT: &'static str = "###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############";

    #[test]
    fn test_parse() {
        let track = parse(INPUT);

        assert_eq!(track.height(), 15);
        assert_eq!(track.width(), 15);
        assert_eq!(str::from_utf8(track.row(0)).unwrap(), "###############");
        assert_eq!(str::from_utf8(track.row(1)).unwrap(), "#...#...#.....#");
        assert_eq!(str::from_utf8(track.row(2)).unwrap(), "#.#.#.#.#.###.#");
        assert_eq!(str::from_utf8(track.row(3)).unwrap(), "#S#...#.#.#...#");
        assert_eq!(str::from_utf8(track.row(4)).unwrap(), "#######.#.#.###");
        assert_eq!(str::from_utf8(track.row(5)).unwrap(), "#######.#.#...#");
        assert_eq!(str::from_utf8(track.row(6)).unwrap(), "#######.#.###.#");
        assert_eq!(str::from_utf8(track.row(7)).unwrap(), "###..E#...#...#");
        assert_eq!(str::from_utf8(track.row(8)).unwrap(), "###.#######.###");
        assert_eq!(str::from_utf8(track.row(9)).unwrap(), "#...###...#...#");
        assert_eq!(str::from_utf8(track.row(10)).unwrap(), "#.#####.#.###.#");
        assert_eq!(str::from_utf8(track.row(11)).unwrap(), "#.#...#.#.#...#");
        assert_eq!(str::from_utf8(track.row(12)).unwrap(), "#.#.#.#.#.#.###");
        assert_eq!(str::from_utf8(track.row(13)).unwrap(), "#...#...#...###");
        assert_eq!(str::from_utf8(track.row(14)).unwrap(), "###############");
    }

    #[test]
    fn test_part1() {
        let racetrack = parse(INPUT);
        let min_score = part1(&racetrack, 2, 20);
        assert_eq!(min_score, "5");
    }

    #[test]
    fn test_part2() {
        let racetrack = parse(INPUT);
        let best_seats = part2(&racetrack, 20, 50);
        assert_eq!(best_seats, "285");
    }
}
