pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut scores = vec![0; 0x9CE73];

    for line in input.lines() {
        let mut seen = vec![false; 0x9CE73];
        let mut secret = line.parse::<u64>().unwrap();
        let mut prev_score = secret % 10;
        let mut diff: usize = 0;
        for i in 0..2000 {
            secret = get_secret(secret);
            let score = secret % 10;
            // 10 + score - prev_score has a max value of 19 (0b10011)
            // Remove the last 5 bits and add the current unsigned diff
            diff = ((diff << 5) & 0xFFFF0) | (10 + score as usize - prev_score as usize);
            if !seen[diff] && i > 3 {
                seen[diff] = true;
                scores[diff] += score as u32
            }
            prev_score = score;
        }
    }

    *scores.iter().max().unwrap()
}

fn get_secret(secret: u64) -> u64 {
    let mut new_secret = ((secret << 6) ^ secret) % 16777216;
    new_secret = ((new_secret >> 5) ^ new_secret) % 16777216;
    ((new_secret << 11) ^ new_secret) % 16777216
}
