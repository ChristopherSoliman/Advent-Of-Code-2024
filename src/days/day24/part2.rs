use std::collections::HashSet;

#[derive(Debug)]
struct Operation<'a> {
    reg1: &'a str,
    reg2: &'a str,
    op: &'a str,
    out: &'a str,
}

pub fn part2(path: &str) -> String {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut ops: Vec<Operation> = Vec::new();

    let (_, ops_in) = input.split_once("\r\n\r\n").expect("Should split");

    for op_in in ops_in.lines() {
        let ins = op_in.split_whitespace().collect::<Vec<&str>>();
        let mut reg1 = ins[0];
        let mut reg2 = ins[2];
        let op = ins[1];
        let out = ins[4];
        if reg1.starts_with("y") {
            std::mem::swap(&mut reg1, &mut reg2);
        }

        ops.push(Operation {
            reg1,
            reg2,
            op,
            out,
        });
    }

    let mut xors: Vec<&Operation> = Vec::new();
    let mut ands: Vec<&Operation> = Vec::new();
    for op in &ops {
        if op.reg1.starts_with("x") {
            if op.op == "AND" {
                ands.push(op);
            }
            if op.op == "XOR" {
                xors.push(op);
            }
        }
    }
    ands.sort_by_key(|v| v.reg1);
    xors.sort_by_key(|v| v.reg1);

    find_swaps(&xors, &ands, &ops).join(",")
}

fn find_swaps<'a>(
    xors: &Vec<&'a Operation<'a>>,
    ands: &Vec<&'a Operation<'a>>,
    ops: &'a Vec<Operation<'a>>,
) -> Vec<&'a str> {
    let mut bad: HashSet<&'a str> = HashSet::new();
    let mut cin = "";
    for i in 0..xors.len() {
        let xor = xors[i].out;
        let and = ands[i].out;
        if xor.starts_with("z") && i != 0 {
            bad.insert(xor);
        }
        if and.starts_with("z") {
            bad.insert(and);
        }
        for op in ops {
            if op.reg1 == xor || op.reg2 == xor {
                if op.op == "XOR" {
                    if !op.out.starts_with("z") {
                        bad.insert(op.out);
                    } else if cin != "" {
                        if op.reg1 == xor && op.op == "XOR" {
                            cin = op.reg1;
                        }
                        if op.reg1 == xor && op.op == "XOR" {
                            cin = op.reg2;
                        }
                    }
                }
                if op.op == "OR" {
                    bad.insert(xor);
                }
                if op.op == "AND" {
                    if op.out.starts_with("z") {
                        bad.insert(op.out);
                    }
                }
            } else if (op.reg1 == and || op.reg2 == and) && i != 0 {
                if op.op != "OR" {
                    bad.insert(and);
                    cin = "";
                } else {
                    cin = op.out;
                    if op.out.starts_with("z") && i != xors.len() - 1 {
                        bad.insert(op.out);
                        cin = "";
                    }
                }
            }
        }
    }

    let mut bad = bad.into_iter().collect::<Vec<_>>();
    bad.sort();
    bad
}
