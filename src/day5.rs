use itertools::Itertools;
use std::collections::HashMap;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day5 {
    /* Rules map X -> List of pages that must come after X */
    rules: HashMap<u32, Vec<u32>>,
    orders: Vec<Vec<u32>>,
}

impl FromInput for Day5 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut orders = vec![];
        let mut reading_rules = true;
        for l in _lines {
            if l == "" {
                reading_rules = false;
                continue;
            }

            if reading_rules {
                let rule = l.split("|").collect_vec();
                let first = u32::from_str_radix(rule[0], 10).unwrap();
                let second = u32::from_str_radix(rule[1], 10).unwrap();
                rules
                    .entry(first)
                    .and_modify(|x| x.push(second))
                    .or_insert(vec![second]);
            } else {
                orders.push(
                    l.split(",")
                        .collect_vec()
                        .into_iter()
                        .map(|x| u32::from_str_radix(x, 10).unwrap())
                        .collect_vec(),
                );
            }
        }
        Day5 { rules, orders }
    }
}

impl Day5 {
    fn order_is_valid(&self, order: &Vec<u32>) -> bool {
        let mut printed = vec![];
        for item in order {
            for pre in &printed {
                if self.rules[item].contains(pre) {
                    /*println!("Found rule violation: Order {:?} Item {} must not come after item {}",
                    order, item, pre);*/
                    return false;
                }
            }
            printed.push(*item);
        }

        true
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        for order in &self.orders {
            if self.order_is_valid(order) {
                /*println!("Valid order: {:?}", order);*/
                let middle = order[order.len() / 2];
                sum += middle as usize;
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        for order in &self.orders {
            if !self.order_is_valid(order) {
                //println!("=== Invalid order: {:?}", order);

                let mut new_order = order.clone();

                /* Basic idea: find a rule violation. Swap the violating
                   elements. Repeat until the list is valid.

                   I did think about the potential of never reaching a quiet state, but
                   that apparently wasn't necessary. One would likely have to determine
                   a maximum number of swaps after which we have swapped all the numbers
                   and thus won't see a valid ordering ever.
                */
                while !self.order_is_valid(&new_order) {
                    for i in 0..new_order.len() {
                        let mut valid = true;
                        for pre in 0..i {
                            if self.rules[&new_order[i]].contains(&new_order[pre]) {
                                let x = new_order[i];
                                new_order[i] = new_order[pre];
                                new_order[pre] = x;
                                valid = false;
                                //println!("  new: {:?}", new_order);
                                break;
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                }
                let middle = new_order[new_order.len() / 2];
                sum += middle as usize;
            }
        }
        sum.to_string()
    }
}
