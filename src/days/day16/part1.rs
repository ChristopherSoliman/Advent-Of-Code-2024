use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, Copy, PartialEq, Clone)]
struct Point {
    col: usize,
    row: usize,
}

const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut start = Point { col: 0, row: 0 };
    let mut end = Point { col: 0, row: 0 };

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut grid_line: Vec<char> = line.chars().collect();
            if let Some(col) = grid_line.iter().position(|c| *c == 'S') {
                start = Point { row, col };
                grid_line[col] = '.';
            }
            if let Some(col) = grid_line.iter().position(|c| *c == 'E') {
                end = Point { row, col };
                grid_line[col] = '.';
            }
            grid_line
        })
        .collect();

    find_path(&grid, &start, &end)
}

fn get_minimum(
    unvisited: &Vec<(Point, usize)>,
    distances: &HashMap<(Point, usize), u32>,
) -> Option<(usize, u32)> {
    let mut min_point: Option<usize> = None;
    let mut min = u32::MAX;
    for i in 0..unvisited.len() {
        let (point, dir) = unvisited[i];
        let distance = distances.get(&(point, dir)).unwrap();
        if *distance < min {
            min = *distance;
            min_point = Some(i);
        }
    }
    if let Some(mp) = min_point {
        return Some((mp, min));
    }
    None
}

fn find_path(grid: &Vec<Vec<char>>, start: &Point, end: &Point) -> u32 {
    let mut dist: HashMap<(Point, usize), u32> = HashMap::new();
    let mut q: Vec<(Point, usize)> = Vec::new();
    let mut seen: HashSet<(Point, usize)> = HashSet::new();

    q.push((*start, 0));
    dist.insert((*start, 0), 0);

    while let Some((i, current_dist)) = get_minimum(&q, &dist) {
        let (point, dir) = q.remove(i);
        seen.insert((point, dir));
        for i in 0..3 {
            let new_dir_i = (dir as i32 + i as i32 - 1).rem_euclid(4) as usize;
            let new_dir = DIRECTIONS[new_dir_i];
            let mut next = point;
            let mut new_dist = current_dist + 1000;

            if new_dir_i == dir {
                next = Point {
                    row: (point.row as i32 + new_dir.0 as i32) as usize,
                    col: (point.col as i32 + new_dir.1 as i32) as usize,
                };
                new_dist = current_dist + 1;
            }
            if grid[next.row][next.col] == '#' {
                continue;
            }
            if seen.contains(&(next, new_dir_i)) {
                continue;
            }
            if dist.get(&(next, new_dir_i)).or(Some(&u32::MAX)) > Some(&new_dist) {
                q.push((next, new_dir_i));
                dist.insert((next, new_dir_i), new_dist);
            }
        }
    }

    let mut min = u32::MAX;
    for i in 0..4 {
        let distance = dist.get(&(*end, i)).unwrap();
        if *distance < min {
            min = *distance;
        }
    }
    min
}
