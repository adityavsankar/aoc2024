use super::DayResult;
use crate::utils::bench::time_execution;
use std::{fs, iter::repeat};

pub fn run() -> DayResult {
    let input = fs::read_to_string("inputs/input09.txt").expect("Input file should be readable");

    let parsed = time_execution(|| parse(&input));
    let (disk, files, holes) = parsed.result;
    let part1 = time_execution(|| part1(disk));
    let part2 = time_execution(|| part2(files, holes));

    DayResult {
        parse_duration: parsed.duration,
        part1,
        part2,
    }
}

pub fn parse(input: &str) -> (Vec<isize>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let (mut disk, mut files, mut holes) = (Vec::new(), Vec::new(), Vec::new());
    let mut pos = 0;

    for (i, c) in input.trim().bytes().enumerate() {
        let len = (c - b'0') as usize;
        if i % 2 == 0 {
            disk.extend(repeat(i as isize / 2).take(len));
            files.push((pos, len));
        } else {
            disk.extend(repeat(-1).take(len));
            holes.push((pos, len));
        }
        pos += len;
    }

    (disk, files, holes)
}

pub fn part1(mut disk: Vec<isize>) -> usize {
    let mut hole_idx = disk
        .iter()
        .position(|&block| block < 0)
        .expect("Input should contain empty blocks");

    let mut file_idx = disk
        .iter()
        .rposition(|&block| block >= 0)
        .expect("Input should contain files");

    while file_idx > hole_idx {
        disk.swap(hole_idx, file_idx);
        while file_idx > hole_idx && disk[hole_idx] >= 0 {
            hole_idx += 1;
        }
        while file_idx > hole_idx && disk[file_idx] < 0 {
            file_idx -= 1;
        }
    }

    disk[..hole_idx]
        .iter()
        .enumerate()
        .map(|(i, &block)| i * block as usize)
        .sum()
}

pub fn part2(mut files: Vec<(usize, usize)>, mut holes: Vec<(usize, usize)>) -> usize {
    for (file_pos, file_len) in files.iter_mut().rev() {
        for (hole_pos, hole_len) in holes.iter_mut() {
            if *hole_pos > *file_pos {
                break;
            }
            if *file_len <= *hole_len {
                *file_pos = *hole_pos;
                *hole_pos += *file_len;
                *hole_len -= *file_len;
                break;
            }
        }
    }

    files
        .iter()
        .enumerate()
        .map(|(i, &(file_pos, file_len))| (file_pos..file_pos + file_len).sum::<usize>() * i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let (disk, files, holes) = parse(INPUT);
        assert_eq!(
            disk.len(),
            INPUT.bytes().map(|ch| (ch - b'0') as usize).sum()
        );
        assert_eq!(
            disk,
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        );
        assert_eq!(
            files,
            vec![
                (0, 2),
                (5, 3),
                (11, 1),
                (15, 3),
                (19, 2),
                (22, 4),
                (27, 4),
                (32, 3),
                (36, 4),
                (40, 2)
            ]
        );
        assert_eq!(
            holes,
            vec![
                (2, 3),
                (8, 3),
                (12, 3),
                (18, 1),
                (21, 1),
                (26, 1),
                (31, 1),
                (35, 1),
                (40, 0)
            ]
        )
    }

    #[test]
    fn test_part1() {
        let (disk, _, _) = parse(INPUT);
        let result = part1(disk);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let (_, files, holes) = parse(INPUT);
        let result = part2(files, holes);
        assert_eq!(result, 2858);
    }
}
