use std::collections::HashMap;

const MAX_TOWEL_LENGTH: usize = 8;

pub fn index(char: &char) -> usize {
    match char {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => unreachable!("invalid color"),
    }
}
pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let (towels_in, designs) = input.split_once("\r\n\r\n").unwrap();

    let mut towels: [[Vec<&str>; MAX_TOWEL_LENGTH]; 5] = Default::default();

    towels_in.lines().for_each(|line| {
        let ts = line.split(", ");
        for t in ts {
            let c = t.chars().next().unwrap();
            for i in (t.len() - 1)..MAX_TOWEL_LENGTH {
                towels[index(&c)][i].push(t)
            }
        }
    });

    let designs = designs.lines().collect::<Vec<_>>();
    let mut cache: HashMap<String, bool> = HashMap::new();
    designs
        .iter()
        .filter(|d| dfs(d, &towels, &mut cache))
        .count() as u32
}

fn dfs(
    target: &str,
    towels: &[[Vec<&str>; MAX_TOWEL_LENGTH]; 5],
    cache: &mut HashMap<String, bool>,
) -> bool {
    let s_char = target.chars().next().unwrap();
    let length = std::cmp::min(target.len(), MAX_TOWEL_LENGTH);
    let mut q = towels[index(&s_char)][length - 1].to_owned();

    while !q.is_empty() {
        let towel = q.pop().unwrap();
        if *target == *towel {
            return true;
        }
        if target[0..towel.len()] != *towel || target.len() == towel.len() {
            continue;
        }

        let new_target = &target[towel.len()..];
        if let Some(result) = cache.get(&new_target.to_string()) {
            if *result {
                return true;
            }
            continue;
        }
        if dfs(new_target, towels, cache) {
            cache.insert(new_target.to_string(), true);
            return true;
        }
        cache.insert(new_target.to_string(), false);
    }
    false
}
