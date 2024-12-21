const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1(path: &str) -> u32 {
    let mut numpad: [[char; 3]; 4] = [['#'; 3]; 4];

    for i in 0..3 {
        for j in 0..3 {
            numpad[i][j] = std::char::from_digit((7 - 3 * i + j) as u32, 10).unwrap();
        }
    }
    numpad[3][1] = '0';
    numpad[3][2] = 'A';

    let arrows: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

    let input = std::fs::read_to_string(path).expect("File should be there");
    let codes: Vec<&str> = input.trim().lines().map(|line| line).collect();
    let mut commands_1: Vec<String> = Vec::new();
    let mut commands_2: Vec<String> = Vec::new();
    let mut commands_3: Vec<String> = Vec::new();

    for code in &codes {
        let code = "A".to_string() + code;
        let mut command = String::new();
        for i in 0..code.len() - 1 {
            let mut chars = code.chars();
            command.push_str(&get_path(
                &numpad,
                &chars.nth(i).unwrap(),
                &chars.next().unwrap(),
            ));
            command += "A";
        }
        println!("{}", &command);
        commands_1.push(command);
    }

    for code in &commands_1 {
        let code = "A".to_string() + code;
        let mut command = String::new();
        for i in 0..code.len() - 1 {
            let mut chars = code.chars();
            command.push_str(&get_arrow_path_1(
                &arrows,
                &chars.nth(i).unwrap(),
                &chars.next().unwrap(),
            ));
            command += "A";
        }
        println!("{}", &command);
        commands_2.push(command);
    }
    for code in &commands_2 {
        let code = "A".to_string() + code;
        let mut command = String::new();
        for i in 0..code.len() - 1 {
            let mut chars = code.chars();
            command.push_str(&get_arrow_path_2(
                &arrows,
                &chars.nth(i).unwrap(),
                &chars.next().unwrap(),
            ));
            command += "A";
        }
        println!("{}", &command);
        println!("{}", &command.len());
        commands_3.push(command);
    }

    let mut sum = 0;
    for i in 0..codes.len() {
        let code = codes[i];
        let num = &code[0..codes.len() - 2].parse::<u32>().unwrap();
        sum += num * commands_3[i].len() as u32;
    }
    sum
}
fn get_arrow_path_1(arrows: &[[char; 3]; 2], source: &char, target: &char) -> String {
    let mut commands = String::new();
    let mut s_pos: (usize, usize) = (3, 0);
    let mut t_pos: (usize, usize) = (3, 0);
    arrows.iter().enumerate().for_each(|(i, r)| {
        if let Some(j) = r.iter().position(|c| c == source) {
            s_pos = (i, j);
        }
        if let Some(j) = r.iter().position(|c| c == target) {
            t_pos = (i, j);
        }
    });

    let mut cp = s_pos;

    while cp != t_pos {
        let dist = (cp.0 as i32 - t_pos.0 as i32, cp.1 as i32 - t_pos.1 as i32);
        if dist.1 > 0 {
            let n_pos = (cp.0, cp.1 - 1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('<');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 < 0 {
            let n_pos = (cp.0 + 1, cp.1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('v');
                cp = n_pos;
                continue;
            }
        }
        if dist.1 < 0 {
            let n_pos = (cp.0, cp.1 + 1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('>');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 > 0 {
            let n_pos = (cp.0 - 1, cp.1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('^');
                cp = n_pos;
                continue;
            }
        }
    }
    commands
}
fn get_arrow_path_2(arrows: &[[char; 3]; 2], source: &char, target: &char) -> String {
    let mut commands = String::new();
    let mut s_pos: (usize, usize) = (3, 0);
    let mut t_pos: (usize, usize) = (3, 0);
    arrows.iter().enumerate().for_each(|(i, r)| {
        if let Some(j) = r.iter().position(|c| c == source) {
            s_pos = (i, j);
        }
        if let Some(j) = r.iter().position(|c| c == target) {
            t_pos = (i, j);
        }
    });

    let mut cp = s_pos;

    while cp != t_pos {
        let dist = (cp.0 as i32 - t_pos.0 as i32, cp.1 as i32 - t_pos.1 as i32);
        if dist.1 > 0 {
            let n_pos = (cp.0, cp.1 - 1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('<');
                cp = n_pos;
                continue;
            }
        }
        if dist.1 < 0 {
            let n_pos = (cp.0, cp.1 + 1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('>');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 < 0 {
            let n_pos = (cp.0 + 1, cp.1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('v');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 > 0 {
            let n_pos = (cp.0 - 1, cp.1);
            if arrows[n_pos.0][n_pos.1] != '#' {
                commands.push('^');
                cp = n_pos;
                continue;
            }
        }
    }
    commands
}

fn get_path(numpad: &[[char; 3]; 4], source: &char, target: &char) -> String {
    let mut commands = String::new();
    let mut s_pos: (usize, usize) = (3, 0);
    let mut t_pos: (usize, usize) = (3, 0);
    numpad.iter().enumerate().for_each(|(i, r)| {
        if let Some(j) = r.iter().position(|c| c == source) {
            s_pos = (i, j);
        }
        if let Some(j) = r.iter().position(|c| c == target) {
            t_pos = (i, j);
        }
    });

    let mut cp = s_pos;

    while cp != t_pos {
        let dist = (cp.0 as i32 - t_pos.0 as i32, cp.1 as i32 - t_pos.1 as i32);
        match commands.chars().last() {
            Some(c) => {
                let mut n_pos = (cp.0 as i32, cp.1 as i32);
                match c {
                    '^' => n_pos = (n_pos.0 - 1, n_pos.1),
                    '>' => n_pos = (n_pos.0, n_pos.1 + 1),
                    'v' => n_pos = (n_pos.0 + 1, n_pos.1),
                    '<' => n_pos = (n_pos.0, n_pos.1 - 1),
                    _ => unreachable!(),
                }
                if n_pos.0 >= 0 && n_pos.0 < 4 && n_pos.1 >= 0 && n_pos.1 < 3 {
                    let n_pos = (n_pos.0 as usize, n_pos.1 as usize);
                    if numpad[n_pos.0][n_pos.1] != '#' {
                        commands.push(c);
                        cp = n_pos;
                        continue;
                    }
                }
            }
            None => {}
        }
        if dist.1 > 0 {
            let n_pos = (cp.0, cp.1 - 1);
            if numpad[n_pos.0][n_pos.1] != '#' {
                commands.push('<');
                cp = n_pos;
                continue;
            }
        }
        if dist.1 < 0 {
            let n_pos = (cp.0, cp.1 + 1);
            if numpad[n_pos.0][n_pos.1] != '#' {
                commands.push('>');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 > 0 {
            let n_pos = (cp.0 - 1, cp.1);
            if numpad[n_pos.0][n_pos.1] != '#' {
                commands.push('^');
                cp = n_pos;
                continue;
            }
        }
        if dist.0 < 0 {
            let n_pos = (cp.0 + 1, cp.1);
            if numpad[n_pos.0][n_pos.1] != '#' {
                commands.push('v');
                cp = n_pos;
                continue;
            }
        }
    }
    commands
}
