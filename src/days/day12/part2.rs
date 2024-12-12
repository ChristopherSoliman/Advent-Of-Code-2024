use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DIAGONALS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

pub fn part2(path: &str) -> u64 {
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

    let mut perimeters: Vec<(i32, i32)> = Vec::new();
    let mut area: u64 = 0;

    let region_type = map[root.0][root.1];

    q.push_back(root);

    while let Some((i, j)) = q.pop_back() {
        area += 1;
        for (di, dj) in DIRECTIONS {
            let (ii, jj) = (i as i32 + di, j as i32 + dj);

            if ii < 0 || jj < 0 || ii >= height as i32 || jj >= width as i32 {
                perimeters.push((ii, jj));
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
                perimeters.push((ii as i32, jj as i32));
            }
        }
    }

    let unique_perimeters = perimeters
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut corners: u32 = 0;

    for perim in &unique_perimeters {
        if unique_perimeters.contains(&(perim.0 + 1, perim.1))
            && unique_perimeters.contains(&(perim.0 - 1, perim.1))
        {
            continue;
        }
        if unique_perimeters.contains(&(perim.0, perim.1 + 1))
            && unique_perimeters.contains(&(perim.0, perim.1 - 1))
        {
            continue;
        }

        corners += 1;
    }

    //for i in 0..unique_perimeters.len() {
    //    for j in i + 1..unique_perimeters.len() {
    //        let (ii, jj) = unique_perimeters[i];
    //        for (di, dj) in DIAGONALS {
    //            if unique_perimeters[j] == (ii + di, jj + dj) {
    //                //i+di, j, i, j+dj
    //                let (ci, cj) = (ii as i32 + di, jj as i32);
    //                if ci >= 0
    //                    && ci < height as i32
    //                    && cj >= 0
    //                    && cj < width as i32
    //                    && map[ci as usize][cj as usize] == region_type
    //                {
    //                    corners += 1;
    //                    break;
    //                }
    //                let (ci, cj) = (ii as i32, jj as i32 + dj);
    //                if ci >= 0
    //                    && ci < height as i32
    //                    && cj >= 0
    //                    && cj < width as i32
    //                    && map[ci as usize][cj as usize] == region_type
    //                {
    //                    corners += 1;
    //                    break;
    //                }
    //            }
    //        }
    //    }
    //}
    println!("{}: {}", region_type, corners);
    corners as u64 * area
}
