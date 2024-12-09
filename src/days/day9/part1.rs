#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    start: usize,
    length: usize,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    start: usize,
    length: usize,
}

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Filled(Block),
    Empty(Space),
}

pub fn part1(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut id = 0;
    let mut cursor = 0;

    let mut disk: Vec<BlockType> = input
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, length)| {
            let length: usize = length.to_string().parse().expect("Couldn't parse length");
            if length == 0 {
                return None;
            }
            let data_type: BlockType;
            if i % 2 == 0 {
                data_type = BlockType::Filled(Block {
                    id,
                    start: cursor,
                    length,
                });
                id += 1;
            } else {
                data_type = BlockType::Empty(Space {
                    start: cursor,
                    length,
                });
            }
            cursor += length;
            return Some(data_type);
        })
        .collect();

    while let Some(data) = disk.pop() {
        if let BlockType::Filled(block) = data {
            let mut processed = 0;

            for i in 0..disk.len() {
                if let BlockType::Empty(empty) = disk[i] {
                    if empty.length > 0 {
                        disk.remove(i);
                        let to_process = std::cmp::min(block.length - processed, empty.length);
                        disk.insert(
                            i,
                            BlockType::Filled(Block {
                                id: block.id,
                                start: empty.start,
                                length: to_process,
                            }),
                        );

                        if empty.length - to_process > 0 {
                            disk.insert(
                                i + 1,
                                BlockType::Empty(Space {
                                    start: empty.start + to_process,
                                    length: empty.length - to_process,
                                }),
                            );
                        }
                        processed += to_process;

                        if processed == block.length {
                            break;
                        }
                    }
                }
            }
            if processed != block.length {
                if let Some(BlockType::Filled(last)) = disk.last() {
                    disk.push(BlockType::Filled(Block {
                        id: block.id,
                        start: last.start + last.length,
                        length: block.length - processed,
                    }));
                }
                break;
            }
        }
    }

    disk.iter()
        .map(|d| {
            if let BlockType::Filled(block) = d {
                return (block.start..(block.start + block.length))
                    .map(|v| v as u64 * block.id as u64)
                    .sum::<u64>();
            }
            0
        })
        .sum::<u64>()
}
