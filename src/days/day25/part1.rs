pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();

    let mut lock;
    let mut heights;
    for schem in input.trim().split("\r\n\r\n").into_iter() {
        heights = [0; 5];
        lock = false;
        if schem.lines().next().unwrap() == "#####" {
            lock = true;
        }
        for line in schem.lines() {
            line.chars().enumerate().for_each(|(i, char)| {
                if char == '#' {
                    heights[i] += 1;
                }
            });
        }
        if lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    keys.iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| k.iter().zip(*l).all(|(k, l)| k + l < 8))
                .count() as u32
        })
        .sum::<u32>()
}
