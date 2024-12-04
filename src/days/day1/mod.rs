#![allow(dead_code)]
use std::fs;

mod part1;
mod part2;

pub fn run() {
    println!("=================Day 1=================");
    println!("Part 1: {}", part1::part1());
    println!("Part 2: {}", part2::part2());
    println!("=======================================");
}

fn read_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(path).expect("File should be there");

    let (left, right): (Vec<i32>, Vec<i32>) = input
        .trim()
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.trim().split_whitespace().collect();
            (
                v[0].parse::<i32>().expect("Error converting to int"),
                v[1].parse::<i32>().expect("Error converting to int"),
            )
        })
        .collect();

    (left, right)
}
