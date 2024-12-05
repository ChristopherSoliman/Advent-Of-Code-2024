use std::collections::{hash_map::Entry, HashMap};

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
        match self.ordering_set.entry(pair[0]) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(pair[1]);
            }
            Entry::Vacant(e) => {
                e.insert(vec![pair[1]]);
            }
        }
    }

    pub fn comparator(&self, a: i32, b: i32) -> bool {
        match self.ordering_set.get(&b) {
            Some(v) => {
                if v.contains(&a) {
                    return false;
                }
            }
            None => return true,
        }
        true
    }

    pub fn validate(&self, print: &Vec<i32>) -> Vec<i32> {
        let mut dependencies: HashMap<i32, usize> = HashMap::new();

        self.ordering_set
            .iter()
            .filter(|(key, _)| print.contains(&key))
            .for_each(|(_, value)| {
                for v in value.iter() {
                    if print.contains(&v) {
                        match dependencies.entry(*v) {
                            Entry::Occupied(mut e) => {
                                *e.get_mut() += 1;
                            }
                            Entry::Vacant(e) => {
                                e.insert(1);
                            }
                        }
                    }
                }
            });

        let mut result: Vec<i32> = Vec::new();

        while result.len() < print.len() {
            print.iter().for_each(|page| {
                if result.contains(&page) {
                    return;
                }
                let dependency = dependencies.get(&page);
                if let Some(c) = dependency {
                    if *c > 0 {
                        return;
                    }
                }
                result.push(page.clone());
                self.ordering_set.iter().for_each(|(key, value)| {
                    if key == page {
                        value.iter().for_each(|v| {
                            if let Some(d) = dependencies.get_mut(&v) {
                                *d -= 1;
                            }
                        });
                    }
                });
            })
        }

        result
    }
}

pub fn part2(path: &str) -> i32 {
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

    let reordered_prints: Vec<Vec<i32>> = prints
        .iter()
        .filter(|print| {
            !print
                .iter()
                .is_sorted_by(|a, b| ordering.comparator(**a, **b))
        })
        .map(|print| ordering.validate(&print))
        .collect();

    let result: i32 = reordered_prints
        .iter()
        .map(|print| print[print.len() / 2])
        .sum();
    result
}
