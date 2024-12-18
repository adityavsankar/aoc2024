use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Direction::*, Grid, Robot},
};
use rayon::prelude::*;
use std::{collections::HashSet, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/06.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let lab = parsed.result;
    let part1 = time_execution(|| part1(&lab));
    let part2 = time_execution(|| part2(&lab));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Grid<u8> {
    Grid::from(input)
}

fn patrol(lab: &Grid<u8>, start: Coord) -> Option<HashSet<Coord>> {
    let mut guard = Robot::new(start, North);
    let mut path = HashSet::with_capacity(5000);

    loop {
        let next_pos = guard.next_pos();
        if !lab.contains(next_pos) {
            break;
        }
        if lab[next_pos] == b'#' {
            guard.rotate(90);
        } else {
            guard.locomote();
        }
        if !path.insert(guard) {
            return None;
        }
    }

    Some(path.into_iter().map(|robot| robot.position()).collect())
}

fn part1(lab: &Grid<u8>) -> String {
    let start = lab.position(b'^').expect("Lab should have a guard");
    let tiles = patrol(lab, start);
    let distinct_position_count = tiles.expect("Input should not contain cycles").len();
    format!("{distinct_position_count}")
}

fn part2(lab: &Grid<u8>) -> String {
    let start = lab.position(b'^').expect("Lab should have a guard");
    let tiles = patrol(lab, start).expect("Input should not contain cycles");

    let potential_obstruction_count = tiles
        .into_par_iter()
        .filter(|&point| {
            let mut g = lab.to_owned();
            g[point] = b'#';
            patrol(&g, start).is_none()
        })
        .count();

    format!("{potential_obstruction_count}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const INPUT: &'static str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    #[test]
    fn test_parse() {
        let grid = parse(INPUT);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid.width(), 10);
        assert_eq!(str::from_utf8(grid.row(0)).unwrap(), "....#.....");
        assert_eq!(str::from_utf8(grid.row(1)).unwrap(), ".........#");
        assert_eq!(str::from_utf8(grid.row(2)).unwrap(), "..........");
        assert_eq!(str::from_utf8(grid.row(3)).unwrap(), "..#.......");
        assert_eq!(str::from_utf8(grid.row(4)).unwrap(), ".......#..");
        assert_eq!(str::from_utf8(grid.row(5)).unwrap(), "..........");
        assert_eq!(str::from_utf8(grid.row(6)).unwrap(), ".#..^.....");
        assert_eq!(str::from_utf8(grid.row(7)).unwrap(), "........#.");
        assert_eq!(str::from_utf8(grid.row(8)).unwrap(), "#.........");
        assert_eq!(str::from_utf8(grid.row(9)).unwrap(), "......#...");
    }

    #[test]
    fn test_part1() {
        let lab = parse(INPUT);
        let distinct_position_count = part1(&lab);
        assert_eq!(distinct_position_count, "41");
    }

    #[test]
    fn test_part2() {
        let lab = parse(INPUT);
        let potential_obstruction_count = part2(&lab);
        assert_eq!(potential_obstruction_count, "6");
    }
}
