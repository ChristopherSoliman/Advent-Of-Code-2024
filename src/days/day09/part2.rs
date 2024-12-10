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

pub fn part2(path: &str) -> u64 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut id = 0;
    let mut cursor = 0;
    let mut empty: Vec<Space> = Vec::new();
    let mut disk: Vec<Block> = Vec::new();

    input.trim().chars().enumerate().for_each(|(i, length)| {
        let length: usize = length.to_string().parse().expect("Couldn't parse length");
        if i % 2 == 1 {
            let data = Space {
                start: cursor,
                length,
            };
            empty.push(data);
        } else {
            let data = Block {
                id,
                start: cursor,
                length,
            };
            id += 1;
            disk.push(data);
        }
        cursor += length;
    });

    for i in (0..disk.len()).rev() {
        let block = disk.get_mut(i).unwrap();
        for j in 0..empty.len() {
            let space = empty.get_mut(j).unwrap();
            if space.start >= block.start {
                empty = empty[..j].to_vec();
                break;
            }
            if space.length >= block.length {
                block.start = space.start;
                if space.length > block.length {
                    space.start = block.start + block.length;
                    space.length = space.length - block.length;
                } else {
                    empty.remove(j);
                }
                break;
            }
        }
    }

    disk.iter()
        .map(|block| {
            (block.start..(block.start + block.length))
                .map(|v| v as u64 * block.id as u64)
                .sum::<u64>()
        })
        .sum()
}
