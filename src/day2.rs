use std::{i32::MAX};

use itertools::Itertools;
use num::abs;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day2
{
    reports : Vec<Vec<i32>>
}

impl FromInput for Day2 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut reports = vec![];
        for l in _lines {
            let x : Vec<i32> = l.split(" ")
                                .into_iter()
                                .map(|a| i32::from_str_radix(a, 10).unwrap())
                                .collect_vec();
            reports.push(x);
        }

        Day2{ reports }
    }
}

fn is_increasing(v: &Vec<i32>) -> bool {
    let mut last = -1;
    for e in v {
        if *e <= last {
            return false;
        }
        last = *e;
    }
    true
}

fn is_decreasing(v: &Vec<i32>) -> bool {
    let mut last = MAX;
    for e in v {
        if *e >= last {
            return false;
        }
        last = *e;
    }
    true
}

fn diff_check(v: &Vec<i32>, threshold: i32) -> bool {
    let mut last = v[0];
    for e in v {
        if abs(e - last) > threshold {
            return false;
        }
        last = *e;
    }
    true
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        
        for r in &self.reports {
            if is_increasing(r) || is_decreasing(r) {
                if diff_check(r, 3) {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        todo!("Solve part two of day 2 using your parsed input");
        sum.to_string()
    }
}
