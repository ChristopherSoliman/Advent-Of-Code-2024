use std::collections::HashSet;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const PROCESS: usize = 1024;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let grid = [true; WIDTH];
    let mut grid = [grid; HEIGHT];

    let mut lines = input.trim().lines();
    let mut i = 0;

    loop {
        if i >= PROCESS {
            break;
        }
        let (x, y) = lines.next().unwrap().split_once(',').unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        grid[y][x] = false;
        i += 1;
    }

    let target = (HEIGHT - 1, WIDTH - 1);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut q = Vec::from([(0, 0, 0)]);

    while !q.is_empty() {
        let (cy, cx, steps) = q.remove(0);
        if (cy, cx) == target {
            return steps;
        }
        if !seen.insert((cy, cx)) {
            continue;
        }
        for (dy, dx) in DIRS {
            let (ny, nx) = (cy as i32 + dy as i32, cx as i32 + dx as i32);
            if ny < 0 || ny >= HEIGHT as i32 || nx < 0 || nx >= WIDTH as i32 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if grid[ny][nx] {
                q.push((ny, nx, steps + 1));
            }
        }
    }
    0
}
