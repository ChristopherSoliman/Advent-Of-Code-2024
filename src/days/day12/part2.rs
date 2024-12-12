use std::collections::{HashMap, VecDeque};

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

    let mut area: u64 = 0;
    let mut neighbours_map: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();

    let region_type = map[root.0][root.1];

    q.push_back(root);

    while let Some((i, j)) = q.pop_back() {
        area += 1;
        neighbours_map.entry((i, j)).or_insert(vec![]);
        for (di, dj) in DIRECTIONS {
            let (ii, jj) = (i as i32 + di, j as i32 + dj);

            if ii < 0 || jj < 0 || ii >= height as i32 || jj >= width as i32 {
                continue;
            }

            let (ii, jj) = (ii as usize, jj as usize);

            if map[ii][jj] == region_type {
                neighbours_map
                    .entry((i, j))
                    .and_modify(|v| v.push((di, dj)))
                    .or_insert(vec![(di, dj)]);
                if seen[ii * width + jj] {
                    continue;
                }
                seen[ii * width + jj] = true;
                q.push_front((ii, jj));
            }
        }
    }

    let corners = neighbours_map
        .iter()
        .map(|(plot, neighbours)| {
            match neighbours.len() {
                0 => 4,
                1 => 2,
                2 => {
                    // Opposites
                    if (neighbours.contains(&(0, -1)) && neighbours.contains(&(0, 1)))
                        || (neighbours.contains(&(-1, 0)) && neighbours.contains(&(1, 0)))
                    {
                        return 0;
                    }

                    // Corner
                    let (ci, cj) = (
                        plot.0 as i32 + neighbours.iter().map(|(i, _)| i).sum::<i32>(),
                        plot.1 as i32 + neighbours.iter().map(|(_, j)| j).sum::<i32>(),
                    );
                    if map[ci as usize][cj as usize] != region_type {
                        return 2;
                    }

                    1
                }
                3 => {
                    let mut c = 0;
                    for i in 0..neighbours.len() {
                        for j in i + 1..neighbours.len() {
                            let (ci, cj) = (
                                neighbours[i].0 + neighbours[j].0 + plot.0 as i32,
                                neighbours[i].1 + neighbours[j].1 + plot.1 as i32,
                            );
                            if (ci as usize, cj as usize) != *plot
                                && map[ci as usize][cj as usize] != region_type
                            {
                                c += 1;
                            }
                        }
                    }
                    c
                }
                _ => DIAGONALS
                    .iter()
                    .filter_map(|(di, dj)| {
                        let (ci, cj) = (plot.0 as i32 + di, plot.1 as i32 + dj);

                        if ci >= 0 && ci < height as i32 && cj >= 0 && cj < width as i32 {
                            if map[ci as usize][cj as usize] == region_type {
                                return None;
                            }
                        }
                        return Some(());
                    })
                    .count(),
            }
        })
        .sum::<usize>();
    corners as u64 * area
}
