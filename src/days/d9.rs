use std::fmt::Debug;

use crate::utils::DaySolver;

type FileSystem = Vec<FSType>;

pub struct Day9;

impl DaySolver for Day9 {
    fn part1(&self, input: &str) -> Option<String> {
        let mut filesystem = parse(input);

        let mut empty_block_idx = 1;
        'outer: loop {
            let (id, mut length) = loop {
                match filesystem.pop().unwrap() {
                    FSType::Free(_) => {}
                    FSType::Block(id, length) => break (id, length),
                }
            };

            while length > 0 {
                let Some(file) = filesystem.get_mut(empty_block_idx) else {
                    filesystem.push(FSType::Block(id, length));
                    break 'outer;
                };
                let FSType::Free(empty_size) = file else {
                    empty_block_idx += 1;
                    continue;
                };
                let to_insert = if length < *empty_size {
                    *empty_size -= length;
                    let file = FSType::Block(id, length);
                    length = 0;
                    file
                } else {
                    length -= *empty_size;
                    let file = FSType::Block(id, *empty_size);
                    *empty_size = 0;
                    file
                };
                if *empty_size == 0 {
                    filesystem.remove(empty_block_idx);
                }
                filesystem.insert(empty_block_idx, to_insert);
                empty_block_idx += 1;
            }
        }

        let checksum = get_checksum(&filesystem);
        Some(checksum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut filesystem = parse(input);

        let mut block_idx = filesystem.len() - 1;
        let mut min_block = None;
        while block_idx > 0 {
            let (removed_block, id, length) = loop {
                let removed_block = filesystem[block_idx];
                match removed_block {
                    FSType::Free(_) => {
                        block_idx -= 1;
                    }
                    FSType::Block(id, length) => break (removed_block, id, length),
                }
            };
            match min_block {
                None => {}
                Some(m) => {
                    if id >= m {
                        block_idx -= 1;
                        continue;
                    }
                }
            }
            min_block = Some(id);

            let mut empty_block_idx = 0;
            let mut ok = false;
            while empty_block_idx < block_idx {
                let FSType::Free(s) = &mut filesystem[empty_block_idx] else {
                    empty_block_idx += 1;
                    continue;
                };
                if length > *s {
                    empty_block_idx += 1;
                    continue;
                }
                *s -= length;
                if *s == 0 {
                    filesystem.remove(empty_block_idx);
                } else {
                    block_idx += 1;
                }
                filesystem.insert(empty_block_idx, removed_block);
                ok = true;
                break;
            }
            if ok {
                filesystem[block_idx] = FSType::Free(length);
            }
            block_idx -= 1;
        }

        let checksum = get_checksum(&filesystem);
        Some(checksum.to_string())
    }
}

fn get_checksum(filesystem: &FileSystem) -> usize {
    let mut checksum = 0;
    let mut idx = 0;
    for i in filesystem {
        match i {
            FSType::Free(length) => idx += length,
            FSType::Block(id, length) => {
                for _ in 0..*length {
                    checksum += id * idx;
                    idx += 1;
                }
            }
        }
    }
    checksum
}

fn parse(input: &str) -> FileSystem {
    let mut is_file = true;
    let mut id = 0;
    let mut filesystem = vec![];
    let input = input.replace("\n", "").replace("\r", "");
    input.chars().for_each(|c| {
        is_file = !is_file;
        let length = c.to_digit(10).unwrap() as usize;
        if length == 0 {
            return;
        }
        let entry = match !is_file {
            true => {
                id += 1;
                FSType::Block(id - 1, length)
            }
            false => FSType::Free(length),
        };
        filesystem.push(entry);
    });
    filesystem
}

#[derive(Clone, Copy)]
enum FSType {
    Block(usize, usize),
    Free(usize),
}

impl Debug for FSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        match self {
            Self::Block(arg0, arg1) => {
                for _ in 0..*arg1 {
                    s.push_str(&format!("{}", *arg0));
                }
            }
            Self::Free(arg0) => {
                for _ in 0..*arg0 {
                    s.push('.');
                }
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            2333133121414131402
        "};
        let solver = Day9 {};
        assert_eq!(solver.part1(input).unwrap(), "1928");
        assert_eq!(solver.part2(input).unwrap(), "2858");
    }
}
