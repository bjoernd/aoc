use std::usize::MAX;

use itertools::Itertools;

use crate::{DaySolution, FromInput};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    line: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Ord, Debug, Clone, Copy)]
struct Step {
    p: Point,
    cost: usize,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.partial_cmp(&other.cost).unwrap().reverse())
    }
}

pub struct Day18 {
    drops: Vec<Point>,
}

impl FromInput for Day18 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut r = vec![];
        for l in _lines {
            let (col, line) = l
                .split(",")
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect_tuple()
                .unwrap();
            r.push(Point { line, col });
        }
        Day18 { drops: r }
    }
}

#[allow(dead_code)]
fn print_field(field: &Vec<Vec<char>>) {
    for l in field {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
}

static DIMENSIONS: usize = 71;
static STEPS: usize = 1024;

impl DaySolution for Day18 {
    fn part_one(&self) -> String {
        let mut field = vec![vec!['.'; DIMENSIONS]; DIMENSIONS];

        for idx in 0..STEPS {
            field[self.drops[idx].line][self.drops[idx].col] = '#';
        }
        // print_field(&field);

        let start = Point { line: 0, col: 0 };
        let end = Point {
            line: DIMENSIONS - 1,
            col: DIMENSIONS - 1,
        };

        dijkstra(&field, start, end).to_string()
    }

    fn part_two(&self) -> String {
        let mut field = vec![vec!['.'; DIMENSIONS]; DIMENSIONS];

        for idx in 0..STEPS {
            field[self.drops[idx].line][self.drops[idx].col] = '#';
        }
        // print_field(&field);

        let start = Point { line: 0, col: 0 };
        let end = Point {
            line: DIMENSIONS - 1,
            col: DIMENSIONS - 1,
        };

        let mut pos = start;

        for idx in STEPS..self.drops.len() {
            pos = self.drops[idx];
            field[pos.line][pos.col] = '#';
            let dist = dijkstra(&field, start, end);
            if dist == MAX {
                break;
            }
        }

        format!("{},{}", pos.col, pos.line)
    }
}

fn dijkstra(field: &Vec<Vec<char>>, start: Point, end: Point) -> usize {
    let mut unvisited = std::collections::BinaryHeap::<Step>::new();

    unvisited.push(Step { p: start, cost: 0 });
    unvisited.push(Step {
        p: end,
        cost: usize::MAX,
    });

    let mut shortest = vec![vec![usize::MAX; DIMENSIONS]; DIMENSIONS];

    while !unvisited.is_empty() {
        let next = unvisited.pop().unwrap();
        let l = next.p.line;
        let c = next.p.col;

        // println!("Trying {} {}  cost {}", next.p.line, next.p.col, next.cost);

        if next.cost >= shortest[l][c] {
            continue;
        }

        if c > 0 {
            let left = Point {
                line: l,
                col: c - 1,
            };
            if field[l][c - 1] != '#' {
                unvisited.push(Step {
                    p: left,
                    cost: next.cost + 1,
                });
            }
        }
        if c < DIMENSIONS - 1 {
            let right = Point {
                line: l,
                col: c + 1,
            };
            if field[l][c + 1] != '#' {
                unvisited.push(Step {
                    p: right,
                    cost: next.cost + 1,
                });
            }
        }

        if l > 0 {
            let up = Point {
                line: l - 1,
                col: c,
            };
            if field[l - 1][c] != '#' {
                unvisited.push(Step {
                    p: up,
                    cost: next.cost + 1,
                });
            }
        }

        if l < DIMENSIONS - 1 {
            let down = Point {
                line: l + 1,
                col: c,
            };
            if field[l + 1][c] != '#' {
                unvisited.push(Step {
                    p: down,
                    cost: next.cost + 1,
                });
            }
        }

        if shortest[l][c] > next.cost {
            shortest[l][c] = next.cost;
        }

        if next.p == end {
            break;
        }
    }

    shortest[end.line][end.col]
}
