const WIDTH: u8 = 101;
const HEIGHT: u8 = 103;
const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

struct Robot {
    x0: usize,
    y0: usize,
    vx: i32,
    vy: i32,
}

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut robots: Vec<Robot> = Vec::new();

    let grid: Vec<i64> = vec![-1; WIDTH as usize];
    let mut grid = vec![grid; HEIGHT as usize];
    let mut current_time = 0;

    input.lines().for_each(|line| {
        let mut line = line.split_whitespace();
        let (x0, y0) = line
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .split_once(',')
            .unwrap();

        let (vx, vy) = line
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .split_once(',')
            .unwrap();

        let (x0, y0): (usize, usize) = (x0.parse().unwrap(), y0.parse().unwrap());
        let (vx, vy): (i32, i32) = (vx.parse().unwrap(), vy.parse().unwrap());

        robots.push(Robot { x0, y0, vx, vy });
    });

    let mut found = false;
    while !found {
        let seen = vec![false; WIDTH as usize];
        let mut seen = vec![seen; HEIGHT as usize];

        for robot in &robots {
            let x_f = (robot.x0 as i32 + robot.vx * current_time).rem_euclid(WIDTH as i32);
            let y_f = (robot.y0 as i32 + robot.vy * current_time).rem_euclid(HEIGHT as i32);

            grid[y_f as usize][x_f as usize] = current_time as i64;
        }

        for i in 0..HEIGHT as usize {
            if found {
                break;
            }
            for j in 0..WIDTH as usize {
                if !seen[i][j] && grid[i][j] == current_time.into() {
                    seen[i][j] = true;
                    if bfs(&(i, j), &grid, current_time as u64, &mut seen) >= 50 {
                        println!("============================================================");
                        println!("Time: {current_time}");
                        for i in 0..HEIGHT {
                            for j in 0..WIDTH {
                                if grid[i as usize][j as usize] == current_time as i64 {
                                    print!("X")
                                } else {
                                    print!(" ")
                                }
                            }
                            print!("\n");
                        }
                        found = true;
                        break;
                    }
                }
            }
        }
        current_time += 1;
    }

    current_time as u64 - 1
}

fn bfs(
    pos: &(usize, usize),
    grid: &Vec<Vec<i64>>,
    current_time: u64,
    seen: &mut Vec<Vec<bool>>,
) -> u32 {
    let mut sum: u32 = 1;
    for (di, dj) in DIRECTIONS {
        let (i, j) = (pos.0 as i32 + di as i32, pos.1 as i32 + dj as i32);
        if i >= 0
            && i < HEIGHT as i32
            && j >= 0
            && j < WIDTH as i32
            && !seen[i as usize][j as usize]
            && grid[i as usize][j as usize] == current_time as i64
        {
            let (i, j) = (i as usize, j as usize);
            seen[i][j] = true;
            sum += bfs(&(i, j), &grid, current_time, seen);
        }
    }
    sum
}
