use crate::{DaySolution, FromInput};

pub struct Day9 {
    blocks : Vec<u32>,
}

impl Day9 {
    const EMPTY_NUM: u32 = 999999;
}

impl FromInput for Day9 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut in_file = true;
        let mut file_id = 0;
        let mut blocks = vec![];
        for l in _lines {
            for c in l.chars() {
               let num = c.to_digit(10).unwrap();
               if in_file {
                 for _ in 0..num {
                    blocks.push(file_id);
                 }
                 file_id += 1;
               } else {
                for _ in 0..num {
                    blocks.push(Day9::EMPTY_NUM);
                }
               }
               in_file = !in_file;
            }
        }

        Day9 { blocks }
    }
}

fn next_empty_idx(blocks: &Vec<u32>, start: usize) -> usize {
    for i in start..blocks.len() {
        if blocks[i] == Day9::EMPTY_NUM {
            return i;
        }
    }

    0
}

fn next_file_idx(blocks: &Vec<u32>, start: usize) -> usize {
    for i in (0..start).rev() {
        if blocks[i] == Day9::EMPTY_NUM {
            return i+1;
        }
    }

    0
}

impl DaySolution for Day9 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;

        let mut i = self.blocks.len() - 1;
        let mut blocks = self.blocks.clone();
        let mut free = next_empty_idx(&blocks, 0);

        while free < i {
            if blocks[i] != Day9::EMPTY_NUM {
                blocks[free] = blocks[i];
                blocks[i] = Day9::EMPTY_NUM;
                free = next_empty_idx(&blocks, free);
            }
            i -= 1;
        }

        for j in 0..i+1 {
            //println!("{} -> {}", j, blocks[j]);
            if blocks[j] != Day9::EMPTY_NUM { sum += j * blocks[j] as usize; }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        todo!("Solve part two of day 9 using your parsed input");
        sum.to_string()
    }
}
