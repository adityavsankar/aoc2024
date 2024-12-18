use super::DayResult;
use crate::utils::{
    bench::time_execution,
    grid::{Coord, Grid},
};
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/14.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let robots = parsed.result;
    let part1 = time_execution(|| part1(&robots, 100, 103, 101));
    let part2 = time_execution(|| part2(&robots, 103, 101));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot14 {
    pub pos: Coord,
    pub vel: Coord,
}

fn parse(input: &str) -> Vec<Robot14> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(' ').unwrap();
            let (pos_x, pos_y) = pos.split_once(',').unwrap();
            let pos = Coord::new(pos_y.parse().unwrap(), pos_x[2..].parse().unwrap());
            let (vel_x, vel_y) = vel.split_once(',').unwrap();
            let vel = Coord::new(vel_y.parse().unwrap(), vel_x[2..].parse().unwrap());
            Robot14 { pos, vel }
        })
        .collect()
}

fn part1(robots: &[Robot14], time: usize, height: usize, width: usize) -> String {
    let (time, height, width) = (time as isize, height as isize, width as isize);
    let (half_height, half_width) = (height / 2, width / 2);
    let (mut top_left, mut top_right, mut bot_left, mut bot_right) = (0, 0, 0, 0);

    for robot in robots {
        let new_pos = robot.pos + robot.vel * time;
        let final_x = new_pos.x().rem_euclid(width);
        let final_y = new_pos.y().rem_euclid(height);
        if final_x == half_width || final_y == half_height {
            continue;
        }
        if (0..half_width).contains(&final_x) {
            if (0..half_height).contains(&final_y) {
                top_left += 1;
            } else {
                bot_left += 1;
            }
        } else if (0..half_height).contains(&final_y) {
            top_right += 1;
        } else {
            bot_right += 1;
        }
    }

    let safety_factor: usize = top_left * top_right * bot_left * bot_right;
    format!("{safety_factor}")
}

fn part2(robots: &[Robot14], height: usize, width: usize) -> String {
    let (height, width) = (height as isize, width as isize);
    let mut elapsed = 0;
    let mut seen = Grid::new(height as usize, width as usize, false);

    'a: loop {
        for robot in robots {
            let new_pos = robot.pos + robot.vel * elapsed;
            let final_x = new_pos.x().rem_euclid(width);
            let final_y = new_pos.y().rem_euclid(height);
            let final_pos = Coord::new(final_y, final_x);
            if seen[final_pos] {
                elapsed += 1;
                seen.fill(false);
                continue 'a;
            }
            seen[final_pos] = true;
        }
        return format!("{elapsed}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3\n";

    #[test]
    fn test_parse() {
        let robots = parse(INPUT);
        assert_eq!(robots.len(), 12);
        assert_eq!(
            robots,
            vec![
                Robot14 {
                    pos: Coord::new(4, 0),
                    vel: Coord::new(-3, 3)
                },
                Robot14 {
                    pos: Coord::new(3, 6),
                    vel: Coord::new(-3, -1)
                },
                Robot14 {
                    pos: Coord::new(3, 10),
                    vel: Coord::new(2, -1)
                },
                Robot14 {
                    pos: Coord::new(0, 2),
                    vel: Coord::new(-1, 2)
                },
                Robot14 {
                    pos: Coord::new(0, 0),
                    vel: Coord::new(3, 1)
                },
                Robot14 {
                    pos: Coord::new(0, 3),
                    vel: Coord::new(-2, -2)
                },
                Robot14 {
                    pos: Coord::new(6, 7),
                    vel: Coord::new(-3, -1)
                },
                Robot14 {
                    pos: Coord::new(0, 3),
                    vel: Coord::new(-2, -1)
                },
                Robot14 {
                    pos: Coord::new(3, 9),
                    vel: Coord::new(3, 2)
                },
                Robot14 {
                    pos: Coord::new(3, 7),
                    vel: Coord::new(2, -1)
                },
                Robot14 {
                    pos: Coord::new(4, 2),
                    vel: Coord::new(-3, 2)
                },
                Robot14 {
                    pos: Coord::new(5, 9),
                    vel: Coord::new(-3, -3)
                },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let robots = parse(INPUT);
        let safety_factor = part1(&robots, 100, 7, 11);
        assert_eq!(safety_factor, "12");
    }

    #[test]
    fn test_part2() {
        let robots = parse(INPUT);
        let tree_time = part2(&robots, 7, 11);
        assert_eq!(tree_time, "1");
    }
}
