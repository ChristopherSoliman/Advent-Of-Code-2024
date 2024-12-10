use super::read_input;

pub fn part1() -> i32 {
    let (mut left, mut right) = read_input("inputs/day1.txt");
    left.sort();
    right.sort();

    let diff: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    diff
}
