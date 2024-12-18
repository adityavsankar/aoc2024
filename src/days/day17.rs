use super::DayResult;
use crate::utils::bench::time_execution;
use itertools::Itertools;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/17.in").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (a, b, c, program) = parsed.result;
    let part1 = time_execution(|| part1(a, b, c, &program));
    let part2 = time_execution(|| part2(&program));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

fn parse(input: &str) -> (u64, u64, u64, Vec<u64>) {
    let lines: Vec<&str> = input.lines().collect();
    let a = lines[0][12..]
        .parse()
        .expect("Register A value should be an integer");
    let b = lines[1][12..]
        .parse()
        .expect("Register A value should be an integer");
    let c = lines[2][12..]
        .parse()
        .expect("Register A value should be an integer");
    let program = lines[4][9..]
        .split(',')
        .map(|n| n.parse().expect("Program should consist of integers"))
        .collect();
    (a, b, c, program)
}

fn compute(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let n = program.len() - 1;
    let mut ip = 0;
    let mut output = Vec::new();

    while ip < n {
        let opcode = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            literal @ 0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!("Invalid literal operand"),
        };

        match opcode {
            0 => a >>= combo,
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => unreachable!("Invalid opcode"),
        }

        ip += 2;
    }
    output
}

fn part1(a: u64, b: u64, c: u64, program: &[u64]) -> String {
    let output = compute(a, b, c, program);
    output.iter().join(",")
}

fn part2(program: &[u64]) -> String {
    let mut a = 1;
    let mut index = program.len() - 1;
    loop {
        if compute(a, 0, 0, program) == program[index..] {
            if index == 0 {
                return format!("{a}");
            }
            a *= 8;
            index -= 1;
        } else {
            a += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
        "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";
    const INPUT2: &'static str =
        "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0";

    #[test]
    fn test_parse() {
        let (a, b, c, program) = parse(INPUT1);
        assert_eq!(a, 729);
        assert_eq!(b, 0);
        assert_eq!(c, 0);
        assert_eq!(program.len(), 6);
        assert_eq!(program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_part1() {
        let (a, b, c, program) = parse(INPUT1);
        let output = part1(a, b, c, &program);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let (_, _, _, program) = parse(INPUT2);
        let quine_state = part2(&program);
        assert_eq!(quine_state, "117440");
    }
}
