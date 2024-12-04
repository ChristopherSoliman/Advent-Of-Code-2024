#![allow(dead_code)]
use std::{cmp::min, fs};

pub fn run() {
    println!("=================Day 2=================");
    println!("Part 1: {}", part1("inputs/day2.txt"));
    println!("Part 2: {}", part2("inputs/day2.txt"));
    println!("=======================================");
}

fn part1(path: &str) -> usize {
    let input = fs::read_to_string(path).expect("File should be there");

    let result: usize = input
        .trim()
        .lines()
        .filter(|line| {
            let v: Vec<i32> = line
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|x| x.parse().expect("error converting int"))
                .collect();

            is_safe(&v, false)
        })
        .count();

    result
}

fn part2(path: &str) -> usize {
    let input = fs::read_to_string(path).expect("File should be there");

    let result: usize = input
        .trim()
        .lines()
        .filter(|line| {
            let v: Vec<i32> = line
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|x| x.parse().expect("error converting int"))
                .collect();

            is_safe(&v, true)
        })
        .count();

    result
}

fn is_safe(line: &[i32], tolerate: bool) -> bool {
    if line[1] - line[0] == 0 {
        if tolerate {
            return attempt_to_validate(&line.to_vec(), 0);
        }
        return false;
    }

    let direction = (line[1] - line[0]) / line[1].abs_diff(line[0]) as i32;
    for i in 0..line.len() - 1 {
        if (line[i + 1] - line[i]) * direction < 1 || line[i + 1].abs_diff(line[i]) > 3 {
            if tolerate {
                return attempt_to_validate(&line.to_vec(), i);
            }
            return false;
        }
    }
    true
}
fn attempt_to_validate(line: &Vec<i32>, failed_at: usize) -> bool {
    let start = match failed_at {
        0 => 1,
        _ => 0,
    };
    let end = min(2, line.len() - failed_at) + 1;
    for n in start..end {
        let mut temp_line = line.clone();
        temp_line.remove(failed_at + n - 1);
        if is_safe(&temp_line, false) {
            return true;
        }
    }
    false
}
