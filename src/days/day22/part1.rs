pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut sum = 0;
    for line in input.lines() {
        sum += get_secret(line.parse::<u64>().unwrap(), 2000);
    }
    sum
}

fn get_secret(secret: u64, iter: u32) -> u64 {
    let mut new_secret = ((secret * 64) ^ secret) % 16777216;
    new_secret = ((new_secret / 32) ^ new_secret) % 16777216;
    new_secret = ((new_secret * 2048) ^ new_secret) % 16777216;
    if iter == 1 {
        return new_secret;
    }
    return get_secret(new_secret, iter - 1);
}
