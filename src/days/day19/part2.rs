use std::collections::HashMap;

const MAX_TOWEL_LENGTH: usize = 10;

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
pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let (towels_in, designs) = input.split_once("\r\n\r\n").unwrap();

    let mut towels: [[Vec<&str>; MAX_TOWEL_LENGTH]; 5] = Default::default();

    towels_in.lines().for_each(|line| {
        let ts = line.split(", ");
        for t in ts {
            let c = t.chars().next().unwrap();
            for i in t.len()..MAX_TOWEL_LENGTH {
                towels[index(&c)][i].push(t)
            }
        }
    });

    let designs = designs.lines().collect::<Vec<_>>();
    let mut cache: HashMap<String, u64> = HashMap::new();
    designs
        .iter()
        .map(|d| dfs(d, &towels, &mut cache))
        .sum::<u64>() as u64
}

fn dfs(target: &str, towels: &[[Vec<&str>; 10]; 5], cache: &mut HashMap<String, u64>) -> u64 {
    let s_char = target.chars().next().unwrap();
    let length = std::cmp::min(target.len(), MAX_TOWEL_LENGTH - 1);
    let mut q = towels[index(&s_char)][length].to_owned();
    let mut count = 0;

    while !q.is_empty() {
        let towel = q.pop().unwrap();
        if *target == *towel {
            count += 1;
            continue;
        }
        if target[0..towel.len()] != *towel || target.len() == towel.len() {
            continue;
        }

        let new_target = &target[towel.len()..];
        if let Some(result) = cache.get(&new_target.to_string()) {
            count += result;
            continue;
        }
        let new_count = dfs(new_target, towels, cache);
        if new_count > 0 {
            count += new_count;
        }
        cache.insert(new_target.to_string(), new_count);
    }
    count
}
