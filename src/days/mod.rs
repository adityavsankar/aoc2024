#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

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

#[derive(Clone, Copy, Debug)]
pub struct DayResult {
    pub parse_duration: Duration,
    pub part1: TimedResult<usize>,
    pub part2: TimedResult<usize>,
}
