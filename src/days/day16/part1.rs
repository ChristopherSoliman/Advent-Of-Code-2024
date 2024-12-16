use std::collections::{HashMap, HashSet, VecDeque};

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

    //let height = grid.len();
    //let width = grid[0].len();
    //let mut seen = vec![false; height * width * 4];
    //if let Some(s) = find_path_2(&grid, &start, &end, &dir) {
    //    return s;
    //}
    find_path_2(&grid, &start, &end)
    //find_path(&grid, &start, &end, &dir, None).unwrap()
}

fn get_minimum(
    unvisited: &VecDeque<(Point, usize)>,
    distances: &Vec<u32>,
    width: &usize,
) -> Option<usize> {
    let mut min_point: Option<usize> = None;
    let mut min = u32::MAX;
    for i in 0..unvisited.len() {
        let (point, dir) = unvisited[i];
        let hash = (point.row * width + point.col) * 4 + dir;
        let distance = distances[hash];
        if distance < min {
            min = distance;
            min_point = Some(i);
        }
    }
    min_point
}

fn find_path_3(grid: &Vec<Vec<char>>, start: &Point, end: &Point) -> u32 {
    let mut q: Vec<(Point, usize, u32)> = Vec::new();
    let mut seen: HashSet<(Point, usize)> = HashSet::new();

    q.push((*start, 0, 0));

    while let Some((point, dir, score)) = q.pop() {
        println!("{}", q.len());
        seen.insert((point, dir));
        if point == *end {
            return score;
        }
        for i in 0..3 {
            let new_dir_i = (dir as i32 + i as i32 - 1).rem_euclid(4) as usize;
            let new_dir = DIRECTIONS[new_dir_i];

            if new_dir_i == dir {
                let new_start = Point {
                    row: (point.row as i32 + new_dir.0 as i32) as usize,
                    col: (point.col as i32 + new_dir.1 as i32) as usize,
                };
                if grid[new_start.row][new_start.col] == '#' {
                    continue;
                }
                if seen.contains(&(new_start, dir)) {
                    continue;
                }
                q.push((new_start, dir, score + 1));
            } else {
                if seen.contains(&(point, new_dir_i)) {
                    continue;
                }
                q.push((point, new_dir_i, score + 1000));
            }
        }
    }
    0
}

fn find_path_2(grid: &Vec<Vec<char>>, start: &Point, end: &Point) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut distances: HashMap<(Point, usize), u32> = HashMap::new();
    let mut unvisited: VecDeque<(Point, usize)> = VecDeque::new();
    let mut distances = vec![u32::MAX; width * height * 4];

    for i in 0..height {
        for j in 0..width {
            let point = Point { row: i, col: j };
            for i in 0..4 {
                unvisited.push_back((point, i));
            }
        }
    }

    distances[(start.row * width + start.col) * 4] = 0;

    while let Some(i) = get_minimum(&unvisited, &distances, &width) {
        //println!("{}", unvisited.len());
        let (point, dir) = unvisited.remove(i).unwrap();
        for i in 0..3 {
            let new_dir_i = (dir as i32 + i as i32 - 1).rem_euclid(4) as usize;
            let new_dir = DIRECTIONS[new_dir_i];

            let hash = (point.row * width + point.col) * 4 + dir;
            let current_dist = distances[hash];

            if new_dir_i == dir {
                let new_start = Point {
                    row: (point.row as i32 + new_dir.0 as i32) as usize,
                    col: (point.col as i32 + new_dir.1 as i32) as usize,
                };
                if grid[new_start.row][new_start.col] == '#' {
                    continue;
                }

                if unvisited.contains(&(new_start, dir)) {
                    let new_dist = current_dist + 1;
                    let hash = (new_start.row * width + new_start.col) * 4 + dir;
                    if distances[hash] > new_dist {
                        distances[hash] = new_dist;
                    }
                }
            } else {
                if unvisited.contains(&(point, new_dir_i)) {
                    let new_dist = current_dist.clone() + 1000;
                    let hash = (point.row * width + point.col) * 4 + new_dir_i;
                    if distances[hash] > new_dist {
                        distances[hash] = new_dist;
                    }
                }
            }
        }
    }

    let mut min = u32::MAX;
    for i in 0..4 {
        let hash = (end.row * width + end.col) * 4 + i;
        let distance = distances[hash];
        if distance < min {
            min = distance;
        }
    }
    min
}

fn find_path(
    grid: &Vec<Vec<char>>,
    start: &Point,
    end: &Point,
    dir: &usize,
    seen: Option<&mut Vec<bool>>,
) -> Option<u32> {
    let height = grid.len();
    let width = grid[0].len();

    let mut sum: Vec<Option<u32>> = Vec::new();
    let seen = match seen {
        Some(v) => v,
        None => &mut vec![false; width * height],
    };

    for i in 0..3 {
        let new_dir_i = (*dir as i32 + i as i32 - 1).rem_euclid(4) as usize;
        let new_dir = DIRECTIONS[new_dir_i];
        let new_start = Point {
            row: (start.row as i32 + new_dir.0 as i32) as usize,
            col: (start.col as i32 + new_dir.1 as i32) as usize,
        };

        let hash = new_start.row * width + new_start.col;
        if seen[hash] {
            continue;
        }
        seen[hash] = true;

        if grid[new_start.row][new_start.col] == '#' {
            sum.push(None);
            continue;
        }
        if new_start == *end {
            sum.push(Some(1));
            continue;
        }
        if let Some(s) = find_path(grid, &new_start, end, &new_dir_i, Some(seen)) {
            if new_dir_i != *dir {
                sum.push(Some(1001 + s));
            } else {
                sum.push(Some(1 + s));
            }
        }
    }
    let sum = sum
        .iter()
        .filter_map(|s| {
            if s.is_some() {
                return Some(s.unwrap());
            }
            None
        })
        .collect::<Vec<_>>();
    sum.iter().min().copied()
}
