use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Grid, Point},
};
use std::{
    collections::{HashMap, HashSet},
    fs, str,
};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/input08.txt").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let grid = parsed.result;
    let part1 = time_execution(|| part1(&grid));
    let part2 = time_execution(|| part2(&grid));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Grid<u8> {
    input.into()
}

pub fn part1(grid: &Grid<u8>) -> usize {
    solve(grid, false)
}

pub fn part2(grid: &Grid<u8>) -> usize {
    solve(grid, true)
}

fn find_antennae(grid: &Grid<u8>) -> HashMap<u8, Vec<Point>> {
    let mut antenna_map: HashMap<u8, Vec<Point>> = HashMap::new();
    for r in 0..grid.height as isize {
        for c in 0..grid.width as isize {
            let point = Point::new(r, c);
            let ch = grid[point];
            if ch != b'.' && ch != b'#' {
                antenna_map.entry(ch).or_default().push(point);
            }
        }
    }
    antenna_map
}

fn solve(grid: &Grid<u8>, is_part2: bool) -> usize {
    let antenna_map = find_antennae(grid);
    let mut antinodes = HashSet::new();

    for antennae in antenna_map.into_values() {
        let k = antennae.len();
        for i in 0..k {
            for j in i + 1..k {
                let (tower_1, tower_2) = (antennae[i], antennae[j]);
                let offset = tower_1 - tower_2;

                for direction in [1, -1] {
                    let start = if direction == 1 { tower_1 } else { tower_2 };
                    let mut m = if is_part2 { 0 } else { 1 };
                    loop {
                        let antinode = start + offset * m * direction;
                        if !grid.contains(antinode) || (!is_part2 && m > 1) {
                            break;
                        }
                        antinodes.insert(antinode);
                        m += 1;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............\n";

    #[test]
    fn test_parse() {
        let grid = parse(INPUT);
        assert_eq!(grid.width, 12);
        assert_eq!(grid.height, 12);
        assert_eq!(str::from_utf8(grid.row(0)).unwrap(), "............");
        assert_eq!(str::from_utf8(grid.row(1)).unwrap(), "........0...");
        assert_eq!(str::from_utf8(grid.row(2)).unwrap(), ".....0......");
        assert_eq!(str::from_utf8(grid.row(3)).unwrap(), ".......0....");
        assert_eq!(str::from_utf8(grid.row(4)).unwrap(), "....0.......");
        assert_eq!(str::from_utf8(grid.row(5)).unwrap(), "......A.....");
        assert_eq!(str::from_utf8(grid.row(6)).unwrap(), "............");
        assert_eq!(str::from_utf8(grid.row(7)).unwrap(), "............");
        assert_eq!(str::from_utf8(grid.row(8)).unwrap(), "........A...");
        assert_eq!(str::from_utf8(grid.row(9)).unwrap(), ".........A..");
        assert_eq!(str::from_utf8(grid.row(10)).unwrap(), "............");
        assert_eq!(str::from_utf8(grid.row(11)).unwrap(), "............");
    }

    #[test]
    fn test_part1() {
        let grid = parse(INPUT);
        let result = part1(&grid);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let grid = parse(INPUT);
        let result = part2(&grid);
        assert_eq!(result, 34);
    }
}
