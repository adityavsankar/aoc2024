use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use std::{collections::HashSet, fs, str};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/10.in").expect("Input file should be readable");

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

fn dfs(
    start: Coord,
    grid: &Grid<u8>,
    stack: &mut Vec<(Coord, u8)>,
    mut peaks: Option<&mut HashSet<Coord>>,
) -> usize {
    let mut rating = 0;
    stack.push((start, b'0'));
    while let Some((point, height)) = stack.pop() {
        if height == b'9' {
            if let Some(ref mut peaks) = peaks {
                peaks.insert(point);
            }
            rating += 1;
            continue;
        }
        for point in point.orthogonal_neighbours() {
            if grid.contains(point) && grid[point] == height + 1 {
                stack.push((point, grid[point]));
            }
        }
    }
    rating
}

pub fn part1(grid: &Grid<u8>) -> usize {
    let mut stack = Vec::new();
    let mut peaks = HashSet::new();
    grid.iter_with_coords()
        .filter(|(_, cell)| **cell == b'0')
        .map(|(point, _)| {
            dfs(point, grid, &mut stack, Some(&mut peaks));
            let score = peaks.len();
            peaks.clear();
            score
        })
        .sum()
}

pub fn part2(grid: &Grid<u8>) -> usize {
    let mut stack = Vec::new();
    grid.iter_with_coords()
        .filter(|(_, cell)| **cell == b'0')
        .map(|(point, _)| dfs(point, grid, &mut stack, None))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_parse() {
        let grid = parse(INPUT);
        assert_eq!(grid.height(), 8);
        assert_eq!(grid.width(), 8);
        assert_eq!(str::from_utf8(grid.row(0)).unwrap(), "89010123");
        assert_eq!(str::from_utf8(grid.row(1)).unwrap(), "78121874");
        assert_eq!(str::from_utf8(grid.row(2)).unwrap(), "87430965");
        assert_eq!(str::from_utf8(grid.row(3)).unwrap(), "96549874");
        assert_eq!(str::from_utf8(grid.row(4)).unwrap(), "45678903");
        assert_eq!(str::from_utf8(grid.row(5)).unwrap(), "32019012");
        assert_eq!(str::from_utf8(grid.row(6)).unwrap(), "01329801");
        assert_eq!(str::from_utf8(grid.row(7)).unwrap(), "10456732");
    }

    #[test]
    fn test_part1() {
        let grid = parse(INPUT);
        let result = part1(&grid);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2() {
        let grid = parse(INPUT);
        let result = part2(&grid);
        assert_eq!(result, 81);
    }
}
