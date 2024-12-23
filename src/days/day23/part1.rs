use std::collections::HashMap;

pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut count = 0;

    for line in input.lines() {
        let (pc1, pc2) = line.trim().split_once('-').expect("Should split");
        connections
            .entry(pc1)
            .and_modify(|v| v.push(pc2))
            .or_insert(vec![pc2]);

        connections
            .entry(pc2)
            .and_modify(|v| v.push(pc1))
            .or_insert(vec![pc1]);

        let con1 = connections.get(pc1).unwrap();
        let con2 = connections.get(pc2).unwrap();

        count += con1
            .iter()
            .filter(|c| {
                con2.contains(c)
                    && (pc1.starts_with("t") || pc2.starts_with("t") || c.starts_with("t"))
            })
            .count();
    }
    count as u32
}
