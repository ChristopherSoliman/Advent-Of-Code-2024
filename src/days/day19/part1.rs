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
    let mut cache: HashMap<&str, bool> = HashMap::new();
    designs
        .iter()
        .filter(|d| is_possible(d, &towels, &mut cache))
        .count() as u32
}

fn is_possible<'a>(
    target: &'a str,
    towels: &[[Vec<&str>; MAX_TOWEL_LENGTH]; 5],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if cache.get(&target).is_none() {
        if target == "" {
            return true;
        }
        let s_char = target.chars().next().unwrap();
        let length = std::cmp::min(target.len(), MAX_TOWEL_LENGTH);
        let mut result = false;
        for towel in towels[index(&s_char)][length - 1].to_owned() {
            if target.starts_with(towel) {
                if is_possible(&target[towel.len()..], towels, cache) {
                    result = true;
                    break;
                }
            }
        }
        cache.insert(target, result);
    }
    *cache.get(&target).unwrap()
}
