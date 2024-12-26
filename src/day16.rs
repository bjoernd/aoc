use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::usize::MAX;

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

        // Dijkstra algo: unvisited nodes
        let mut unvisited = HashSet::<Node>::new();

        // this tracks all the intermediate results for the Dijkstra implementation below
        let mut results = vec![];

        unvisited.insert(Node {
            line: start_l,
            col: start_c,
            cost: 0,
            direction: Direction::Right,
        });

        for l in 0..self.map.len() {
            let mut resline = vec![];
            for c in 0..self.map[0].len() {
                resline.push(Node {
                    line: l,
                    col: c,
                    cost: MAX,
                    direction: Direction::Unknown,
                });

                if c == start_c && l == start_l {
                    continue;
                }

                if self.map[l][c] != '#' {
                    unvisited.insert(Node {
                        line: l,
                        col: c,
                        cost: usize::MAX,
                        direction: Direction::Unknown,
                    });
                    // println!("{}", unvisited.len());
                }
            }
            results.push(resline);
        }

        results[start_l][start_c].cost = 0;

        /* Full Dijkstra -- compute all best values until we have no reachable fields anymore */
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

            results[current_node.line][current_node.col] = current_node;
        }

        // println!("{:?}", results);
        // let mut map = self.map.clone();

        // let mut il = end_l;
        // let mut ic = end_c;
        // loop {
        //     if il == start_l && ic == start_c { break; }

        //     match results[il][ic].direction {
        //         Direction::Left => {
        //             map[il][ic] = '<';
        //             ic += 1;
        //         },
        //         Direction::Right => {
        //             map[il][ic] = '>';
        //             ic -= 1;
        //         },
        //         Direction::Up => {
        //             map[il][ic] = '^';
        //             il += 1;
        //         },
        //         Direction::Down => {
        //             map[il][ic] = 'v';
        //             il -= 1;
        //         },
        //         _ => {},
        //     }
        // }

        // for i in 0..map.len() {
        //     for j in 0..map[0].len() {
        //         print!("{}", map[i][j]);
        //     }

        //     print!("    ");

        //     for j in 0..results[0].len() {
        //         if results[i][j].cost == MAX {
        //             print!("..... ");
        //         } else {
        //             print!("{:5} ", results[i][j].cost);
        //         }
        //     }

        //     println!();
        // }

        /* Part 2: We start at the end. We move backwards based on what we found.
          This gives us the actual shortest route found.

          For alternative routes, we need to be aware of "gaps". Example excerpt
          from a map:

             A    B    C    D    E
           0 3008 4009 3010 4011 4012 ->
           1 3007 #### 3009 #### ####
           2 3006 #### 3008 #### ####
        -> 3 2005 2006 2007 #### ####

           Starting at the bottom left (A3), there are two paths with identical
           cost to the top right (E0). The paths split at A3 and meet again at C0.
           The second path however found a lower cost trip to C0 and thus overwrote
           C0's cost with the lower value, but then turns to D0 and at that point
           gets the same cost as the first path which would have gone straight up from
           A3 to A0 and then right.

           If we unroll paths backwards, this means we have to check straight lines
           for "gaps" where a second path with a seemingly lower cost comes in. We then
           need to check if going in a straight line we'd end up with a similarly optimal
           path in any of the other directions and then pursue these as well.
        */

        let mut to_visit = BinaryHeap::<(usize, usize)>::new();
        let mut visited2 = HashSet::<(usize, usize)>::new();
        let mut path_nodes = HashSet::<(usize, usize)>::new();

        to_visit.push((end_l, end_c));

        while !to_visit.is_empty() {
            let (line, col) = to_visit.pop().unwrap();
            let my_cost = results[line][col].cost;

            if self.map[line][col] == '#' {
                continue;
            }

            if visited2.contains(&(line, col)) {
                continue;
            }

            visited2.insert((line, col));

            /* we are definitely a path node ourselves */
            path_nodes.insert((line, col));

            let (left_l, left_c) = (line, col - 1);
            let (right_l, right_c) = (line, col + 1);
            let (down_l, down_c) = (line + 1, col);
            let (up_l, up_c) = (line - 1, col);

            macro_rules! check_and_push {
                ($dir:expr, $offset:expr) => {
                    if my_cost > $offset + 1
                        && results[$dir.0][$dir.1].cost == my_cost - 1 - $offset
                    {
                        to_visit.push(($dir.0, $dir.1));
                    }
                    if my_cost > 1001 + $offset
                        && results[$dir.0][$dir.1].cost == my_cost - 1001 - $offset
                    {
                        to_visit.push(($dir.0, $dir.1));
                    }
                };
            }
            // println!("{} {}", line, col);

            /* a direct predecessor has a cost of either current - 1 (if we moved
              in a straight line) or current - 10001 (if we had to turn)
            */
            if self.map[left_l][left_c] != '#' && my_cost > results[left_l][left_c].cost {
                check_and_push!((left_l, left_c), 0);
                check_and_push!((left_l - 1, left_c), 1);
                check_and_push!((left_l + 1, left_c), 1);
                check_and_push!((left_l, left_c - 1), 1);
            }

            if self.map[right_l][right_c] != '#' && my_cost > results[right_l][right_c].cost {
                check_and_push!((right_l, right_c), 0);
                check_and_push!((right_l - 1, right_c), 1);
                check_and_push!((right_l + 1, right_c), 1);
                check_and_push!((right_l, right_c + 1), 1);
            }

            if self.map[up_l][up_c] != '#' && my_cost > results[up_l][up_c].cost {
                check_and_push!((up_l, up_c), 0);
                check_and_push!((up_l - 1, up_c), 1);
                check_and_push!((up_l, up_c - 1), 1);
                check_and_push!((up_l, up_c + 1), 1);
            }

            if self.map[down_l][down_c] != '#' && my_cost > results[down_l][down_c].cost {
                check_and_push!((down_l, down_c), 0);
                check_and_push!((down_l + 1, down_c), 1);
                check_and_push!((down_l, down_c - 1), 1);
                check_and_push!((down_l, down_c + 1), 1);
            }
        }

        // for i in 0..map.len() {
        //     for j in 0..map[0].len() {
        //         if path_nodes.contains(&(i,j)) {
        //             print!("O");
        //         } else {
        //             print!("{}", self.map[i][j]);
        //         }
        //     }
        //     println!();
        // }

        println!("{}", path_nodes.len() + 1);

        results[end_l][end_c].cost.to_string()
    }

    fn part_two(&self) -> String {
        let sum = 0_usize;
        sum.to_string()
    }
}
