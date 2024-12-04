#![allow(dead_code)]
use std::fs;

mod part1;
mod part2;

pub fn run() {
    println!("=================Day 3=================");
    println!("Part 1: {}", part1("inputs/day3.txt"));
    println!("Part 2: {}", part2("inputs/day3.txt"));
    println!("=======================================");
}

fn part1(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("File should be there");
    let mut program = part1::Program::new(input.trim().chars());
    program.parse()
}

fn part2(path: &str) -> i32 {
    let input = fs::read_to_string(path).expect("File should be there");
    let mut program = part2::Program::new(input.trim().chars());
    program.parse()
}
