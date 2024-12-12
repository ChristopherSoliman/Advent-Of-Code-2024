use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let width = map[0].len();
    let height = map.len();
    let mut seen = vec![false; width * height];
    let mut sum: u64 = 0;

    for i in 0..height {
        for j in 0..width {
            if seen[i * width + j] {
                continue;
            } else {
                seen[i * width + j] = true;
                sum += dfs_fence(&map, (i, j), &mut seen);
            }
        }
    }

    sum
}

fn dfs_fence(map: &Vec<Vec<char>>, root: (usize, usize), seen: &mut Vec<bool>) -> u64 {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let width = map[0].len();
    let height = map.len();

    let mut perimeter: u64 = 0;
    let mut area: u64 = 0;

    let region_type = map[root.0][root.1];

    q.push_back(root);

    while let Some((i, j)) = q.pop_back() {
        area += 1;
        for (dy, dx) in DIRECTIONS {
            let (ii, jj) = (i as i32 + dy, j as i32 + dx);

            if ii < 0 || jj < 0 || ii >= height as i32 || jj >= width as i32 {
                perimeter += 1;
                continue;
            }

            let (ii, jj) = (ii as usize, jj as usize);

            if map[ii][jj] == region_type {
                if seen[ii * width + jj] {
                    continue;
                }
                seen[ii * width + jj] = true;
                q.push_front((ii, jj));
            } else {
                perimeter += 1;
            }
        }
    }
    perimeter * area
}
