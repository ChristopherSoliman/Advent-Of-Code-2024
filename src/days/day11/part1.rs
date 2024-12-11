const BLINKS: u8 = 25;

pub fn part1(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut stones: Vec<usize> = input
        .trim()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().expect("Couldn't parse number"))
        .collect();

    for _ in 0..BLINKS {
        let mut j = 0;
        while j < stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
                j += 1;
                continue;
            }

            let num_str = stones[j].to_string();
            if num_str.len() % 2 == 0 {
                let nums = num_str.split_at(num_str.len() / 2);
                let nums = (
                    nums.0.parse::<usize>().unwrap(),
                    nums.1.parse::<usize>().unwrap(),
                );
                stones.remove(j);

                stones.insert(j, nums.0);
                stones.insert(j + 1, nums.1);

                j += 2;
                continue;
            }
            stones[j] *= 2024;
            j += 1;
        }
    }

    stones.len()
}
