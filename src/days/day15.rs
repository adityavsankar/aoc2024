use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{
        Coord,
        Direction::{self, *},
        Grid, Robot,
    },
};
use std::{collections::VecDeque, fs};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/15.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (warehouse1, warehouse2, movements) = parsed.result;
    let part1 = time_execution(|| part1(warehouse1, &movements));
    let part2 = time_execution(|| part2(warehouse2, &movements));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> (Grid<u8>, Grid<u8>, Vec<Direction>) {
    let input = input.trim().replace('\r', "");
    let (warehouse, movements) = input
        .split_once("\n\n")
        .expect("Warehouse and movements should be separated by a blank line");

    let grid = Grid::from(warehouse);

    let expanded_warehouse = warehouse
        .replace('#', "##")
        .replace('.', "..")
        .replace('O', "[]")
        .replace('@', "@.");
    let expanded_grid = Grid::from(expanded_warehouse.as_str());

    let movements = movements.lines().collect::<Vec<_>>().join("");
    let movements = movements
        .bytes()
        .map(|movement| match movement {
            b'^' => North,
            b'>' => East,
            b'v' => South,
            b'<' => West,
            _ => unreachable!("Movements should only consist of ^><v"),
        })
        .collect();

    (grid, expanded_grid, movements)
}

fn go(warehouse: &mut Grid<u8>, robot: &mut Robot) {
    warehouse[robot.position()] = b'.';
    warehouse[robot.next_pos()] = b'@';
    robot.locomote();
}

fn part1(mut warehouse: Grid<u8>, movements: &[Direction]) -> String {
    let start = warehouse
        .position(b'@')
        .expect("Warehouse should contain robot");
    let mut robot = Robot::new(start, North);

    for &movement in movements {
        robot.dir = movement;
        let next_pos = robot.next_pos();

        match warehouse[next_pos] {
            b'#' => (),
            b'.' => {
                go(&mut warehouse, &mut robot);
            }
            b'O' => {
                let mut fut_pos = next_pos + movement;
                while warehouse[fut_pos] == b'O' {
                    fut_pos += movement;
                }
                if warehouse[fut_pos] != b'#' {
                    warehouse[fut_pos] = b'O';
                    go(&mut warehouse, &mut robot);
                }
            }
            _ => unreachable!(),
        }
    }

    let total_gps_coordinates: isize = warehouse
        .positions(b'O')
        .map(|coord| coord.r * 100 + coord.c)
        .sum();
    format!("{total_gps_coordinates}")
}

