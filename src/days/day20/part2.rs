use std::{
    collections::{HashMap, HashSet},
    usize,
};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const TIME_SAVED: usize = 100;

pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut start = Point { row: 0, col: 0 };
    let mut end = Point { row: 0, col: 0 };

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

    let base_path = base_path(&grid, &start, &end);
    get_shortcuts(&base_path)
}

fn base_path(grid: &Vec<Vec<char>>, source: &Point, target: &Point) -> Vec<Point> {
    let height = grid.len();
    let width = grid[0].len();

    let mut prev: HashMap<Point, Point> = HashMap::new();
    let mut q: Vec<(Point, u32)> = Vec::new();
    let mut seen: HashSet<Point> = HashSet::new();

    q.push((*source, 0));

    while !q.is_empty() {
        let (point, current_time) = q.remove(0);
        seen.insert(point);
        for dir in DIRECTIONS {
            let (nr, nc) = (
                point.row as i32 + dir.0 as i32,
                point.col as i32 + dir.1 as i32,
            );
            if nc < 0 || nc >= width as i32 || nr < 0 || nr >= height as i32 {
                continue;
            }

            let next = Point {
                row: nr as usize,
                col: nc as usize,
            };

            if grid[next.row][next.col] == '#' {
                continue;
            }
            if seen.contains(&next) {
                continue;
            }

            q.push((next, current_time + 1));
            prev.insert(next, point);
        }
    }

    let mut cp = target;
    let mut path: Vec<Point> = Vec::from([*target]);
    while let Some(pp) = prev.get(cp) {
        path.push(*pp);
        cp = pp
    }
    path.reverse();
    path
}

fn get_manhattan_neighbours(path: &Vec<Point>, pos: &usize) -> u32 {
    path.iter()
        .enumerate()
        .skip(*pos + 1)
        .filter_map(|(j, point)| {
            let source = path[*pos];
            let dist = point.col.abs_diff(source.col) + point.row.abs_diff(source.row);
            if dist <= 20 {
                if j >= TIME_SAVED + pos + dist {
                    return Some(());
                }
            }
            None
        })
        .count() as u32
}

fn get_shortcuts(base_path: &Vec<Point>) -> u32 {
    base_path
        .iter()
        .enumerate()
        .map(|(i, _)| get_manhattan_neighbours(base_path, &i))
        .sum::<u32>()
}
