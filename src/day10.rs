use std::collections::HashSet;

use crate::{DaySolution, FromInput};

pub struct Day10 {
    map: Vec<Vec<u32>>
}

impl FromInput for Day10 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        for l in _lines {
            let mut nums = vec![];
            nums.push(100);
            for c in l.chars() {
                if c == '.'
                {
                    nums.push(100);
                } else {
                    let digit = c.to_digit(10).unwrap();
                    nums.push(digit);
                }
            }
            nums.push(100);
            map.push(nums);
        }

        let bound = vec![100; map[0].len()];
        map.insert(0, bound.clone());
        map.push(bound);

        Day10{ map }
    }
}

fn count_trails1(map: &Vec<Vec<u32>>, pos_l: usize, pos_c: usize, expected: u32) -> HashSet<(usize, usize)> {
    if map[pos_l][pos_c] != expected {
        return HashSet::new();
    }

    if expected == 9 {
        let mut hs = HashSet::new();
        hs.insert((pos_l, pos_c));
        return hs;
    }

    let mut res = HashSet::new();
    res.extend(count_trails1(map, pos_l-1, pos_c, expected+1));
    res.extend(count_trails1(map, pos_l+1, pos_c, expected+1));
    res.extend(count_trails1(map, pos_l, pos_c-1, expected+1));
    res.extend(count_trails1(map, pos_l, pos_c+1, expected+1));
    
    res
}

fn count_trails2(map: &Vec<Vec<u32>>, pos_l: usize, pos_c: usize, expected: u32) -> usize {
    if map[pos_l][pos_c] != expected { return 0; }

    if expected == 9 {
        return 1;
    }

    count_trails2(map, pos_l-1, pos_c, expected+1)
    + count_trails2(map, pos_l+1, pos_c, expected+1)
    + count_trails2(map, pos_l, pos_c-1, expected+1)
    + count_trails2(map, pos_l, pos_c+1, expected+1)
}

impl DaySolution for Day10 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        for l in 1..self.map.len()-1 {
            for c in 1..self.map[0].len()-1 {
                let count = count_trails1(&self.map, l, c, 0);
                sum += count.len();
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        for l in 1..self.map.len()-1 {
            for c in 1..self.map[0].len()-1 {
                let count = count_trails2(&self.map, l, c, 0);
                sum += count;
            }
        }
        sum.to_string()
    }
}
