use super::DayResult;
use crate::utils::bench::time_execution;
use std::fs;

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/input04.txt").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let grid = parsed.result;
    let part1 = time_execution(|| part1(&grid, "XMAS".as_bytes()));
    let part2 = time_execution(|| part2(&grid, "MAS".as_bytes()));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn match_count(mat: &[Vec<u8>], target: &[u8], r: usize, c: usize, m: usize, n: usize) -> u8 {
    let (m, n, r, c) = (m as isize, n as isize, r as isize, c as isize);
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if target.iter().enumerate().all(|(k, &ch)| {
                let nr = r + k as isize * i;
                let nc = c + k as isize * j;
                (0..m).contains(&nr) && (0..n).contains(&nc) && mat[nr as usize][nc as usize] == ch
            }) {
                count += 1;
            }
        }
    }

    count
}

pub fn part1(mat: &[Vec<u8>], target: &[u8]) -> usize {
    let first_char = target[0];
    let (m, n) = (mat.len(), mat[0].len());
    let mut ans = 0;

    for r in 0..m {
        for c in 0..n {
            if mat[r][c] == first_char {
                ans += match_count(mat, target, r, c, m, n) as usize;
            }
        }
    }

    ans
}

fn is_x(mat: &[Vec<u8>], target: &[u8], rev_target: &[u8], r: usize, c: usize, th: usize) -> bool {
    let (r, c, th) = (r as isize, c as isize, th as isize);

    let check_diagonal = |dir: isize, target: &[u8]| {
        (-th..=th)
            .enumerate()
            .all(|(i, k)| mat[(r + k) as usize][(c + k * dir) as usize] == target[i])
    };

    (check_diagonal(1, target) || check_diagonal(1, rev_target))
        && (check_diagonal(-1, target) || check_diagonal(-1, rev_target))
}

pub fn part2(mat: &[Vec<u8>], target: &[u8]) -> usize {
    let t = target.len();
    if t % 2 == 0 {
        eprintln!("The target must be of odd length");
        return 0;
    }
    let th = t / 2;
    let middle_char = target[th];
    let rev_target: Vec<u8> = target.iter().rev().copied().collect();
    let (m, n) = (mat.len(), mat[0].len());
    let mut ans = 0;

    for r in th..m - th {
        for c in th..n - th {
            if mat[r][c] == middle_char && is_x(mat, target, &rev_target, r, c, th) {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_parse() {
        let grid = parse(INPUT);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid[0], vec![77, 77, 77, 83, 88, 88, 77, 65, 83, 77]);
        assert_eq!(grid[1], vec![77, 83, 65, 77, 88, 77, 83, 77, 83, 65]);
        assert_eq!(grid[2], vec![65, 77, 88, 83, 88, 77, 65, 65, 77, 77]);
        assert_eq!(grid[3], vec![77, 83, 65, 77, 65, 83, 77, 83, 77, 88]);
        assert_eq!(grid[4], vec![88, 77, 65, 83, 65, 77, 88, 65, 77, 77]);
        assert_eq!(grid[5], vec![88, 88, 65, 77, 77, 88, 88, 65, 77, 65]);
        assert_eq!(grid[6], vec![83, 77, 83, 77, 83, 65, 83, 88, 83, 83]);
        assert_eq!(grid[7], vec![83, 65, 88, 65, 77, 65, 83, 65, 65, 65]);
        assert_eq!(grid[8], vec![77, 65, 77, 77, 77, 88, 77, 77, 77, 77]);
        assert_eq!(grid[9], vec![77, 88, 77, 88, 65, 88, 77, 65, 83, 88]);
    }

    #[test]
    fn test_part1() {
        let grid = parse(INPUT);
        let result = part1(&grid, "XMAS".as_bytes());
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let grid = parse(INPUT);
        let result = part2(&grid, "MAS".as_bytes());
        assert_eq!(result, 9);
    }
}
