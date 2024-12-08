pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();
    let mut antinodes = vec![false; width * height];

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '.' {
                continue;
            }
            find_antinodes(&i, &j, &grid, &mut antinodes);
        }
    }
    antinodes.iter().filter(|a| **a).count() as u32
}

fn find_antinodes(
    start_row: &usize,
    start_col: &usize,
    grid: &Vec<Vec<char>>,
    antinodes: &mut Vec<bool>,
) {
    let width = grid[0].len();
    let height = grid.len();

    let f = grid[*start_row][*start_col];

    for i in *start_row..height {
        for j in 0..width {
            if grid[i][j] != f || (j == *start_col && i == *start_row) {
                continue;
            }

            let di = i as i32 - *start_row as i32;
            let dj = j as i32 - *start_col as i32;

            let ai = *start_row as i32 - di;
            let aj = *start_col as i32 - dj;

            if ai >= 0 && aj >= 0 && aj < width as i32 {
                antinodes[(ai * width as i32 + aj) as usize] = true;
            }

            let ai = i as i32 + di;
            let aj = j as i32 + dj;

            if ai < height as i32 && aj >= 0 && aj < width as i32 {
                antinodes[(ai * width as i32 + aj) as usize] = true;
            }
        }
    }
}
