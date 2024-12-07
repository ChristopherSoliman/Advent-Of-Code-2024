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

    println!("{:?}", equations);

    let sum = equations
        .iter()
        .filter_map(|equation| {
            if is_solvable(&equation) {
                return Some(equation.total);
            }
            None
        })
        .sum();
    sum
}

fn is_solvable(eq: &Equation) -> bool {
    let n = eq.components.len();
    for i in 0..n {
        for j in 0..n {}
    }
    false
}