fn part2(mut warehouse: Grid<u8>, movements: &[Direction]) -> String {
    let start = warehouse
        .position(b'@')
        .expect("Warehouse should contain robot");
    let mut robot = Robot::new(start, North);

    'a: for &movement in movements {
        robot.dir = movement;
        let next_pos = robot.next_pos();

        if warehouse[next_pos] == b'#' {
            continue;
        }
        if warehouse[next_pos] == b'.' {
            go(&mut warehouse, &mut robot);
            continue;
        }

        let dir = Coord::from(movement);
        match movement {
            East | West => {
                let (b1, b2) = if warehouse[next_pos] == b'[' {
                    (b'[', b']')
                } else {
                    (b']', b'[')
                };

                let mut len = 0;
                let mut fut_pos = next_pos;
                while warehouse[fut_pos] == b1 {
                    fut_pos += dir * 2;
                    len += 2;
                }

                if warehouse[fut_pos] != b'#' {
                    for i in 1..=len {
                        let pos = next_pos + dir * i;
                        if i % 2 == 1 {
                            warehouse[pos] = b1;
                        } else {
                            warehouse[pos] = b2;
                        }
                    }
                    go(&mut warehouse, &mut robot);
                }
            }
            North | South => {
                let mut queue = VecDeque::new();
                let mut seen = Vec::new();
                let adj_pos = if warehouse[next_pos] == b'[' {
                    next_pos + East
                } else {
                    next_pos + West
                };
                queue.push_back(next_pos);
                queue.push_back(adj_pos);
                seen.push(next_pos);
                seen.push(adj_pos);

                while let Some(coord) = queue.pop_front() {
                    let new_pos = coord + dir;

                    if warehouse[new_pos] == b'.' {
                        continue;
                    }
                    if warehouse[new_pos] == b'#' {
                        continue 'a;
                    }

                    let adj_pos = if warehouse[new_pos] == b'[' {
                        new_pos + East
                    } else {
                        new_pos + West
                    };

                    if !seen.contains(&new_pos) {
                        queue.push_back(new_pos);
                        seen.push(new_pos);
                    }
                    if !seen.contains(&adj_pos) {
                        queue.push_back(adj_pos);
                        seen.push(adj_pos);
                    }
                }

                for coord in seen.into_iter().rev() {
                    let new_coord = coord + dir;
                    warehouse[new_coord] = warehouse[coord];
                    warehouse[coord] = b'.';
                }

                go(&mut warehouse, &mut robot);
            }
            _ => unreachable!(),
        }
    }

    let total_gps_coordinates: isize = warehouse
        .positions(b'[')
        .map(|coord| coord.r * 100 + coord.c)
        .sum();
    format!("{total_gps_coordinates}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str;

    const INPUT1: &'static str = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";
    const INPUT2: &'static str =
        "#######\n#...#.#\n#.....#\n#..OO@#\n#..O..#\n#.....#\n#######\n\n<vv<<^^<<^^";

    #[test]
    fn test_parse() {
        let (warehouse1, _, movements1) = parse(INPUT1);

        assert_eq!(warehouse1.height(), 8);
        assert_eq!(warehouse1.width(), 8);
        assert_eq!(str::from_utf8(warehouse1.row(0)).unwrap(), "########");
        assert_eq!(str::from_utf8(warehouse1.row(1)).unwrap(), "#..O.O.#");
        assert_eq!(str::from_utf8(warehouse1.row(2)).unwrap(), "##@.O..#");
        assert_eq!(str::from_utf8(warehouse1.row(3)).unwrap(), "#...O..#");
        assert_eq!(str::from_utf8(warehouse1.row(4)).unwrap(), "#.#.O..#");
        assert_eq!(str::from_utf8(warehouse1.row(5)).unwrap(), "#...O..#");
        assert_eq!(str::from_utf8(warehouse1.row(6)).unwrap(), "#......#");
        assert_eq!(str::from_utf8(warehouse1.row(7)).unwrap(), "########");

        assert_eq!(
            movements1,
            vec![
                West, North, North, East, East, East, South, South, West, South, East, East, South,
                West, West
            ]
        );

        let (_, warehouse2, movements2) = parse(INPUT2);

        assert_eq!(warehouse2.height(), 7);
        assert_eq!(warehouse2.width(), 14);
        assert_eq!(str::from_utf8(warehouse2.row(0)).unwrap(), "##############");
        assert_eq!(str::from_utf8(warehouse2.row(1)).unwrap(), "##......##..##");
        assert_eq!(str::from_utf8(warehouse2.row(2)).unwrap(), "##..........##");
        assert_eq!(str::from_utf8(warehouse2.row(3)).unwrap(), "##....[][]@.##");
        assert_eq!(str::from_utf8(warehouse2.row(4)).unwrap(), "##....[]....##");
        assert_eq!(str::from_utf8(warehouse2.row(5)).unwrap(), "##..........##");
        assert_eq!(str::from_utf8(warehouse2.row(6)).unwrap(), "##############");

        assert_eq!(
            movements2,
            vec![West, South, South, West, West, North, North, West, West, North, North]
        );
    }

    #[test]
    fn test_part1() {
        let (warehouse, _, movements) = parse(INPUT1);
        let total_gps_coordinates = part1(warehouse, &movements);
        assert_eq!(total_gps_coordinates, "2028");
    }

    #[test]
    fn test_part2() {
        let (_, warehouse, movements) = parse(INPUT2);
        let total_min_tokens = part2(warehouse, &movements);
        assert_eq!(total_min_tokens, "618");
    }
}
