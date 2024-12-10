#[derive(Debug)]
struct Equation {
    pub components: Vec<u32>,
    pub total: u64,
}

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let equations: Vec<Equation> = input
        .trim()
        .lines()
        .map(|line| {
            let mut split_line = line.split(':');
            let total = split_line
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Couldn't parse value");
            let components = split_line
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|c| c.parse().expect("Couldn't parse component"))
                .collect();
            Equation { total, components }
        })
        .collect();

    let sum = equations
        .iter()
        .filter_map(|equation| {
            if is_solvable(equation) {
                return Some(equation.total);
            }
            None
        })
        .sum();
    sum
}

fn is_solvable(eq: &Equation) -> bool {
    let b: i32 = 2;
    let n = b.pow(eq.components.len() as u32 - 1);

    for i in 0..n {
        let mut sum: u64 = eq.components[0] as u64;
        for j in 0..eq.components.len() - 1 {
            match i & (1 << j) != 0 {
                true => sum *= eq.components[j + 1] as u64,
                false => sum += eq.components[j + 1] as u64,
            }
        }
        if sum == eq.total {
            return true;
        }
    }
    false
}
