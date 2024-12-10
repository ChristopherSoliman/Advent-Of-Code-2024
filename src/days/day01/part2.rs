use super::read_input;

pub fn part2() -> i32 {
    let (left, right) = read_input("inputs/day1.txt");
    let score: i32 = left
        .iter()
        .map(|l| {
            let right_filtered = right
                .iter()
                .filter(|&&r| r == *l)
                .cloned()
                .collect::<Vec<i32>>();
            l * right_filtered.len() as i32
        })
        .sum();
    score
}
