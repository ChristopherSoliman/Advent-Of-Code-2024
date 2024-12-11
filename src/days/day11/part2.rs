use std::collections::HashMap;

const BLINKS: u8 = 75;

pub fn part2(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut stones: HashMap<usize, usize> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| (v.parse().expect("Couldn't parse number"), 1))
        .collect::<HashMap<_, _>>();

    let mut updates: HashMap<usize, usize> = HashMap::<_, _>::new();

    for _ in 0..BLINKS {
        updates.clear();

        for (stone, count) in stones {
            let stone_str = stone.to_string();
            if stone == 0 {
                *updates.entry(1).or_insert(0) += count;
            } else if stone_str.len() % 2 == 0 {
                let nums = stone_str.split_at(stone_str.len() / 2);
                let nums = (nums.0.parse().unwrap(), nums.1.parse().unwrap());
                *updates.entry(nums.0).or_insert(0) += count;
                *updates.entry(nums.1).or_insert(0) += count;
            } else {
                *updates.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stones = updates.clone();
    }

    stones.iter().fold(0, |acc, (_, val)| acc + val)
}
