mod days;
mod utils;

use days::*;
use itertools::Itertools;
use std::{env, fmt::Display, process::exit, time::Duration};

fn parse_input() -> u8 {
    let args = env::args().collect_vec();
    if let Some(arg) = args.get(1) {
        if let Ok(num) = arg.parse::<u8>() {
            if (1..=25).contains(&num) {
                num
            } else {
                eprintln!("Day must be between 1 and 25");
                exit(1);
            }
        } else {
            eprintln!("Invalid number");
            exit(1);
        }
    } else {
        eprintln!("Usage: {} <day>", args[0]);
        exit(1);
    }
}

fn print_header() {
    println!("{:<15} {:<15} {:<15}", "Step", "Result", "Time Taken");
    println!("{:-<15} {:-<15} {:-<15}", "", "", "");
}

fn print_row<T: Display>(step: &str, result: T, duration: &Duration) {
    println!(
        "{:<15} {:<15} {:<15}",
        step,
        result,
        format!("{:?}", duration)
    );
}

fn print_footer(parse_duration: Duration, part1_duration: Duration, part2_duration: Duration) {
    println!("{:-<15} {:-<15} {:-<15}", "", "", "");
    println!(
        "{:<15} {:<15} {:<15}",
        "Total",
        "-",
        format!("{:?}", parse_duration + part1_duration + part2_duration)
    );
}

fn print_output(day_result: DayResult) {
    let DayResult {
        parse_duration,
        part1,
        part2,
    } = day_result;
    print_header();
    print_row("Parsing", "-", &parse_duration);
    print_row("Part 1", part1.result, &part1.duration);
    print_row("Part 2", part2.result, &part2.duration);
    print_footer(parse_duration, part1.duration, part2.duration);
}

fn main() {
    let day = parse_input();

    let day_result = match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        _ => {
            eprintln!("Day {day} not implemented!");
            exit(1);
        }
    };

    print_output(day_result);
}
