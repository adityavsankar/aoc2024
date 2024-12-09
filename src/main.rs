mod days;
mod utils;

#[allow(clippy::wildcard_imports)]
use days::*;
use std::{env, error::Error, fmt::Display, time::Duration};

fn parse_input() -> Result<u8, String> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        match arg.parse::<u8>() {
            Ok(num) if (1..=25).contains(&num) => Ok(num),
            Ok(_) => Err("Day must be between 1 and 25".into()),
            Err(_) => Err("Invalid number".into()),
        }
    } else {
        Err(format!("Usage: {} <day>", args[0]))
    }
}

fn print_header(column_width: usize) {
    println!(
        "{:<width$} {:<width$} {:<width$}",
        "Step",
        "Result",
        "Time Taken",
        width = column_width
    );
    println!(
        "{:-<width$} {:-<width$} {:-<width$}",
        "",
        "",
        "",
        width = column_width
    );
}

fn print_row<T: Display>(step: &str, result: T, duration: &Duration, column_width: usize) {
    println!(
        "{:<width$} {:<width$} {:<width$}",
        step,
        result,
        format!("{:?}", duration),
        width = column_width
    );
}

fn print_footer(
    parse_duration: Duration,
    part1_duration: Duration,
    part2_duration: Duration,
    column_width: usize,
) {
    println!(
        "{:-<width$} {:-<width$} {:-<width$}",
        "",
        "",
        "",
        width = column_width
    );
    println!(
        "{:<width$} {:<width$} {:<width$}",
        "Total",
        "-",
        format!("{:?}", parse_duration + part1_duration + part2_duration),
        width = column_width
    );
}

fn print_output(day_result: DayResult, column_width: usize) {
    let DayResult {
        parse_duration,
        part1,
        part2,
    } = day_result;

    print_header(column_width);
    print_row("Parsing", "-", &parse_duration, column_width);
    print_row("Part 1", part1.result, &part1.duration, column_width);
    print_row("Part 2", part2.result, &part2.duration, column_width);
    print_footer(parse_duration, part1.duration, part2.duration, column_width);
}

fn main() -> Result<(), Box<dyn Error>> {
    let day = parse_input()?;

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
        _ => Err(format!("Day {day} not implemented!")),
    }?;

    print_output(day_result, 20);

    Ok(())
}
