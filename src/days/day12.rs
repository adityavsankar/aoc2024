use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use std::{collections::HashSet, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/12.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let garden_plot = parsed.result;
    let part1 = time_execution(|| part1(&garden_plot));
    let part2 = time_execution(|| part2(&garden_plot));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Grid<u8> {
    Grid::from(input)
}

fn is_in_region(farm: &Grid<u8>, coord: Coord, plant: u8) -> bool {
    farm.contains(coord) && farm[coord] == plant
}

fn dfs(farm: &Grid<u8>, start: Coord, plant: u8) -> HashSet<Coord> {
    let mut seen = HashSet::with_capacity(128);
    let mut stack = Vec::with_capacity(128);
    seen.insert(start);
    stack.push(start);

    while let Some(coord) = stack.pop() {
        for neighbour in coord.orthogonal_neighbours() {
            if !seen.contains(&neighbour) && is_in_region(farm, neighbour, plant) {
                seen.insert(neighbour);
                stack.push(neighbour);
            }
        }
    }

    seen
}

fn count_perimeter(farm: &Grid<u8>, region: &HashSet<Coord>, plant: u8) -> usize {
    region
        .iter()
        .map(|coord| {
            coord
                .orthogonal_neighbours()
                .into_iter()
                .filter(|&neighbour| !is_in_region(farm, neighbour, plant))
                .count()
        })
        .sum()
}

fn count_sides(farm: &Grid<u8>, region: &HashSet<Coord>, plant: u8) -> usize {
    region
        .iter()
        .map(|coord| {
            let orthogonal = coord.orthogonal_neighbours();
            let diagonal = coord.diagonal_neighbours();
            (0..4)
                .filter(|&i| {
                    let c1 = is_in_region(farm, orthogonal[i], plant);
                    let c2 = is_in_region(farm, orthogonal[(i + 1) % 4], plant);
                    let c3 = is_in_region(farm, diagonal[i], plant);
                    !(c1 || c2) || (c1 && c2 && !c3)
                })
                .count()
        })
        .sum()
}

fn solve<F>(farm: &Grid<u8>, cost_metric: F) -> usize
where
    F: Fn(&Grid<u8>, &HashSet<Coord>, u8) -> usize,
{
    let mut visited = HashSet::with_capacity(farm.height() * farm.width());
    farm.iter_with_coords()
        .filter_map(|(coord, &plant)| {
            if visited.contains(&coord) {
                return None;
            }
            let region = dfs(farm, coord, plant);
            let area = region.len();
            let metric = cost_metric(farm, &region, plant);
            visited.extend(region);
            Some(area * metric)
        })
        .sum()
}

fn part1(farm: &Grid<u8>) -> String {
    let total_fence_cost = solve(farm, count_perimeter);
    format!("{total_fence_cost}")
}

fn part2(farm: &Grid<u8>) -> String {
    let total_fence_cost = solve(farm, count_sides);
    format!("{total_fence_cost}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    const INPUT: &'static str = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    #[test]
    fn test_parse() {
        let farm = parse(INPUT);
        assert_eq!(farm.height(), 10);
        assert_eq!(farm.width(), 10);
        assert_eq!(str::from_utf8(farm.row(0)).unwrap(), "RRRRIICCFF");
        assert_eq!(str::from_utf8(farm.row(1)).unwrap(), "RRRRIICCCF");
        assert_eq!(str::from_utf8(farm.row(2)).unwrap(), "VVRRRCCFFF");
        assert_eq!(str::from_utf8(farm.row(3)).unwrap(), "VVRCCCJFFF");
        assert_eq!(str::from_utf8(farm.row(4)).unwrap(), "VVVVCJJCFE");
        assert_eq!(str::from_utf8(farm.row(5)).unwrap(), "VVIVCCJJEE");
        assert_eq!(str::from_utf8(farm.row(6)).unwrap(), "VVIIICJJEE");
        assert_eq!(str::from_utf8(farm.row(7)).unwrap(), "MIIIIIJJEE");
        assert_eq!(str::from_utf8(farm.row(8)).unwrap(), "MIIISIJEEE");
        assert_eq!(str::from_utf8(farm.row(9)).unwrap(), "MMMISSJEEE");
    }

    #[test]
    fn test_part1() {
        let farm = parse(INPUT);
        let total_fence_cost = part1(&farm);
        assert_eq!(total_fence_cost, "1930");
    }

    #[test]
    fn test_part2() {
        let topo_map = parse(INPUT);
        let total_fence_cost = part2(&topo_map);
        assert_eq!(total_fence_cost, "1206");
    }
}
