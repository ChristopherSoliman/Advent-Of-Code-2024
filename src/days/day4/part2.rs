pub fn part2(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();

    let mut sum: u32 = 0;
    for i in 1..chars.len() - 1 {
        for j in 1..chars[i].len() - 1 {
            if chars[i][j] == 'A' {
                if different_values(&chars, i, j) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn different_values(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    ((chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S')
        || (chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M'))
        && ((chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S')
            || (chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M'))
}
