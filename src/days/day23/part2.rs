use std::collections::HashMap;

pub fn part2(path: &str) -> String {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

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
    }

    connections
        .iter()
        .map(|(comp, _)| {
            let mut network = max_connections(&comp, &vec![], &connections);
            network.sort();
            network
        })
        .max_by_key(|n| n.len())
        .unwrap()
        .join(",")
}

fn max_connections<'a>(
    comp: &'a str,
    network: &Vec<&'a str>,
    connections: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let mut max_network: Vec<&'a str> = network.clone();
    let conns = connections.get(comp).unwrap();
    if !network.iter().all(|c| conns.contains(c)) {
        return max_network;
    }
    max_network.push(comp);
    for conn in conns {
        if network.contains(conn) {
            continue;
        }
        let new_network = max_connections(&conn, &max_network, &connections);
        if new_network.len() > max_network.len() {
            max_network = new_network;
        }
    }
    max_network
}
