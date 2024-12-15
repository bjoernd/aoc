use std::cmp::min;

use itertools::Itertools;
use rayon::prelude::*;

use crate::{DaySolution, FromInput};

struct XPair {
    x: usize,
    y: usize,
}

struct Problem {
    buttona: XPair,
    buttonb: XPair,
    prize: XPair,
}

pub struct Day13 {
    inputs: Vec<Problem>,
}

impl FromInput for Day13 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut inputs = vec![];

        let mut ba = XPair { x: 0, y: 0 };
        let mut bb = XPair { x: 0, y: 0 };
        let mut pr = XPair { x: 0, y: 0 };

        for l in _lines {
            //println!("{}", l);
            if l.contains("A:") {
                let x = l.split(" ").collect_vec()[2].split("+").collect_vec()[1].replace(",", "");
                let y = l.split(" ").collect_vec()[3].split("+").collect_vec()[1].replace(",", "");
                ba = XPair {
                    x: usize::from_str_radix(x.as_str(), 10).unwrap(),
                    y: usize::from_str_radix(y.as_str(), 10).unwrap(),
                };
            } else if l.contains("B:") {
                let x = l.split(" ").collect_vec()[2].split("+").collect_vec()[1].replace(",", "");
                let y = l.split(" ").collect_vec()[3].split("+").collect_vec()[1].replace(",", "");
                bb = XPair {
                    x: usize::from_str_radix(x.as_str(), 10).unwrap(),
                    y: usize::from_str_radix(y.as_str(), 10).unwrap(),
                };
            } else if l.contains("Prize:") {
                let x = l.split(" ").collect_vec()[1].split("=").collect_vec()[1].replace(",", "");
                let y = l.split(" ").collect_vec()[2].split("=").collect_vec()[1].replace(",", "");
                pr = XPair {
                    x: usize::from_str_radix(x.as_str(), 10).unwrap(),
                    y: usize::from_str_radix(y.as_str(), 10).unwrap(),
                };
            } else {
                inputs.push(Problem {
                    buttona: ba,
                    buttonb: bb,
                    prize: pr,
                });
                ba = XPair { x: 0, y: 0 };
                bb = XPair { x: 0, y: 0 };
                pr = XPair { x: 0, y: 0 };
            }
        }

        inputs.push(Problem {
            buttona: ba,
            buttonb: bb,
            prize: pr,
        });
        ba = XPair { x: 0, y: 0 };
        bb = XPair { x: 0, y: 0 };
        pr = XPair { x: 0, y: 0 };

        Day13 { inputs }
    }
}

impl DaySolution for Day13 {
    fn part_one(&self) -> String {
        /*
         *   a and b being the variables to solve for, x1+2, y1+2, prize_x and prize_y are given
         *
         *   a*x1 + b*x2 = PX
         *   a*y1 + b*y2 = PY
         *
         *   a*x1 = PX - b*x2
         *   a = (PX - b*x2) / x1
         *
         *   b*y2 = PY - a*y1
         *   b*y2 = PY -  (PX - b*x2) / x1 * y1
         *   b * y2 = PY - y1 * PX/x1 + y1 * b * x2 / x1
         *   b * y2 - y1 * b * x2 / x1 = PY - y1 * PX / x1
         *   b * y2 * x1 - b * y1 * x2 = PY * x1 - y1 * PX
         *   b * (y2 * x1 - y1 * x2) = PY * x1 - y1 * PX
         *
         *       PY * x1 - y1 * PX
         *   b = -----------------
         *       y2 * x1 - y1 * x2
         */
        let mut sum: usize = 0;

        for problem in &self.inputs {
            //println!("?? {} {}", problem.prize.x, problem.prize.y);

            let PX = problem.prize.x as isize;
            let PY = problem.prize.y as isize;
            let x1 = problem.buttona.x as isize;
            let x2 = problem.buttonb.x as isize;
            let y1 = problem.buttona.y as isize;
            let y2 = problem.buttonb.y as isize;

            let b = (PY * x1 - y1 * PX) / (y2 * x1 - y1 * x2);
            let a = (PX - b * x2) / x1;

            /* We only want non-fractional integers, so validate by replacing back */
            if a * x1 + b * x2 == PX && a * y1 + b * y2 == PY {
                //println!("A = {} B = {}", a, b);
                sum += (3 * a as usize + b as usize);
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        /*
         *   Now it's just PX and PY that change...
         */
        let mut sum: usize = 0;

        for problem in &self.inputs {
            //println!("?? {} {}", problem.prize.x, problem.prize.y);

            let PX = problem.prize.x as isize + 10000000000000;
            let PY = problem.prize.y as isize + 10000000000000;
            let x1 = problem.buttona.x as isize;
            let x2 = problem.buttonb.x as isize;
            let y1 = problem.buttona.y as isize;
            let y2 = problem.buttonb.y as isize;

            let b = (PY * x1 - y1 * PX) / (y2 * x1 - y1 * x2);
            let a = (PX - b * x2) / x1;

            /* We only want non-fractional integers, so validate by replacing back */
            if a * x1 + b * x2 == PX && a * y1 + b * y2 == PY {
                //println!("A = {} B = {}", a, b);
                sum += (3 * a as usize + b as usize);
            }
        }

        sum.to_string()
    }
}
