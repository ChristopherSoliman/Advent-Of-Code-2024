use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

pub fn part2(path: &str) -> u32 {
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

    let initial_pos = pos.clone();

    let mut visited_directions: HashMap<Position, Vec<Direction>> = HashMap::new();
    visited_directions.insert(pos, vec![Direction::Up]);

    let mut count = 0;

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    loop {
        let starting_pos = pos.clone();
        match dir {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
        if pos.x >= width || pos.y >= height || pos.x < 0 || pos.y < 0 {
            break;
        }
        if map[pos.y as usize][pos.x as usize] == '#' {
            dir = dir.get_next_direction();
            pos = starting_pos;
        } else {
            if causes_loop(&pos, &dir.get_next_direction(), &map, &visited_directions) {
                count += 1;
            }
            match visited_directions.get_mut(&pos) {
                Some(d) => {
                    if !d.contains(&dir) {
                        d.push(dir);
                    }
                }
                None => {
                    visited_directions.insert(pos, vec![dir]);
                }
            }
        }
    }

    count
}

fn causes_loop(
    position: &Position,
    dir: &Direction,
    map: &Vec<Vec<char>>,
    visited_directions: &HashMap<Position, Vec<Direction>>,
) -> bool {
    let mut pos = position.clone();

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    loop {
        match dir {
            Direction::Up => pos.y -= 1,
            Direction::Down => pos.y += 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }

        if pos.x >= width
            || pos.y >= height
            || pos.x < 0
            || pos.y < 0
            || map[pos.y as usize][pos.x as usize] == '#'
        {
            return false;
        }
        if let Some(c) = visited_directions.get(&pos) {
            if c.contains(&dir) {
                return true;
            }
        }
    }
}
