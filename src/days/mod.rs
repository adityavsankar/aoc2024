#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::enum_glob_use)]

use crate::utils::bench::TimedResult;
use std::time::Duration;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

#[derive(Clone, Debug)]
pub struct DayResult {
    pub parse_duration: Duration,
    pub part1: TimedResult<String>,
    pub part2: TimedResult<String>,
}
