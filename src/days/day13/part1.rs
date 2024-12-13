pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut lines = input.trim().lines();

    let mut result = 0;

    while let Some(line) = lines.next() {
        let (ax, ay) = parse_line(line);
        let (bx, by) = parse_line(lines.next().unwrap());
        let (tx, ty) = parse_line(lines.next().unwrap());
        let s = solve(ax, ay, bx, by, tx, ty);
        result += s;
        lines.next();
    }
    result
}

fn solve(ax: u32, ay: u32, bx: u32, by: u32, tx: u32, ty: u32) -> u64 {
    if bx * ay == ax * by {
        return 0;
    }
    let det: i64 = (ax * by) as i64 - (bx * ay) as i64;
    let na: i64 = (tx * by) as i64 - (bx * ty) as i64;
    let nb: i64 = (ax * ty) as i64 - (tx * ay) as i64;

    if na % det == 0 && nb % det == 0 {
        let p_a = na / det;
        let p_b = nb / det;
        if p_a < 0 || p_b < 0 {
            return 0;
        }
        return p_a as u64 * 3 + p_b as u64;
    }
    0
}

fn parse_line(line: &str) -> (u32, u32) {
    let (_, d) = line.trim().split_once(':').expect("Should split 1");
    let (ax, ay) = d.trim().split_once(',').expect("Should split 2");

    let ax: u32 = match ax.split_once('+') {
        Some((_, r)) => r.parse().unwrap(),
        None => ax.split_once('=').unwrap().1.parse().unwrap(),
    };

    let ay: u32 = match ay.split_once('+') {
        Some((_, r)) => r.parse().unwrap(),
        None => ay.split_once('=').unwrap().1.parse().unwrap(),
    };

    (ax, ay)
}
