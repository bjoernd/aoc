use std::fmt::format;

use crate::{DaySolution, FromInput};
use memoize::memoize;

pub struct Day19 {
    patterns: Vec<String>,
    targets: Vec<String>,
}

impl FromInput for Day19 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut patterns = vec![];
        let mut targets = vec![];
        let mut first = true;

        for l in _lines {
            if first {
                for pat in l.split(", ") {
                    patterns.push(pat.to_string());
                }
                first = false;
            } else {
                if l != "" {
                    targets.push(l);
                }
            }
        }

        Day19 { patterns, targets }
    }
}

#[memoize]
fn can_build(target: String, patterns: Vec<String>) -> usize {
    let mut rem = target.clone();

    // println!("can_build({})", rem);

    if rem.len() > 0 {
        let mut prefixes = vec![];

        for p in &patterns {
            if rem.starts_with(p) {
                prefixes.push(p);
            }
        }

        if prefixes.is_empty() {
            return 0;
        }

        let mut count = 0;
        for p in prefixes {
            count += can_build(rem[p.len()..].to_string(), patterns.clone());
        }

        count
    } else {
        1
    }
}

impl DaySolution for Day19 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        let mut count = 0_usize;

        for t in &self.targets {
            let c = can_build(t.clone(), self.patterns.clone());
            if c > 0 {
                sum += 1;
            }
            count += c;
        }

        format!("{} {}", sum, count)
    }

    fn part_two(&self) -> String {
        let sum = 0_usize;
        sum.to_string()
    }
}
