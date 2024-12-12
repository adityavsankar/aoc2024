use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs, str,
};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/08.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let city = parsed.result;
    let part1 = time_execution(|| part1(&city));
    let part2 = time_execution(|| part2(&city));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from(input)
}

pub fn part1(city: &Grid<u8>) -> usize {
    solve(city, false)
}

pub fn part2(city: &Grid<u8>) -> usize {
    solve(city, true)
}

fn find_antennae(city: &Grid<u8>) -> HashMap<u8, Vec<Coord>> {
    let mut antenna_map: HashMap<u8, Vec<Coord>> = HashMap::new();
    city.iter_with_coords()
        .filter(|(_, &ch)| ch != b'.' && ch != b'#')
        .for_each(|(coord, &ch)| antenna_map.entry(ch).or_default().push(coord));
    antenna_map
}

fn solve(city: &Grid<u8>, is_part2: bool) -> usize {
    let antenna_map = find_antennae(city);
    let mut antinodes = HashSet::new();

    for antennae in antenna_map.into_values() {
        let k = antennae.len();
        for (i, j) in (0..k).tuple_combinations() {
            let (tower1, tower2) = (antennae[i], antennae[j]);
            let offset = tower1 - tower2;

            for direction in [1, -1] {
                let start = if direction == 1 { tower1 } else { tower2 };
                let mut m = isize::from(!is_part2);
                loop {
                    let antinode = start + offset * m * direction;
                    if !city.contains(antinode) || (!is_part2 && m > 1) {
                        break;
                    }
                    antinodes.insert(antinode);
                    m += 1;
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
        let city = parse(INPUT);
        assert_eq!(city.width(), 12);
        assert_eq!(city.height(), 12);
        assert_eq!(str::from_utf8(city.row(0)).unwrap(), "............");
        assert_eq!(str::from_utf8(city.row(1)).unwrap(), "........0...");
        assert_eq!(str::from_utf8(city.row(2)).unwrap(), ".....0......");
        assert_eq!(str::from_utf8(city.row(3)).unwrap(), ".......0....");
        assert_eq!(str::from_utf8(city.row(4)).unwrap(), "....0.......");
        assert_eq!(str::from_utf8(city.row(5)).unwrap(), "......A.....");
        assert_eq!(str::from_utf8(city.row(6)).unwrap(), "............");
        assert_eq!(str::from_utf8(city.row(7)).unwrap(), "............");
        assert_eq!(str::from_utf8(city.row(8)).unwrap(), "........A...");
        assert_eq!(str::from_utf8(city.row(9)).unwrap(), ".........A..");
        assert_eq!(str::from_utf8(city.row(10)).unwrap(), "............");
        assert_eq!(str::from_utf8(city.row(11)).unwrap(), "............");
    }

    #[test]
    fn test_part1() {
        let city = parse(INPUT);
        let antinode_count = part1(&city);
        assert_eq!(antinode_count, 14);
    }

    #[test]
    fn test_part2() {
        let city = parse(INPUT);
        let antinode_count = part2(&city);
        assert_eq!(antinode_count, 34);
    }
}
