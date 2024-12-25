pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();

    let mut lock;
    let mut heights;
    for schem in input.trim().split("\r\n\r\n").into_iter() {
        heights = [6; 5];
        lock = false;
        if schem.lines().next().unwrap() == "#####" {
            lock = true;
            heights = [0; 5];
        }
        for line in schem.lines() {
            line.chars().enumerate().for_each(|(i, char)| {
                if lock {
                    if char == '#' {
                        heights[i] += 1;
                    }
                } else {
                    if char != '#' {
                        heights[i] -= 1;
                    }
                }
            });
        }
        if lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut sum = 0;
    for lock in &locks {
        for key in &keys {
            if key.iter().zip(lock).all(|(k, l)| (k + l) < 7) {
                sum += 1;
            }
        }
    }
    sum
}
