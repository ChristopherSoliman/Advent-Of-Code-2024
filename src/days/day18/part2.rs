use std::collections::HashSet;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part2(path: &str) -> String {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let target = (HEIGHT - 1, WIDTH - 1);
    let lines = input.trim().lines().collect::<Vec<_>>();

    let mut range = (0, lines.len());

    while range.0 < range.1 - 1 {
        let grid = [true; WIDTH];
        let mut grid = [grid; HEIGHT];
        for i in 0..(range.0 + range.1) / 2 {
            let (x, y) = lines[i].split_once(',').unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            grid[y][x] = false;
        }
        if bfs(&grid, &target) {
            range.0 = (range.0 + range.1) / 2;
        } else {
            range.1 = (range.0 + range.1) / 2;
        }
    }
    lines[range.0].to_string()
}

fn bfs(grid: &[[bool; WIDTH]; HEIGHT], target: &(usize, usize)) -> bool {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut q: Vec<(usize, usize)> = Vec::from([(0, 0)]);

    while !q.is_empty() {
        let (cy, cx) = q.remove(0);
        if (cy, cx) == *target {
            return true;
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
                q.push((ny, nx));
            }
        }
    }
    false
}
