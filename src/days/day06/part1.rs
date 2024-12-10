enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn get_next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct Position {
    pub x: isize,
    pub y: isize,
}

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut pos = Position { x: 0, y: 0 };
    let mut dir = Direction::Up;

    let map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            if let Some(c) = line.chars().position(|c| c == '^') {
                pos.x = c as isize;
                pos.y = idx as isize;
            }
            line.chars().collect()
        })
        .collect();

    let visited = vec![false; map[0].len()];
    let mut visited = vec![visited; map.len()];
    visited[pos.y as usize][pos.x as usize] = true;

    loop {
        let starting_pos = pos.clone();
        match dir {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
        if pos.x as usize >= map[0].len() || pos.y as usize >= map.len() || pos.x < 0 || pos.y < 0 {
            break;
        }
        if map[pos.y as usize][pos.x as usize] == '#' {
            dir = dir.get_next_direction();
            pos = starting_pos;
        } else {
            visited[pos.y as usize][pos.x as usize] = true;
        }
    }

    visited
        .iter()
        .map(|col| col.iter().filter(|v| **v).count() as u32)
        .sum()
}
