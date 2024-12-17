mod days;
mod utils;

use crate::utils::io::{input, output};
#[allow(clippy::wildcard_imports)]
use days::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = input::parse_args()?;

    let day_result = match day {
        1 => Ok(day01::run()),
        2 => Ok(day02::run()),
        3 => Ok(day03::run()),
        4 => Ok(day04::run()),
        5 => Ok(day05::run()),
        6 => Ok(day06::run()),
        7 => Ok(day07::run()),
        8 => Ok(day08::run()),
        9 => Ok(day09::run()),
        10 => Ok(day10::run()),
        11 => Ok(day11::run()),
        12 => Ok(day12::run()),
        13 => Ok(day13::run()),
        14 => Ok(day14::run()),
        15 => Ok(day15::run()),
        16 => Ok(day16::run()),
        _ => Err(format!("Day {day} not implemented!")),
    }?;

    output::print_table(day_result, 15);

    Ok(())
}
