#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Filled(Block),
    Empty(Space),
}

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut id = 0;
    let mut free_space = false;
    let mut cursor = 0;

    let mut disk: Vec<BlockType> = input
        .trim()
        .chars()
        .filter_map(|data| {
            let length: usize = data.to_string().parse().expect("Couldn't parse length");
            if length == 0 {
                free_space = !free_space;
                return None;
            }
            let data_type: BlockType;
            if !free_space {
                data_type = BlockType::Filled(Block {
                    id,
                    start: cursor,
                    end: cursor + length - 1,
                });
                id += 1;
            } else {
                data_type = BlockType::Empty(Space {
                    start: cursor,
                    end: cursor + length - 1,
                });
            }

            cursor += length;
            free_space = !free_space;
            return Some(data_type);
        })
        .collect();

    while let Some(data) = disk.pop() {
        if let BlockType::Filled(block) = data {
            let n = block.end - block.start + 1;
            let mut processed = 0;

            for i in 0..disk.len() {
                if let BlockType::Empty(empty) = disk[i] {
                    let space = empty.end - empty.start + 1;
                    if (n - processed) < space {
                        disk.remove(i);
                        disk.insert(
                            i,
                            BlockType::Filled(Block {
                                id: block.id,
                                start: empty.start,
                                end: empty.start + (n - processed) - 1,
                            }),
                        );

                        disk.insert(
                            i + 1,
                            BlockType::Empty(Space {
                                start: empty.start + n - processed,
                                end: empty.end,
                            }),
                        );
                        processed = n;
                        break;
                    } else {
                        disk.remove(i);
                        disk.insert(
                            i,
                            BlockType::Filled(Block {
                                id: block.id,
                                start: empty.start,
                                end: empty.end,
                            }),
                        );
                        processed += space;
                        if processed == n {
                            break;
                        }
                    }
                }
            }
            if processed != n {
                if let Some(BlockType::Filled(last)) = disk.last() {
                    disk.push(BlockType::Filled(Block {
                        id: block.id,
                        start: last.end + 1,
                        end: last.end + (n - processed),
                    }));
                }
                break;
            }
        }
    }

    disk.iter()
        .filter_map(|d| {
            if let BlockType::Filled(b) = d {
                return Some(b);
            }
            None
        })
        .map(|block| {
            (block.start..=block.end)
                .map(|v| v as u64 * block.id as u64)
                .sum::<u64>()
        })
        .sum()
}
