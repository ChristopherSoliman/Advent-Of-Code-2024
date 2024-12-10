#![allow(dead_code)]
const DIRECTIONS: [(i64, i64); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    let mut sum: u32 = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == 'X' {
                sum += find_words(&chars, i, j) as u32
            }
        }
    }
    sum
}

fn find_words(chars: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let i_s = i64::try_from(i).unwrap();
    let j_s = i64::try_from(j).unwrap();

    DIRECTIONS
        .iter()
        .filter_map(|dir| {
            if i_s + 3 * dir.0 < chars.len().try_into().unwrap()
                && i_s + 3 * dir.0 >= 0
                && j_s + 3 * dir.1 < chars[i].len().try_into().unwrap()
                && j_s + 3 * dir.1 >= 0
            {
                if chars[(i_s + dir.0) as usize][(j_s + dir.1) as usize] == 'M'
                    && chars[(i_s + 2 * dir.0) as usize][(j_s + 2 * dir.1) as usize] == 'A'
                    && chars[(i_s + 3 * dir.0) as usize][(j_s + 3 * dir.1) as usize] == 'S'
                {
                    return Some(dir);
                }
            }
            None
        })
        .count()
}
