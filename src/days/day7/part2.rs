#[derive(Debug, Clone)]
struct Equation {
    pub components: Vec<u32>,
    pub total: u64,
}

pub fn part2(path: &str) -> u64 {
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

    let unsolvable_sum: u64 = equations
        .iter()
        .filter_map(|equation| {
            let total = equation.total.clone();
            if is_solvable(equation) {
                return Some(total);
            }
            None
        })
        .sum();

    unsolvable_sum
}

fn concat(a: &u64, b: &u64) -> u64 {
    let cb: i32 = 10;
    a * cb.pow(b.to_string().len() as u32) as u64 + b
}

//Non brute force
fn is_solvable(eq: &Equation) -> bool {
    if eq.components.len() == 1 {
        return eq.components[0] as u64 == eq.total;
    }

    let last = *eq.components.last().unwrap() as u64;
    if eq.total % last == 0 {
        let mut eq = eq.clone();
        eq.components.pop().unwrap();
        eq.total /= last;
        if is_solvable(&eq) {
            return true;
        }
    }
    if eq.total > last {
        let mut eq = eq.clone();
        eq.components.pop().unwrap();
        eq.total -= last;
        if is_solvable(&eq) {
            return true;
        }
    }

    let total_s = eq.total.to_string();
    let last_s = last.to_string();

    if total_s.len() > last_s.len() && total_s[(total_s.len() - last_s.len())..] == last_s {
        let mut eq = eq.clone();
        eq.components.pop().unwrap();
        eq.total = total_s[..(total_s.len() - last_s.len())].parse().unwrap();
        if is_solvable(&eq) {
            return true;
        }
    }

    false
}

//Brute force
fn is_solvable_force(eq: &Equation) -> bool {
    let b: i32 = 2;
    let n = b.pow(eq.components.len() as u32 - 1);

    for i in 0..n {
        for k in 0..n {
            let mut sum: u64 = eq.components[0] as u64;
            for j in 0..eq.components.len() - 1 {
                match i & (1 << j) != 0 {
                    false => match k & (1 << j) != 0 {
                        true => sum *= eq.components[j + 1] as u64,
                        false => sum += eq.components[j + 1] as u64,
                    },
                    true => sum = concat(&sum, &(eq.components[j + 1] as u64)),
                }
            }
            if sum == eq.total {
                return true;
            }
        }
    }
    false
}
