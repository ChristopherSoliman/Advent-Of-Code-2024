use std::cmp::Ordering;

pub fn part2(path: &str) -> u64 {
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
            let grid_line: Vec<char> = line.chars().collect();
            if let Some(j) = grid_line.iter().position(|c| *c == '@') {
                pos = (i, j * 2);
            }
            grid.push(
                grid_line
                    .iter()
                    .flat_map(|c| match c {
                        '#' => ['#', '#'],
                        '.' => ['.', '.'],
                        'O' => ['[', ']'],
                        '@' => ['.', '.'],
                        _ => panic!("invalid grid character"),
                    })
                    .collect(),
            );
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

    let height = grid.len();
    let width = grid[0].len();

    for movement in movements {
        pos = move_robot(&mut grid, &pos, movement);
    }

    let mut sum: u64 = 0;

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '[' {
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
    let mut pushed: Vec<(usize, usize)>;
    let new_pos = (
        (pos.0 as i32 + movement.0 as i32) as usize,
        (pos.1 as i32 + movement.1 as i32) as usize,
    );

    match grid[new_pos.0][new_pos.1] {
        '#' => return *pos,
        '.' => return new_pos,
        '[' | ']' => {
            if let Some(b) = get_boxes(grid, &new_pos, movement) {
                pushed = b;
            } else {
                return *pos;
            }
        }
        _ => unreachable!("hit invalid character"),
    }

    pushed.sort_by(|a, b| compare(a, b, &movement));

    for (i, j) in pushed {
        let (ii, jj) = (
            (i as i32 + movement.0 as i32) as usize,
            (j as i32 + movement.1 as i32) as usize,
        );

        grid[i][j] = '.';
        grid[i][j + 1] = '.';
        grid[ii][jj] = '[';
        grid[ii][jj + 1] = ']';
    }
    new_pos
}

fn compare(a: &(usize, usize), b: &(usize, usize), movement: &(i8, i8)) -> Ordering {
    match movement {
        (_, 1) => b.1.cmp(&a.1),
        (_, -1) => a.1.cmp(&b.1),
        (1, _) => b.0.cmp(&a.0),
        (-1, _) => a.0.cmp(&b.0),
        _ => unreachable!("invalid direction"),
    }
}

fn get_boxes(
    grid: &mut Vec<Vec<char>>,
    box_pos: &(usize, usize),
    movement: (i8, i8),
) -> Option<Vec<(usize, usize)>> {
    // Always refer to box from `[`
    let box_pos = match grid[box_pos.0][box_pos.1] {
        '[' => *box_pos,
        ']' => (box_pos.0, box_pos.1 - 1),
        other => panic!("expected a box ('[' or ']') but got:'{}'", other),
    };

    let mut boxes: Vec<(usize, usize)> = vec![box_pos];

    let i = (box_pos.0 as i32 + movement.0 as i32) as usize;
    let j = match movement.1 {
        1 => (box_pos.1 as i32 + movement.1 as i32 + 1) as usize,
        _ => (box_pos.1 as i32 + movement.1 as i32) as usize,
    };

    match movement.0 {
        0 if grid[i][j] == '.' => return Some(boxes),
        0 if grid[i][j] == '#' => return None,
        _ if grid[i][j] == '.' && grid[i][j + 1] == '.' => return Some(boxes),
        _ if grid[i][j] == '#' || grid[i][j + 1] == '#' => return None,
        _ => {}
    }

    // This check could only be false if moving vertically
    if grid[i][j] != '.' {
        if let Some(mut b) = get_boxes(grid, &(i, j), movement) {
            boxes.append(&mut b);
        } else {
            return None;
        }
    }

    // If moving vertically, check the other side of the box
    if movement.0 != 0 {
        if grid[i][j + 1] != '.' {
            if let Some(mut b) = get_boxes(grid, &(i, j + 1), movement) {
                boxes.append(&mut b);
            } else {
                return None;
            }
        }
    }

    Some(boxes)
}
