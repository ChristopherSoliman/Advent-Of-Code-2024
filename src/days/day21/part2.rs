use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const ARROWS: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut sum = 0;
    let mut cache: HashMap<(char, char, u8), u64> = HashMap::new();
    for line in input.lines() {
        let num = line.strip_suffix("A").unwrap().parse::<u64>().unwrap();
        sum += num * get_code_length(&line, &mut cache);
    }
    sum
}

fn map_dir(dir: &(i8, i8)) -> char {
    match dir {
        (-1, 0) => '^',
        (1, 0) => 'v',
        (0, -1) => '<',
        (0, 1) => '>',
        _ => unreachable!(),
    }
}

fn get_code_length(code: &str, cache: &mut HashMap<(char, char, u8), u64>) -> u64 {
    let mut result = 0;
    let code = "A".to_string() + code;
    for i in 0..code.len() - 1 {
        let mut code_it = code.chars();
        let char1 = &code_it.nth(i).unwrap();
        let char2 = &code_it.next().unwrap();
        result += get_shortest(char1, char2, cache);
    }
    result
}

fn get_shortest(num1: &char, num2: &char, cache: &mut HashMap<(char, char, u8), u64>) -> u64 {
    let mut seen: HashSet<char> = HashSet::new();
    let mut q: Vec<(char, String)> = Vec::from([(*num1, "".to_string())]);
    let mut paths: HashMap<char, Vec<String>> = HashMap::new();

    paths.insert(*num1, vec![]);

    while !q.is_empty() {
        let (c, path) = q.remove(0);
        seen.insert(c);
        let mut c_pos = (0, 0);
        NUMPAD.iter().enumerate().for_each(|(i, r)| {
            if let Some(j) = r.iter().position(|x| *x == c) {
                c_pos = (i, j);
            }
        });
        for dir in DIRECTIONS {
            let (nr, nc) = (c_pos.0 as i32 + dir.0 as i32, c_pos.1 as i32 + dir.1 as i32);
            if nr < 0 || nr >= 4 || nc < 0 || nc >= 3 || NUMPAD[nr as usize][nc as usize] == ' ' {
                continue;
            }
            let new_c = NUMPAD[nr as usize][nc as usize];
            if seen.contains(&new_c) {
                continue;
            }

            let empty = Vec::new();
            let old_paths = paths.get(&new_c).or(Some(&empty)).unwrap();
            let new_path = path.clone() + &map_dir(&dir).to_string();
            if old_paths.len() == 0 || old_paths[0].len() > new_path.len() {
                paths.insert(new_c, vec![new_path.clone()]);
            } else if old_paths[0].len() < new_path.len() {
                continue;
            } else {
                paths.entry(new_c).and_modify(|v| v.push(new_path.clone()));
            }
            q.push((new_c, new_path));
        }
        q.sort_by_key(|v| v.1.len());
    }

    paths
        .get(&num2)
        .unwrap()
        .iter()
        .map(|v| get_lowest_score(v.to_owned() + "A", cache, 25))
        .min()
        .unwrap()
}

fn get_lowest_score(code: String, cache: &mut HashMap<(char, char, u8), u64>, level: u8) -> u64 {
    let mut result = 0;
    let code = "A".to_string() + &code;

    for i in 0..code.len() - 1 {
        let mut code_it = code.chars();
        let char1 = &code_it.nth(i).unwrap();
        let char2 = &code_it.next().unwrap();
        if let Some(res) = cache.get(&(*char1, *char2, level)) {
            result += res;
            continue;
        }
        if char1 == char2 {
            result += 1;
            continue;
        }
        let mut seen: HashSet<char> = HashSet::new();
        let mut q: Vec<(char, String)> = Vec::from([(*char1, "".to_string())]);
        let mut paths: HashMap<char, Vec<String>> = HashMap::new();

        paths.insert(*char1, vec![]);

        while !q.is_empty() {
            let (c, path) = q.remove(0);
            seen.insert(c);
            let mut c_pos = (0, 0);
            ARROWS.iter().enumerate().for_each(|(i, r)| {
                if let Some(j) = r.iter().position(|x| *x == c) {
                    c_pos = (i, j);
                }
            });
            for dir in DIRECTIONS {
                let (nr, nc) = (c_pos.0 as i32 + dir.0 as i32, c_pos.1 as i32 + dir.1 as i32);
                if nr < 0 || nr >= 2 || nc < 0 || nc >= 3 || ARROWS[nr as usize][nc as usize] == ' '
                {
                    continue;
                }
                let new_c = ARROWS[nr as usize][nc as usize];
                if seen.contains(&new_c) {
                    continue;
                }

                let empty = Vec::new();
                let old_paths = paths.get(&new_c).or(Some(&empty)).unwrap();
                let new_path = path.clone() + &map_dir(&dir).to_string();
                if old_paths.len() == 0 || old_paths[0].len() > new_path.len() {
                    paths.insert(new_c, vec![new_path.clone()]);
                } else if old_paths[0].len() < new_path.len() {
                    continue;
                } else {
                    paths.entry(new_c).and_modify(|v| v.push(new_path.clone()));
                }
                q.push((new_c, new_path));
            }
            q.sort_by_key(|v| v.1.len());
        }

        if level == 1 {
            if let Some(path) = paths.get(&char2) {
                result += path[0].len() as u64 + 1;
                cache.insert((*char1, *char2, level), path[0].len() as u64 + 1);
            }
        } else {
            let min = paths
                .get(&char2)
                .unwrap()
                .iter()
                .map(|v| get_lowest_score(v.to_owned() + "A", cache, level - 1))
                .min()
                .or(Some(0))
                .unwrap();
            cache.insert((*char1, *char2, level), min as u64);
            result += min;
        }
    }

    result
}
