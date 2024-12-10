use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut heads: Vec<(usize, usize)> = Vec::new();

    let map: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        heads.push((i, j));
                    }
                    c.to_digit(10).expect("Couldn't parse number") as u8
                })
                .collect()
        })
        .collect();

    heads.iter().map(|head| bfs(&head, &map)).sum()
}

fn bfs(start: &(usize, usize), map: &Vec<Vec<u8>>) -> u64 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut summits: Vec<(usize, usize)> = Vec::new();

    queue.push_back(*start);

    while let Some(curr) = queue.pop_back() {
        for dir in DIRECTIONS.iter() {
            let next = (curr.0 as i32 + dir.0, curr.1 as i32 + dir.1);
            if next.0 < 0
                || next.0 >= map.len() as i32
                || next.1 < 0
                || next.1 >= map[0].len() as i32
            {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);

            if map[next.0][next.1] != map[curr.0][curr.1] + 1 {
                continue;
            }
            if map[next.0][next.1] == 9 {
                summits.push(next);
            } else {
                queue.push_front(next);
            }
        }
    }
    summits.len() as u64
}
