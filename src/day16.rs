use itertools::Itertools;
use std::cmp::Ordering;
use std::hash::Hash;
use std::{collections::HashMap, collections::HashSet};

use crate::{DaySolution, FromInput};

pub struct Day16 {
    map: Vec<Vec<char>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Node {
    line: usize,
    col: usize,
    cost: usize,
    direction: Direction,
}

impl FromInput for Day16 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        for l in _lines {
            map.push(l.chars().collect_vec());
        }
        Day16 { map }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.line.cmp(&other.line))
            .then(self.col.cmp(&other.col))
    }
}

fn how_many_turns(dir1: Direction, dir2: Direction) -> usize {
    *HashMap::<(Direction, Direction), usize>::from([
        ((Direction::Left, Direction::Left), 0),
        ((Direction::Right, Direction::Right), 0),
        ((Direction::Up, Direction::Up), 0),
        ((Direction::Down, Direction::Down), 0),
        ((Direction::Left, Direction::Up), 1),
        ((Direction::Left, Direction::Down), 1),
        ((Direction::Right, Direction::Up), 1),
        ((Direction::Right, Direction::Down), 1),
        ((Direction::Up, Direction::Left), 1),
        ((Direction::Up, Direction::Right), 1),
        ((Direction::Down, Direction::Left), 1),
        ((Direction::Down, Direction::Right), 1),
    ])
    .get(&(dir1, dir2))
    .unwrap_or(&2)
}

impl DaySolution for Day16 {
    fn part_one(&self) -> String {
        let start_l = self.map.iter().position(|x| x.contains(&'S')).unwrap();
        let start_c = self.map[start_l].iter().position(|x| *x == 'S').unwrap();

        let end_l = self.map.iter().position(|x| x.contains(&'E')).unwrap();
        let end_c = self.map[end_l].iter().position(|x| *x == 'E').unwrap();

        // println!("Searching path from {},{} to {},{}", start_l, start_c, end_l, end_c);

        let mut unvisited = HashSet::<Node>::new();
        let mut results = vec![];

        unvisited.insert(Node {
            line: start_l,
            col: start_c,
            cost: 0,
            direction: Direction::Right,
        });

        for l in 0..self.map.len() {
            for c in 0..self.map[0].len() {
                if self.map[l][c] != '#' {
                    unvisited.insert(Node {
                        line: l,
                        col: c,
                        cost: usize::MAX,
                        direction: Direction::Unknown,
                    });
                    // println!("{} rc {}", unvisited.len(), rc);
                }
            }
        }

        loop {
            let current_node = unvisited.iter().min().cloned();

            // println!("  Trying node {:?} (out of {})", current_node, unvisited.len());

            match current_node {
                None => {
                    break;
                }
                Some(node) => {
                    if node.cost == usize::MAX {
                        break;
                    }

                    if node.line == end_l && node.col == end_c {
                        results.push(node);
                    }
                }
            }

            let current_node = current_node.unwrap();

            let below = unvisited
                .iter()
                .find(|n| (**n).col == current_node.col && (**n).line == current_node.line + 1)
                .cloned();
            // println!("    BELOW: {:?}", below);
            match below {
                Some(node) => {
                    unvisited.remove(&node);

                    let new_cost = 1
                        + current_node.cost
                        + 1000 * how_many_turns(current_node.direction, Direction::Down);
                    // println!("      new cost: {}", new_cost);

                    if new_cost < node.cost {
                        unvisited.insert(Node {
                            line: node.line,
                            col: node.col,
                            cost: new_cost,
                            direction: Direction::Down,
                        });
                    } else {
                        unvisited.insert(node);
                    }
                }
                None => {}
            }

            let up = unvisited
                .iter()
                .find(|n| (**n).col == current_node.col && (**n).line == current_node.line - 1)
                .cloned();
            // println!("    UP: {:?}", up);
            match up {
                Some(node) => {
                    unvisited.remove(&node);

                    let new_cost = 1
                        + current_node.cost
                        + 1000 * how_many_turns(current_node.direction, Direction::Up);
                    // println!("      new cost: {}", new_cost);

                    if new_cost < node.cost {
                        unvisited.insert(Node {
                            line: node.line,
                            col: node.col,
                            cost: new_cost,
                            direction: Direction::Up,
                        });
                    } else {
                        unvisited.insert(node);
                    }
                }
                None => {}
            }

            let left = unvisited
                .iter()
                .find(|n| (**n).col == current_node.col - 1 && (**n).line == current_node.line)
                .cloned();
            // println!("    LEFT: {:?}", left);
            match left {
                Some(node) => {
                    unvisited.remove(&node);

                    let new_cost = 1
                        + current_node.cost
                        + 1000 * how_many_turns(current_node.direction, Direction::Left);
                    // println!("      new cost: {}", new_cost);

                    if new_cost < node.cost {
                        unvisited.insert(Node {
                            line: node.line,
                            col: node.col,
                            cost: new_cost,
                            direction: Direction::Left,
                        });
                    } else {
                        unvisited.insert(node);
                    }
                }
                None => {}
            }

            let right = unvisited
                .iter()
                .find(|n| (**n).col == current_node.col + 1 && (**n).line == current_node.line)
                .cloned();
            // println!("    RIGHT: {:?}", right);
            match right {
                Some(node) => {
                    unvisited.remove(&node);

                    let new_cost = 1
                        + current_node.cost
                        + 1000 * how_many_turns(current_node.direction, Direction::Right);
                    // println!("      new cost: {}", new_cost);

                    if new_cost < node.cost {
                        unvisited.insert(Node {
                            line: node.line,
                            col: node.col,
                            cost: new_cost,
                            direction: Direction::Right,
                        });
                    } else {
                        unvisited.insert(node);
                    }
                }
                None => {}
            }

            unvisited.remove(&current_node);
        }

        // println!("{:?}", results);

        results[0].cost.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        sum.to_string()
    }
}
