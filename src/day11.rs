use cached::proc_macro::cached;
use num::pow;
use std::collections::HashMap;

use itertools::Itertools;

use crate::{DaySolution, FromInput};

pub struct Day11 {
    seed: Vec<usize>,
}

impl FromInput for Day11 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut seed = vec![];
        for l in _lines {
            seed = l
                .split(" ")
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect_vec();
        }

        Day11 { seed }
    }
}

#[cached]
fn compute_one(n: usize) -> Vec<usize> {
    let mut res = Vec::new(); // Pre-allocate for efficiency

    if n == 0 {
        res.push(1);
    } else {
        let mut num = n;
        let mut digits = Vec::new();

        // Extract digits without converting to string
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        let len = digits.len();

        // Check if even or odd length
        if len & 1 == 0 {
            let mid = len / 2;

            // Convert back to numbers without using strings
            let left = digits[..mid].iter().fold(0, |acc, &d| acc * 10 + d);
            let right = digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);

            res.push(left);
            res.push(right);
        } else {
            res.push(n * 2024);
        }
    }
    res
}

/* Tried this solution first which performs the actual iterations and generates all stones as is.
This works fine for part 1 and runs out of memory for part 2 due to caching. */
#[allow(dead_code)]
fn compute(n: usize, iterations: usize) -> usize {
    let mut res = vec![n];

    for i in 0..iterations {
        let mut res_new = vec![];
        for x in res {
            res_new.extend(compute_one(x));
        }
        res = res_new;
        println!("{} {:?}", i, res.len());
    }

    res.len()
}

fn count_digits(mut n: usize) -> usize {
    if n == 0 {
        return 1; // Special case: 0 has 1 digit
    }

    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

/* https://www.reddit.com/r/adventofcode/comments/1hcrm32/comment/m1trhj5
 *
 * The idea here is to not expand the actual stone set, but just compute the number of each stone. This works
 * because there is only a limited set of actual values to be considered (starting values plus multiples
 * of 2024 plus some intermediate values essentially) and every step just modifies the count of one of these
 * values. Then we can count total stones by summing up the counts in the end.
 */
fn compute2(ve: Vec<usize>, iterations: usize) -> usize {
    let mut current = HashMap::<usize, usize>::new();
    for item in ve {
        *current.entry(item).or_insert(0) += 1;
    }

    let mut next = HashMap::<usize, usize>::new();

    for _ in 0..iterations {
        next.clear();
        for (k, v) in current.iter_mut() {
            if *k == 0 {
                *next.entry(1).or_insert(0) += *v;
            } else {
                let digits = count_digits(*k);
                if digits % 2 == 0 {
                    let magnitude = pow(10, digits / 2);
                    *next.entry(*k / magnitude).or_default() += *v;
                    *next.entry(*k % magnitude).or_default() += *v;
                } else {
                    *next.entry(*k * 2024).or_insert(0) += *v;
                }
            }
        }
        //println!("{:?}", next);
        current = next.clone();
    }

    current.values().sum()
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        compute2(self.seed.clone(), 25).to_string()
    }

    fn part_two(&self) -> String {
        compute2(self.seed.clone(), 75).to_string()
    }
}
