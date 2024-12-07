use itertools::Itertools;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day7 {
    equations: Vec<(usize, Vec<usize>)>,
}

impl FromInput for Day7 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut equations = vec![];
        for l in _lines {
            let mut items = l.split(":");
            let value = usize::from_str_radix(items.next().unwrap(), 10).unwrap();
            let operands = items
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect_vec();
            equations.push((value, operands));
        }

        Day7 { equations }
    }
}

fn compute(target: usize, operands: &Vec<usize>) -> bool {
    if operands.len() == 1 {
        return target == operands[0];
    }

    let op = *operands.last().unwrap();

    let plus = target > op && compute(target - op, &operands[..operands.len() - 1].to_vec());
    let div = target % op == 0 && compute(target / op, &operands[..operands.len() - 1].to_vec());

    /*
    if plus {
        println!("  {} <== + {:?}", target, operands);
    }
    if div {
        println!("  {} <== * {:?}", target, operands);
    }
    */

    plus || div
}

fn compute2(target: usize, operands: &Vec<usize>) -> bool {
    //println!("{} <==< {:?}", target, operands);

    let mut equations: Vec<String> = vec![format!("{}", operands[0])];

    for op in operands[1..].to_vec() {
        let mut eq_new = vec![];
        for eq in &equations {
            let s1 = format!("{} + {}", eq, op);
            let s2 = format!("{} * {}", eq, op);
            let s3 = format!("{} || {}", eq, op);
            eq_new.push(s1);
            eq_new.push(s2);
            eq_new.push(s3);
        }
        equations = eq_new;
    }

    for eq in equations {
        //println!("  Testing eq {}", eq);
        let mut items = eq.split(" ");
        let mut res = usize::from_str_radix(items.next().unwrap(), 10).unwrap();
        loop {
            let op = items.next();
            //println!("  op: {:?} res {}", op, res);
            match op {
                None => {
                    if res == target {
                        //println!("!! {} == {}", eq, target);
                        return true;
                    } else {
                        break;
                    }
                }
                Some(operator) => {
                    let next_op = usize::from_str_radix(items.next().unwrap(), 10).unwrap();
                    match operator {
                        "+" => {
                            res += next_op;
                        }
                        "*" => {
                            res *= next_op;
                        }
                        "||" => {
                            res = format!("{}{}", res, next_op).parse::<usize>().unwrap();
                        }
                        _ => {
                            panic!("Should not get here!");
                        }
                    }
                }
            }
        }
    }

    false
}

impl DaySolution for Day7 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;

        for eq in &self.equations {
            //println!("{:?}", eq);
            //println!("  {}", compute(eq.0, &eq.1));
            if compute(eq.0, &eq.1) {
                //println!("{} <== {:?}", eq.0, eq.1);
                sum += eq.0;
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        for eq in &self.equations {
            if compute2(eq.0, &eq.1) {
                //println!("{} <== {:?}", eq.0, eq.1);
                sum += eq.0;
            }
        }
        sum.to_string()
    }
}
