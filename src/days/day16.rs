use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{
        Coord,
        Direction::{self, *},
        Grid, Robot,
    },
};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/16.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let maze = parsed.result;
    let part1 = time_execution(|| part1(&maze));
    let part2 = time_execution(|| part2(&maze));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> Grid<u8> {
    Grid::from(input)
}

type Node = Robot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pub cost: usize,
    pub node: Node,
}

impl State {
    pub fn next(mut self, degrees: i16) -> Self {
        self.cost += if degrees == 0 { 1 } else { 1001 };
        self.node.rotate(degrees);
        self.node.locomote();
        self
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn dijkstra(
    maze: &Grid<u8>,
    start: Coord,
    end: Coord,
) -> (HashMap<Node, Vec<Node>>, usize, Direction) {
    let mut costs = HashMap::with_capacity(1024);
    let mut prev = HashMap::with_capacity(1024);
    let mut pq = BinaryHeap::with_capacity(1024);
    let node = Node {
        pos: start,
        dir: East,
    };
    let state = State { cost: 0, node };
    costs.insert(node, 0);
    prev.insert(node, vec![]);
    pq.push(state);

    while let Some(state) = pq.pop() {
        if state.node.pos == end {
            return (prev, state.cost, state.node.dir);
        }

        let next_states = [state.next(-90), state.next(90), state.next(0)];

        for State { cost, node } in next_states {
            if maze[node.pos] == b'#' {
                continue;
            }
            let best = *costs.get(&node).unwrap_or(&usize::MAX);
            if cost <= best {
                pq.push(State { cost, node });
                costs.insert(node, cost);
                if cost < best {
                    prev.insert(node, Vec::with_capacity(4));
                }
                prev.get_mut(&node).unwrap().push(state.node);
            }
        }
    }

    unreachable!("Maze end is not reachable")
}

fn part1(maze: &Grid<u8>) -> String {
    let start = maze.position(b'S').expect("Maze should have a start");
    let end = maze.position(b'E').expect("Maze should have an end");
    let min_score = dijkstra(maze, start, end).1;
    format!("{min_score}")
}

fn part2(maze: &Grid<u8>) -> String {
    let start = maze.position(b'S').expect("Maze should have a start");
    let end = maze.position(b'E').expect("Maze should have an end");
    let (mut prev, _, last_dir) = dijkstra(maze, start, end);
    let mut seen = HashSet::with_capacity(768);
    let mut stack = vec![Node {
        pos: end,
        dir: last_dir,
    }];

    while let Some(node) = stack.pop() {
        seen.insert(node.pos);
        if let Some(x) = prev.get(&node) {
            stack.extend(x);
            prev.remove(&node);
        }
    }

    let best_seats = seen.len();
    format!("{best_seats}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;

    const INPUT: &'static str = "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############";

    #[test]
    fn test_parse() {
        let maze = parse(INPUT);

        assert_eq!(maze.height(), 15);
        assert_eq!(maze.width(), 15);
        assert_eq!(str::from_utf8(maze.row(0)).unwrap(), "###############");
        assert_eq!(str::from_utf8(maze.row(1)).unwrap(), "#.......#....E#");
        assert_eq!(str::from_utf8(maze.row(2)).unwrap(), "#.#.###.#.###.#");
        assert_eq!(str::from_utf8(maze.row(3)).unwrap(), "#.....#.#...#.#");
        assert_eq!(str::from_utf8(maze.row(4)).unwrap(), "#.###.#####.#.#");
        assert_eq!(str::from_utf8(maze.row(5)).unwrap(), "#.#.#.......#.#");
        assert_eq!(str::from_utf8(maze.row(6)).unwrap(), "#.#.#####.###.#");
        assert_eq!(str::from_utf8(maze.row(7)).unwrap(), "#...........#.#");
        assert_eq!(str::from_utf8(maze.row(8)).unwrap(), "###.#.#####.#.#");
        assert_eq!(str::from_utf8(maze.row(9)).unwrap(), "#...#.....#.#.#");
        assert_eq!(str::from_utf8(maze.row(10)).unwrap(), "#.#.#.###.#.#.#");
        assert_eq!(str::from_utf8(maze.row(11)).unwrap(), "#.....#...#.#.#");
        assert_eq!(str::from_utf8(maze.row(12)).unwrap(), "#.###.#.#.#.#.#");
        assert_eq!(str::from_utf8(maze.row(13)).unwrap(), "#S..#.....#...#");
        assert_eq!(str::from_utf8(maze.row(14)).unwrap(), "###############");
    }

    #[test]
    fn test_part1() {
        let maze = parse(INPUT);
        let min_score = part1(&maze);
        assert_eq!(min_score, "7036");
    }

    #[test]
    fn test_part2() {
        let maze = parse(INPUT);
        let best_seats = part2(&maze);
        assert_eq!(best_seats, "45");
    }
}
