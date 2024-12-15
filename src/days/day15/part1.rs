pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<(i8, i8)> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut grid_flag = true;

    input.trim().lines().enumerate().for_each(|(i, line)| {
        if line.is_empty() {
            grid_flag = false;
            return;
        }

        if grid_flag {
            let mut grid_line: Vec<char> = line.chars().collect();
            if let Some(j) = grid_line.iter().position(|c| *c == '@') {
                pos = (i, j);
                grid_line[j] = '.';
            }
            grid.push(grid_line);
        } else {
            line.chars().for_each(|c| {
                movements.push(match c {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    '<' => (0, -1),
                    'v' => (1, 0),
                    _ => panic!("invalid direction"),
                });
            });
        }
    });

    for movement in movements {
        pos = move_robot(&mut grid, &pos, movement);
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut sum: u64 = 0;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'O' {
                sum += 100 * i as u64 + j as u64;
            }
        }
    }

    sum
}

fn move_robot(
    grid: &mut Vec<Vec<char>>,
    pos: &(usize, usize),
    movement: (i8, i8),
) -> (usize, usize) {
    let mut pushed: Vec<(usize, usize)> = Vec::new();
    let mut cursor = (
        (pos.0 as i32 + movement.0 as i32) as usize,
        (pos.1 as i32 + movement.1 as i32) as usize,
    );
    pushed.push(cursor);

    while grid[cursor.0][cursor.1] != '.' {
        if grid[cursor.0][cursor.1] == '#' {
            return pos.clone();
        }
        cursor = (
            (cursor.0 as i32 + movement.0 as i32) as usize,
            (cursor.1 as i32 + movement.1 as i32) as usize,
        );
        pushed.push(cursor);
    }
    grid[pos.0][pos.1] = '.';

    let first = pushed.first().unwrap();
    grid[first.0][first.1] = '.';

    if pushed.len() > 1 {
        let last = pushed.last().unwrap();
        grid[last.0][last.1] = 'O';
    }

    *first
}
