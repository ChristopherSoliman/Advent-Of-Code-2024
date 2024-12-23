use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let numpad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let arrows = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    input
        .lines()
        .map(|line| {
            (
                line,
                line.strip_suffix("A").unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold(0, |acc, (line, num)| {
            acc + num * get_lowest_score(&line, 3, true, &numpad, &arrows)
        })
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
fn get_lowest_score(
    code: &str,
    level: u8,
    first: bool,
    numpad: &Vec<Vec<char>>,
    arrows: &Vec<Vec<char>>,
) -> u32 {
    let mut result = 0;
    let code = "A".to_string() + &code;

    let keypad = match first {
        true => numpad,
        false => arrows,
    };

    let height = keypad.len();

    for i in 0..code.len() - 1 {
        let mut code_it = code.chars();
        let char1 = &code_it.nth(i).unwrap();
        let char2 = &code_it.next().unwrap();
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
            keypad.iter().enumerate().for_each(|(i, r)| {
                if let Some(j) = r.iter().position(|x| *x == c) {
                    c_pos = (i, j);
                }
            });
            for dir in DIRECTIONS {
                let (nr, nc) = (c_pos.0 as i32 + dir.0 as i32, c_pos.1 as i32 + dir.1 as i32);
                if nr < 0
                    || nr >= height as i32
                    || nc < 0
                    || nc >= 3
                    || keypad[nr as usize][nc as usize] == ' '
                {
                    continue;
                }
                let new_c = keypad[nr as usize][nc as usize];
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
        }

        result += paths
            .get(&char2)
            .unwrap()
            .iter()
            .map(|v| {
                if level == 1 {
                    return v.len() as u32 + 1;
                }
                get_lowest_score(&(v.to_owned() + "A")[..], level - 1, false, numpad, arrows)
            })
            .min()
            .or(Some(0))
            .unwrap();
    }

    result
}
