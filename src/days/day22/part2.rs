use std::collections::{HashMap, HashSet};

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut prices: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let secret = line.parse::<u64>().unwrap();
        prices.push(get_secret(secret, 2000));
    }

    let diffs = prices
        .iter()
        .map(|m| {
            m.windows(2)
                .map(|p| p[1] as i8 - p[0] as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<_>>();

    for i in 0..prices.len() {
        prices[i].remove(0);
    }
    min_sequence(&diffs, &prices)
}

fn get_secret(secret: u64, iter: u32) -> Vec<u8> {
    let mut prices: Vec<u8> = Vec::from([(secret % 10) as u8]);
    let mut new_secret = ((secret * 64) ^ secret) % 16777216;
    new_secret = ((new_secret / 32) ^ new_secret) % 16777216;
    new_secret = ((new_secret * 2048) ^ new_secret) % 16777216;
    if iter != 1 {
        prices.append(&mut get_secret(new_secret, iter - 1))
    }
    return prices;
}

fn min_sequence(diffs: &Vec<Vec<i8>>, prices: &Vec<Vec<u8>>) -> u32 {
    let mut sequences: HashMap<[i8; 4], u32> = HashMap::new();
    let mut seen: HashSet<[i8; 4]> = HashSet::new();
    let mut max = 0;
    for i in 0..diffs.len() {
        seen.clear();
        for j in 0..diffs[i].len() - 4 {
            let new = [
                diffs[i][j],
                diffs[i][j + 1],
                diffs[i][j + 2],
                diffs[i][j + 3],
            ];
            if !seen.contains(&new) {
                let val = prices[i][j + 3];
                let new_val = sequences
                    .entry(new)
                    .and_modify(|v| *v += val as u32)
                    .or_insert(val as u32);
                if *new_val > max {
                    max = *new_val;
                }
                seen.insert(new);
            }
        }
    }
    max
}
