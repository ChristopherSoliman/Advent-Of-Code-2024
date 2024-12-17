#[derive(Debug)]
enum Input {
    Combo(u8),
    Literal(u8),
    None,
}

impl Input {
    pub fn get_value(&self, reg: &[u64]) -> u64 {
        match self {
            Input::Literal(v) => return *v as u64,
            Input::Combo(c) => match c {
                v @ 0..=3 => return *v as u64,
                v @ 4..=6 => return reg[*v as usize - 4],
                7 => panic!("invalid program"),
                _ => unreachable!("invalid combo code"),
            },
            _ => 0,
        }
    }
}

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let (reg_in, inst) = input.trim().split_once("\r\n\r\n").expect("Should split");

    let mut reg: [u64; 3] = [0; 3];

    for (i, line) in reg_in.trim().lines().enumerate() {
        reg[i] = line
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .parse()
            .expect("Couldn't parse int");
    }

    let inst = inst
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .replace(",", "")
        .chars()
        .collect::<Vec<_>>();

    reg[0] = 0;
    reg[1] = 0;
    reg[2] = 0;

    let mut program: Vec<u8> = Vec::new();
    let mut inst_pairs: Vec<(u8, Input)> = Vec::new();

    let mut i = 0;
    while i < inst.len() {
        let opcode = inst[i].to_string().parse::<u8>().unwrap();
        let raw_operand = inst[i + 1].to_string().parse::<u8>().unwrap();
        program.push(opcode);
        program.push(raw_operand);

        let operand: Input;
        match opcode {
            0 | 2 | 5..=7 => operand = Input::Combo(raw_operand),
            1 | 3 => operand = Input::Literal(raw_operand),
            4 => operand = Input::None,
            _ => panic!("invalid opcode"),
        }
        inst_pairs.push((opcode, operand));
        i += 2;
    }

    let mut q: Vec<(usize, u64)> = Vec::new();
    q.push((0, 0));
    while !q.is_empty() {
        let (level, a_val) = q.remove(0);
        let desired = &program[program.len() - 1 - level..];
        for j in 0..=7 {
            reg[0] = a_val << 3 | j;
            if execute(&inst_pairs, &mut reg) == *desired {
                if level == program.len() - 1 {
                    return a_val << 3 | j;
                } else {
                    q.push((level + 1, a_val << 3 | j));
                }
            }
        }
    }
    panic!("No solution found");
}

fn execute(inst_pairs: &Vec<(u8, Input)>, reg: &mut [u64; 3]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut cursor = 0;

    while cursor < inst_pairs.len() {
        let pair = &inst_pairs[cursor];
        match pair.0 {
            0 => reg[0] = reg[0] >> pair.1.get_value(reg),
            1 => reg[1] = reg[1] ^ pair.1.get_value(reg),
            2 => reg[1] = pair.1.get_value(reg) % 8,
            3 => {
                if reg[0] != 0 {
                    cursor = pair.1.get_value(reg) as usize / 2;
                    continue;
                }
            }
            4 => reg[1] = reg[1] ^ reg[2],
            5 => out.push((pair.1.get_value(reg) % 8) as u8),
            6 => reg[1] = reg[0] >> pair.1.get_value(reg),
            7 => reg[2] = reg[0] >> pair.1.get_value(reg),
            _ => unreachable!("invalid opcode"),
        }
        cursor += 1;
    }

    out
}
