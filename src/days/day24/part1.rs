use std::collections::HashMap;

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut registers: HashMap<&str, u8> = HashMap::new();
    let mut ops: Vec<(&str, &str, &str, &str)> = Vec::new(); // reg1, reg2, op, out

    let (assign, ops_in) = input.split_once("\r\n\r\n").expect("Should split");
    for line in assign.lines() {
        let (register, value) = line.split_once(": ").unwrap();
        registers.insert(register, value.parse::<u8>().unwrap());
    }

    let mut z_vals: Vec<&str> = Vec::new();
    for op in ops_in.lines() {
        let ins = op.split_whitespace().collect::<Vec<&str>>();
        let r1 = ins[0];
        let r2 = ins[2];
        let com = ins[1];
        let out = ins[4];
        if out.starts_with("z") {
            z_vals.push(out);
        }
        ops.push((r1, r2, com, out));
    }

    loop {
        let mut z_done = false;
        for (reg1, reg2, op, out) in &ops {
            if z_vals_done(&z_vals, &registers) {
                z_done = true;
                break;
            }

            if !registers.contains_key(reg1) || !registers.contains_key(reg2) {
                continue;
            }

            let r1 = registers.get(reg1).unwrap();
            let r2 = registers.get(reg2).unwrap();
            let val = match op {
                &"AND" => r1 & r2,
                &"OR" => r1 | r2,
                &"XOR" => r1 ^ r2,
                _ => unreachable!("Invalid operation"),
            };
            registers.insert(out, val);
        }
        if z_done {
            break;
        }
    }

    let mut dec = 0;
    z_vals.sort();
    for i in 0..z_vals.len() {
        let val = registers.get(z_vals[i]).unwrap();
        dec += (*val as u64) << i
    }
    dec
}

fn z_vals_done(z_vals: &Vec<&str>, registers: &HashMap<&str, u8>) -> bool {
    for z in z_vals {
        if let None = registers.get(z) {
            return false;
        }
    }
    true
}
