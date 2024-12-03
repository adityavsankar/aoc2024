use std::time::Duration;

use crate::utils::TimedResult;

pub mod day01;
pub mod day02;
pub mod day03;

pub struct DayResult {
    pub parse_duration: Duration,
    pub part1: TimedResult<usize>,
    pub part2: TimedResult<usize>,
}