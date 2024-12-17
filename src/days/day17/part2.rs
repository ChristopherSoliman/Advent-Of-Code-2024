use std::collections::HashMap;

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

    //find_reg_a(&inst_pairs, &program, &mut reg)

    let mut a: u64 = 0;
    let mut q: Vec<(usize, u64)> = Vec::new();
    q.push((0, 0));
    whil
    for i in 0..program.len() {
        println!("{}", a);
        let desired = &program[program.len() - 1 - i..];
        for j in 0..=8 {
            println!("{:?}", desired);
            reg[0] = (a << (i * 3)) + j;
            if execute(&inst_pairs, &mut reg) == *desired {
                pos;
                pos_a.push((a << (i * 3)) + j);
                a = (a << (i * 3)) + j;
                break;
            }
        }
    }
    println!("{}", a);
    a
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

//fn find_reg_a(inst_pairs: &Vec<(u8, Input)>, program: &Vec<u8>, reg: &mut [u32; 3]) -> u32 {
//    let mut repeat_idx: Vec<usize> = Vec::new();
//    let mut cursor = 0;
//
//    while cursor < inst_pairs.len() {
//        let pair = &inst_pairs[cursor];
//        if pair.0 == 3 {
//            repeat_idx.push(cursor);
//        }
//        cursor += 1;
//    }
//
//    let mut p_cursor = program.len() - 1;
//    let mut range: Vec<Vec<u32>> = Vec::new();
//    range.push(vec![0]);
//    range.push(vec![0]);
//    range.push(vec![0]);
//
//    for i in 0..repeat_idx.len() {
//        cursor = repeat_idx[i] - 1;
//
//        while cursor > 0 {
//            if inst_pairs[cursor].0 == 5 {
//                break;
//            }
//            cursor -= 1;
//        }
//
//        let mut done = false;
//        loop {
//            println!("A: {:?}", range[0]);
//            println!("B: {:?}", range[1]);
//            println!("C: {:?}", range[2]);
//            let pair = &inst_pairs[cursor];
//            let desired = program[p_cursor];
//
//            println!("P: {}", desired);
//            println!("opcode: {:?}", pair);
//            match pair.0 {
//                0 => {
//                    if let Input::Combo(inp) = pair.1 {
//                        match inp {
//                            reg_idx @ 4..=6 => {
//                                let reg_i = reg_idx as usize - 4;
//                                let base = range[0][0] * 2_u32.pow(pair.1.get_value(reg));
//                                range[0].clear();
//                                for i in 0..8 {
//                                    range[0].push(base + i);
//                                }
//                            }
//                            _ => {}
//                        }
//                    } else {
//                        panic!("");
//                    }
//                }
//                1 => {
//                    let mut new_range = Vec::new();
//                    for b in range[1].iter() {
//                        new_range.push(pair.1.get_value(reg) ^ b);
//                    }
//                    new_range.sort();
//                    range[1] = new_range;
//                }
//                2 => {
//                    if let Input::Combo(inp) = pair.1 {
//                        match inp {
//                            reg_idx @ 4..=6 => {
//                                let reg_i = reg_idx as usize - 4;
//                                range[reg_i] = range[reg_i]
//                                    .clone()
//                                    .into_iter()
//                                    .filter(|v| {
//                                        let rem = *v % 8;
//                                        range[1].contains(&rem)
//                                    })
//                                    .collect::<Vec<u32>>();
//                            }
//                            _ => {}
//                        }
//                    } else {
//                        panic!("Something wrong happened");
//                    }
//                }
//                4 => {
//                    let mut new_range = Vec::new();
//                    for c in range[2].iter() {
//                        for b in range[1].iter() {
//                            new_range.push(c ^ b);
//                        }
//                    }
//                    new_range.sort();
//                    range[1] = new_range;
//                }
//                5 => {
//                    if let Input::Combo(inp) = pair.1 {
//                        match inp {
//                            reg_idx @ 4..=6 => {
//                                let reg_i = reg_idx as usize - 4;
//                                reg[reg_i] = range[reg_i][0] + desired as u32;
//                                range[reg_i] = vec![reg[reg_i]];
//                            }
//                            _ => {}
//                        }
//                        if p_cursor == 0 {
//                            done = true;
//                        } else {
//                            p_cursor -= 1;
//                        }
//                    } else {
//                        panic!("Something wrong happened");
//                    }
//                }
//                6 => {
//                    let base = range[0][0] * 2_u32.pow(pair.1.get_value(reg));
//                    for i in 0..8 {
//                        range[1].push(base + i);
//                    }
//                }
//                7 => {
//                    let base = range[0][0] * 2_u32.pow(pair.1.get_value(reg));
//                    range[2].clear();
//                    for i in 0..8 {
//                        range[2].push(base + i);
//                    }
//                }
//                _ => unreachable!("invalid opcode"),
//            }
//
//            if cursor > 0 {
//                cursor -= 1;
//            } else {
//                if done {
//                    break;
//                }
//                cursor = repeat_idx[i] - 1;
//            }
//        }
//    }
//    range[0][0]
//}
