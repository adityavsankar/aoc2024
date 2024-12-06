use super::DayResult;
use crate::utils::time_execution;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/input06.txt").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let grid = parsed.result;
    let part1 = time_execution(|| part1(&grid));
    let part2 = time_execution(|| part2(grid));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn find_start(grid: &[Vec<u8>]) -> (usize, usize) {
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'^' {
                return (i, j);
            }
        }
    }
    unreachable!("Grid must have a start location")
}

fn patrol(grid: &[Vec<u8>], start: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (m, n) = (grid.len(), grid[0].len());
    let (mut i, mut j) = start;
    let mut dir = 0;

    let mut path = HashSet::with_capacity(5000);

    loop {
        if !path.insert((i, j, dir)) {
            return None;
        }
        let (ni, nj) = (i + DIRS[dir].0 as usize, j + DIRS[dir].1 as usize);
        if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
            break;
        }
        if grid[ni][nj] == b'#' {
            dir = (dir + 1) % 4;
        } else {
            (i, j) = (ni, nj);
        }
    }

    Some(
        path.into_iter()
            .map(|(i, j, _)| (i as usize, j as usize))
            .collect(),
    )
}

pub fn part1(grid: &[Vec<u8>]) -> usize {
    let start = find_start(&grid);
    let tiles = patrol(grid, start);
    tiles.expect("Input should not contain cycles").len()
}

pub fn part2(grid: Vec<Vec<u8>>) -> usize {
    let start = find_start(&grid);
    let tiles = patrol(&grid, start).expect("Input should not contain cycles");

    tiles
        .into_par_iter()
        .filter(|&(i, j)| {
            let mut g = grid.clone();
            g[i][j] = b'#';
            patrol(&g, start).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

    #[test]
    fn test_parse() {
        let grid = parse(INPUT);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid[0], vec![46, 46, 46, 46, 35, 46, 46, 46, 46, 46]);
        assert_eq!(grid[1], vec![46, 46, 46, 46, 46, 46, 46, 46, 46, 35]);
        assert_eq!(grid[2], vec![46, 46, 46, 46, 46, 46, 46, 46, 46, 46]);
        assert_eq!(grid[3], vec![46, 46, 35, 46, 46, 46, 46, 46, 46, 46]);
        assert_eq!(grid[4], vec![46, 46, 46, 46, 46, 46, 46, 35, 46, 46]);
        assert_eq!(grid[5], vec![46, 46, 46, 46, 46, 46, 46, 46, 46, 46]);
        assert_eq!(grid[6], vec![46, 35, 46, 46, 94, 46, 46, 46, 46, 46]);
        assert_eq!(grid[7], vec![46, 46, 46, 46, 46, 46, 46, 46, 35, 46]);
        assert_eq!(grid[8], vec![35, 46, 46, 46, 46, 46, 46, 46, 46, 46]);
        assert_eq!(grid[9], vec![46, 46, 46, 46, 46, 46, 35, 46, 46, 46]);
    }

    #[test]
    fn test_part1() {
        let grid = parse(INPUT);
        let result = part1(&grid);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let grid = parse(INPUT);
        let result = part2(grid);
        assert_eq!(result, 6);
    }
}
