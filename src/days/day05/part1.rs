use std::collections::HashMap;

#[derive(Debug)]
struct Ordering {
    ordering_set: HashMap<i32, Vec<i32>>,
}
impl Ordering {
    pub fn new() -> Self {
        let ordering_set: HashMap<i32, Vec<i32>> = HashMap::new();
        Ordering { ordering_set }
    }

    pub fn add(&mut self, pair: Vec<i32>) {
        match self.ordering_set.get_mut(&pair[0]) {
            Some(e) => {
                e.push(pair[1]);
            }
            None => {
                self.ordering_set.insert(pair[0], vec![pair[1]]);
            }
        }
    }

    pub fn comparator(&self, a: &i32, b: &i32) -> bool {
        match self.ordering_set.get(b) {
            Some(v) => {
                if v.contains(a) {
                    return false;
                }
            }
            None => return true,
        }
        true
    }
}

pub fn part1(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut prints: Vec<Vec<i32>> = Vec::new();
    let mut ordering = Ordering::new();

    let mut prints_flag = false;
    input.trim().lines().for_each(|line| {
        if line.is_empty() {
            prints_flag = true;
        } else {
            match prints_flag {
                true => prints.push(
                    line.split(',')
                        .map(|p| p.parse().expect("Can't convert to int"))
                        .collect(),
                ),
                false => ordering.add(
                    line.split('|')
                        .map(|p| p.parse().expect("Can't convert to int"))
                        .collect::<Vec<i32>>(),
                ),
            }
        }
    });

    let result: i32 = prints
        .into_iter()
        .filter(|print| {
            print
                .iter()
                .is_sorted_by(|a, b| ordering.comparator(*a, *b))
        })
        .map(|print| print[print.len() / 2])
        .sum();

    result
}
