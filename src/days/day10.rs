use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use std::{collections::HashSet, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/10.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let topo_map = parsed.result;
    let part1 = time_execution(|| part1(&topo_map));
    let part2 = time_execution(|| part2(&topo_map));

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
    while let Some((coord, height)) = stack.pop() {
        if height == b'9' {
            if let Some(ref mut peaks) = peaks {
                peaks.insert(coord);
            }
            rating += 1;
            continue;
        }
        for coord in coord.orthogonal_neighbours() {
            if grid.contains(coord) && grid[coord] == height + 1 {
                stack.push((coord, grid[coord]));
            }
        }
    }
    rating
}

pub fn part1(topo_map: &Grid<u8>) -> usize {
    let mut stack = Vec::new();
    let mut peaks = HashSet::new();
    topo_map
        .iter_with_coords()
        .filter(|(_, &cell)| cell == b'0')
        .map(|(coord, _)| {
            dfs(coord, topo_map, &mut stack, Some(&mut peaks));
            let score = peaks.len();
            peaks.clear();
            score
        })
        .sum()
}

pub fn part2(topo_map: &Grid<u8>) -> usize {
    let mut stack = Vec::new();
    topo_map
        .iter_with_coords()
        .filter(|(_, &cell)| cell == b'0')
        .map(|(coord, _)| dfs(coord, topo_map, &mut stack, None))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const INPUT: &'static str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_parse() {
        let topo_map = parse(INPUT);
        assert_eq!(topo_map.height(), 8);
        assert_eq!(topo_map.width(), 8);
        assert_eq!(str::from_utf8(topo_map.row(0)).unwrap(), "89010123");
        assert_eq!(str::from_utf8(topo_map.row(1)).unwrap(), "78121874");
        assert_eq!(str::from_utf8(topo_map.row(2)).unwrap(), "87430965");
        assert_eq!(str::from_utf8(topo_map.row(3)).unwrap(), "96549874");
        assert_eq!(str::from_utf8(topo_map.row(4)).unwrap(), "45678903");
        assert_eq!(str::from_utf8(topo_map.row(5)).unwrap(), "32019012");
        assert_eq!(str::from_utf8(topo_map.row(6)).unwrap(), "01329801");
        assert_eq!(str::from_utf8(topo_map.row(7)).unwrap(), "10456732");
    }

    #[test]
    fn test_part1() {
        let topo_map = parse(INPUT);
        let total_trailhead_score = part1(&topo_map);
        assert_eq!(total_trailhead_score, 36);
    }

    #[test]
    fn test_part2() {
        let topo_map = parse(INPUT);
        let total_trailhead_rating = part2(&topo_map);
        assert_eq!(total_trailhead_rating, 81);
    }
}
