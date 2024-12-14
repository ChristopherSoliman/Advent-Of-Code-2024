const WIDTH: u8 = 101;
const HEIGHT: u8 = 103;
const TIME: u8 = 100;

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut quads = [0; 4];
    let m_x = (WIDTH / 2) as i32;
    let m_y = (HEIGHT / 2) as i32;

    input.lines().for_each(|line| {
        let mut line = line.split_whitespace();
        let (x0, y0) = line
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .split_once(',')
            .unwrap();

        let (vx, vy) = line
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .split_once(',')
            .unwrap();

        let (x0, y0): (i32, i32) = (x0.parse().unwrap(), y0.parse().unwrap());
        let (vx, vy): (i32, i32) = (vx.parse().unwrap(), vy.parse().unwrap());
        let x_f = (x0 + vx * TIME as i32).rem_euclid(WIDTH as i32);
        let y_f = (y0 + vy * TIME as i32).rem_euclid(HEIGHT as i32);

        if x_f == m_x || y_f == m_y {
            return;
        }

        match (x_f < m_x, y_f < m_y) {
            (true, true) => quads[0] += 1,
            (true, false) => quads[1] += 1,
            (false, true) => quads[2] += 1,
            (false, false) => quads[3] += 1,
        };
    });

    quads.iter().product()
}
