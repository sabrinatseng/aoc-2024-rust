use std::collections::BTreeMap;

use itertools::Itertools;

advent_of_code::solution!(9);

// Returns (files, free space)
// Value at index i = the size of the file with ID i
fn parse(input: &str) -> (Vec<u8>, Vec<u8>) {
    let files = input.chars().step_by(2).map(|c| (c as u8) - 48).collect();
    // right bound is a hack to skip the \n
    let free_space = input[1..(input.len() - 1)]
        .chars()
        .step_by(2)
        .map(|c| (c as u8) - 48)
        .collect();

    (files, free_space)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut left_ptr = 0;
    let mut curr_block_idx: usize = 0;
    let mut checksum = 0;

    let (mut files, free_space) = parse(input);
    let mut right_ptr = files.len() - 1;

    // Iterate over files and free space together
    while left_ptr <= right_ptr {
        // For the file, add each file block to the checksum
        {
            let file_id = left_ptr;
            let file_size = files[file_id];
            for i in curr_block_idx..curr_block_idx + file_size as usize {
                checksum += i as u64 * file_id as u64;
            }

            // Update curr_block_idx
            curr_block_idx += file_size as usize;
        }

        // For the free space, move files from the end and then add to the checksum
        {
            let mut free_space_to_fill = free_space[left_ptr];
            while free_space_to_fill > 0 && left_ptr < right_ptr {
                let file_id = right_ptr;
                let last_file_size = files[right_ptr];

                // Calculate how much of this file we should fill
                // If the space is bigger than the file, fill the entire file
                // Otherwise fill the entire space with part of the file
                let amount_to_fill = last_file_size.min(free_space_to_fill);
                for i in curr_block_idx..curr_block_idx + amount_to_fill as usize {
                    checksum += i as u64 * file_id as u64;
                }

                // Update state
                curr_block_idx += amount_to_fill as usize;
                free_space_to_fill -= amount_to_fill;

                let new_last_file_size = last_file_size - amount_to_fill;
                if new_last_file_size == 0 {
                    // If the file is now empty because we filled the space with it,
                    // then move onto the next file
                    right_ptr -= 1;
                } else {
                    // If the file is not yet empty, update how much of it is remaining
                    files[right_ptr] -= amount_to_fill;
                }
            }
        }

        left_ptr += 1;
    }

    Some(checksum)
}

#[derive(Copy, Clone, Debug)]
struct File {
    id: usize,
    size: u8,   // blocks
    idx: usize, // block idx of the first file block
}

#[derive(Copy, Clone, Debug)]
struct FreeSpace {
    size: u8, // blocks
    #[allow(dead_code)]
    idx: usize, // block idx of first free block
}

#[derive(Copy, Clone, Debug)]
enum FileOrFreeSpace {
    File(File),
    FreeSpace(FreeSpace),
}

// { starting block idx : file or free space }
#[derive(Debug)]
struct Filesystem(BTreeMap<usize, FileOrFreeSpace>);

impl Filesystem {
    fn new(map: BTreeMap<usize, FileOrFreeSpace>) -> Self {
        Filesystem(map)
    }

    fn calculate_checksum(&self) -> u64 {
        let mut checksum = 0;
        for (idx, file_or_free_space) in self.0.iter() {
            if let FileOrFreeSpace::File(file) = file_or_free_space {
                for i in *idx..*idx + file.size as usize {
                    checksum += i as u64 * file.id as u64;
                }
            }
        }

        checksum
    }

    fn fill_leftmost_free_space(&mut self, file: &File) {
        let entry_to_fill = self
            .0
            .range(0..file.idx)
            .find(|(_, file_or_free_space)| {
                if let FileOrFreeSpace::FreeSpace(free_space) = file_or_free_space {
                    if free_space.size >= file.size {
                        return true;
                    }
                }

                false
            })
            .map(|(idx, _)| idx)
            .cloned();

        let Some(idx) = entry_to_fill else {
            return;
        };

        // First remove the file and add a free space block in its place
        let file = self.0.remove(&file.idx).unwrap();
        let FileOrFreeSpace::File(file) = file else {
            unreachable!()
        };
        let emptied_file = FileOrFreeSpace::FreeSpace(FreeSpace {
            size: file.size,
            idx: file.idx,
        });
        self.0.insert(file.idx, emptied_file);

        // Then remove the free space block
        let free_space = self.0.remove(&idx).unwrap();
        let FileOrFreeSpace::FreeSpace(free_space) = free_space else {
            unreachable!()
        };

        // Then insert the file here
        let moved_file = FileOrFreeSpace::File(File {
            id: file.id,
            size: file.size,
            idx,
        });
        self.0.insert(idx, moved_file);

        // Create a new free space block if necessary
        if free_space.size > file.size {
            let new_idx = idx + file.size as usize;
            let new_free_space = FileOrFreeSpace::FreeSpace(FreeSpace {
                size: free_space.size - file.size,
                idx: new_idx,
            });
            self.0.insert(new_idx, new_free_space);
        }
    }

    #[cfg(test)]
    fn print_blocks(&self) {
        let mut s = "".to_string();
        for (_, file_or_free_space) in self.0.iter() {
            match file_or_free_space {
                FileOrFreeSpace::File(file) => {
                    s.extend(vec![file.id.to_string(); file.size as usize]);
                }
                FileOrFreeSpace::FreeSpace(free_space) => {
                    s.extend(vec!['.'; free_space.size as usize])
                }
            }
        }

        println!("{s}");
    }
}

fn parse_part_two(input: &str) -> Filesystem {
    let mut curr_block_idx = 0;
    let mut map = BTreeMap::new();

    for (file_id, mut chunk) in (&input
        .chars()
        .filter(|c| c != &'\n') // hack to avoid \n when parsing
        .chunks(2))
        .into_iter()
        .enumerate()
    {
        let file_size = chunk.next().unwrap();
        // insert file
        let file_size = (file_size as u8) - 48;
        map.insert(
            curr_block_idx,
            FileOrFreeSpace::File(File {
                id: file_id,
                size: file_size,
                idx: curr_block_idx,
            }),
        );
        curr_block_idx += file_size as usize;

        // insert free space
        if let Some(free_space_size) = chunk.next() {
            let free_space_size = (free_space_size as u8) - 48;
            map.insert(
                curr_block_idx,
                FileOrFreeSpace::FreeSpace(FreeSpace {
                    size: free_space_size,
                    idx: curr_block_idx,
                }),
            );
            curr_block_idx += free_space_size as usize;
        }
    }

    Filesystem::new(map)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut filesystem = parse_part_two(input);

    let mut files_by_id = BTreeMap::new();
    for (_, file_or_free_space) in filesystem.0.clone() {
        if let FileOrFreeSpace::File(file) = file_or_free_space {
            files_by_id.insert(file.id, file);
        }
    }

    // Iterate over each file starting from the right and try to move it to the leftmost free space block
    for (_, file) in files_by_id.into_iter().rev() {
        filesystem.fill_leftmost_free_space(&file);

        #[cfg(test)]
        filesystem.print_blocks();
    }

    // calculate the checksum
    let checksum = filesystem.calculate_checksum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let input = "12345";
        // 0..111....22222
        // 02.111....2222.
        // 022111....222..
        // 0221112...22...
        // 02211122..2....
        // 022111222......
        let result = part_one(input);
        let expected = 1 * 2 + 2 * 2 + 3 * 1 + 4 * 1 + 5 * 1 + 6 * 2 + 7 * 2 + 8 * 2;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
