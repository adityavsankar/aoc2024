use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use std::{collections::VecDeque, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/18.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let corrupted = parsed.result;
    let part1 = time_execution(|| part1(71, 71, &corrupted, 1024));
    let part2 = time_execution(|| part2(71, 71, &corrupted));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (c, r) = line.split_once(',').unwrap();
            let (c, r) = (c.parse().unwrap(), r.parse().unwrap());
            Coord::new(r, c)
        })
        .collect()
}

fn bfs(memory_space: &Grid<u8>) -> Option<usize> {
    let (height, width) = (memory_space.height(), memory_space.width());
    let mut seen = Grid::new(height, width, false);
    let mut queue = VecDeque::from([(Coord::new(0, 0), 0)]);
    let end = Coord::new(height as isize - 1, width as isize - 1);

    while let Some((coord, dist)) = queue.pop_front() {
        if coord == end {
            return Some(dist);
        }

        for neighbour in coord.orthogonal_neighbours() {
            if memory_space.contains(neighbour)
                && memory_space[neighbour] != b'#'
                && !seen[neighbour]
            {
                seen[neighbour] = true;
                queue.push_back((neighbour, dist + 1));
            }
        }
    }

    None
}

fn part1(height: usize, width: usize, corrupted: &[Coord], n: usize) -> String {
    let mut memory_space = Grid::new(height, width, b'.');
    for &coord in &corrupted[..n] {
        memory_space[coord] = b'#';
    }
    let min_dist = bfs(&memory_space).expect("Memory space should contain valid path");
    format!("{min_dist}")
}

fn part2(height: usize, width: usize, corrupted: &[Coord]) -> String {
    let mut memory_space = Grid::new(height, width, b'.');
    let mut best = None;
    let (mut low, mut high) = (0, corrupted.len() - 1);
    while low < high {
        let mid = (low + high) / 2;
        for &coord in &corrupted[..=mid] {
            memory_space[coord] = b'#';
        }
        if bfs(&memory_space).is_none() {
            best = Some(corrupted[mid]);
            high = mid;
        } else {
            low = mid + 1;
        }
        for &coord in &corrupted[..=mid] {
            memory_space[coord] = b'.';
        }
    }
    let best = best.expect("Memory space should be obstructed for some corrupted byte");
    format!("{},{}", best.x(), best.y())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;

    const INPUT: &'static str = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn test_parse() {
        let corrupted = parse(INPUT);

        assert_eq!(corrupted.len(), 25);
        assert_eq!(
            corrupted,
            vec![
                Coord::new(4, 5),
                Coord::new(2, 4),
                Coord::new(5, 4),
                Coord::new(0, 3),
                Coord::new(1, 2),
                Coord::new(3, 6),
                Coord::new(4, 2),
                Coord::new(5, 1),
                Coord::new(6, 0),
                Coord::new(3, 3),
                Coord::new(6, 2),
                Coord::new(1, 5),
                Coord::new(2, 1),
                Coord::new(5, 5),
                Coord::new(5, 2),
                Coord::new(5, 6),
                Coord::new(4, 1),
                Coord::new(4, 0),
                Coord::new(4, 6),
                Coord::new(1, 1),
                Coord::new(1, 6),
                Coord::new(0, 1),
                Coord::new(5, 0),
                Coord::new(6, 1),
                Coord::new(0, 2)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let maze = parse(INPUT);
        let min_dist = part1(7, 7, &maze, 12);
        assert_eq!(min_dist, "22");
    }

    #[test]
    fn test_part2() {
        let maze = parse(INPUT);
        let first_obstacle = part2(7, 7, &maze);
        assert_eq!(first_obstacle, "6,1");
    }
}
