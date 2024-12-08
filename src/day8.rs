use itertools::Itertools;
use std::collections::HashMap;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day8 {
    coordinates: HashMap<char, Vec<(usize, usize)>>,
    dim_l: usize,
    dim_c: usize,
}

impl FromInput for Day8 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut coordinates = HashMap::<char, Vec<(usize, usize)>>::new();
        let mut l = 0usize;
        let mut c = 0usize;
        for line in _lines {
            c = 0;
            for ch in line.chars() {
                if ch != '.' {
                    let v = coordinates.entry(ch).or_insert_with(Vec::new);
                    v.push((l, c));
                }
                c += 1;
            }
            l += 1;
        }
        Day8 {
            coordinates,
            dim_l: l,
            dim_c: c,
        }
    }
}

impl Day8 {
    fn in_bounds(&self, (l, c): (isize, isize)) -> bool {
        l >= 0 && c >= 0 && l < self.dim_l as isize && c < self.dim_c as isize
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> String {
        let mut res = HashMap::<(isize, isize), bool>::new();

        for ch in self.coordinates.keys() {
            //println!("==== {}", ch);
            let coord = &self.coordinates[ch];
            for (a, b) in coord.iter().tuple_combinations() {
                //println!("  {},{} -> {},{}", a.0, a.1, b.0, b.1);
                let ldist = b.0 as isize - a.0 as isize;
                let cdist = b.1 as isize - a.1 as isize;

                let new1 = (a.0 as isize - ldist, a.1 as isize - cdist);
                let new2 = (b.0 as isize + ldist, b.1 as isize + cdist);
                if self.in_bounds(new1) {
                    res.insert(new1, true);
                }
                if self.in_bounds(new2) {
                    res.insert(new2, true);
                }
            }
        }

        res.keys().len().to_string()
    }

    fn part_two(&self) -> String {
        let mut res = HashMap::<(isize, isize), bool>::new();

        for ch in self.coordinates.keys() {
            //println!("==== {}", ch);
            let coord = &self.coordinates[ch];
            for (a, b) in coord.iter().tuple_combinations() {
                //println!("  {},{} -> {},{}", a.0, a.1, b.0, b.1);
                let ldist = b.0 as isize - a.0 as isize;
                let cdist = b.1 as isize - a.1 as isize;

                let mut factor = 0;
                loop {
                    let new1 = (a.0 as isize - factor * ldist, a.1 as isize - factor * cdist);
                    if self.in_bounds(new1) {
                        res.insert(new1, true);
                    } else {
                        break;
                    }
                    factor += 1;
                }

                factor = 0;
                loop {
                    let new2 = (b.0 as isize + factor * ldist, b.1 as isize + factor * cdist);
                    if self.in_bounds(new2) {
                        res.insert(new2, true);
                    } else {
                        break;
                    }
                    factor += 1;
                }
            }
        }

        res.keys().len().to_string()
    }
}
