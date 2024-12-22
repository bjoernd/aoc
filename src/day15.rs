use crate::{DaySolution, FromInput};
use colored::Colorize;
use itertools::Itertools;

pub struct Day15 {
    map: Vec<Vec<char>>,
    moves: Vec<char>,
}

impl FromInput for Day15 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut reading_moves = false;
        let mut map = vec![];
        let mut moves = vec![];

        for l in _lines {
            if reading_moves {
                moves.append(&mut l.clone().chars().collect_vec());
            } else {
                if l == "" {
                    reading_moves = true;
                } else {
                    map.push(l.chars().collect_vec())
                }
            }
        }

        Day15 { map, moves }
    }
}

#[allow(dead_code)]
fn print(map: &Vec<Vec<char>>) {
    let mut idx = 0;
    for l in map {
        print!("{:3} ", idx);
        for c in l {
            if *c == ROBOT {
                print!("{}", c.to_string().bold().blue());
            } else if *c == WALL {
                print!("{}", c.to_string().bold().white());
            } else if *c == BOX || *c == BOX_L || *c == BOX_R {
                print!("{}", c.to_string().yellow());
            } else {
                print!("{}", c);
            }
        }
        println!();
        idx += 1;
    }
}

const WALL: char = '#';
const ROBOT: char = '@';
const BOX: char = 'O';
const EMPTY: char = '.';
const BOX_L: char = '[';
const BOX_R: char = ']';

macro_rules! map_access {
    ($map:expr, $x:expr, $y:expr) => {
        $map[$x as usize][$y as usize]
    };
}

fn move_one(
    map: &mut Vec<Vec<char>>,
    vl: isize,
    vc: isize,
    rob_l: isize,
    rob_c: isize,
) -> (isize, isize) {
    let (target_l, target_c) = (rob_l + vl, rob_c + vc);

    /* 1. Are we moving into a wall? */
    if map_access!(map, target_l, target_c) == WALL {
        return (rob_l, rob_c);
    }

    /* 2. Moving boxes? */
    if map_access!(map, target_l, target_c) == BOX {
        let mut non_box_l = target_l;
        let mut non_box_c = target_c;
        while map_access!(map, non_box_l, non_box_c) == BOX {
            non_box_l += vl;
            non_box_c += vc;
        }
        /* only move if we have an empty space */
        if map_access!(map, non_box_l, non_box_c) == EMPTY {
            while non_box_c != target_c || non_box_l != target_l {
                map_access!(map, non_box_l, non_box_c) = BOX;
                non_box_l -= vl;
                non_box_c -= vc;
            }
            map_access!(map, target_l, target_c) = EMPTY;
        }
    }

    /* 3. Now move the robot if we either moved boxes or had an empty space anyway */
    if map_access!(map, target_l, target_c) == EMPTY {
        map_access!(map, target_l, target_c) = ROBOT;
        map_access!(map, rob_l, rob_c) = EMPTY;
    } else {
        return (rob_l, rob_c);
    }

    (target_l, target_c)
}

fn is_box(map: &Vec<Vec<char>>, l: isize, c: isize) -> bool {
    map_access!(map, l, c) == BOX_L || map_access!(map, l, c) == BOX_R
}

fn move_two(
    map: &mut Vec<Vec<char>>,
    vl: isize,
    vc: isize,
    rob_l: isize,
    rob_c: isize,
) -> (isize, isize) {
    let (target_l, target_c) = (rob_l + vl, rob_c + vc);

    /* 1. Are we moving into a wall? */
    if map_access!(map, target_l, target_c) == WALL {
        return (rob_l, rob_c);
    }

    /* 2. Moving boxes? */
    if is_box(map, target_l, target_c) {
        /* Horizontal move --> same as before */
        if vl == 0 {
            let mut non_box_l: isize = target_l;
            let mut non_box_c = target_c;
            while is_box(map, non_box_l, non_box_c) {
                non_box_l += vl;
                non_box_c += vc;
            }
            if map_access!(map, non_box_l, non_box_c) == EMPTY {
                while non_box_c != target_c || non_box_l != target_l {
                    map_access!(map, non_box_l, non_box_c) =
                        map_access!(map, non_box_l - vl, non_box_c - vc);
                    non_box_l -= vl;
                    non_box_c -= vc;
                }
                map_access!(map, target_l, target_c) = EMPTY;
            }
        } else {
            move_vertical(map, target_l, target_c, vl);
        }
    }

    /* 3. Now move the robot if we either moved boxes or had an empty space anyway */
    if map_access!(map, target_l, target_c) == EMPTY {
        map_access!(map, target_l, target_c) = ROBOT;
        map_access!(map, rob_l, rob_c) = EMPTY;
    } else {
        return (rob_l, rob_c);
    }

    (target_l, target_c)
}

fn can_move_vertical(map: &Vec<Vec<char>>, l: isize, c: isize, vl: isize) -> bool {
    let left_col = if map_access!(map, l, c) == BOX_L {
        c
    } else {
        c - 1
    };

    if map_access!(map, l + vl, left_col) == WALL || map_access!(map, l + vl, left_col + 1) == WALL
    {
        return false;
    }

    if map_access!(map, l + vl, left_col) == EMPTY
        && map_access!(map, l + vl, left_col + 1) == EMPTY
    {
        return true;
    }

    let ls = if map_access!(map, l + vl, left_col) == EMPTY {
        true
    } else {
        can_move_vertical(map, l + vl, left_col, vl)
    };
    let rs = if map_access!(map, l + vl, left_col + 1) == EMPTY {
        true
    } else {
        can_move_vertical(map, l + vl, left_col + 1, vl)
    };

    //println!("can_move_vert({} {}) = LS {} RS {}", l+vl, left_col, ls, rs);

    ls && rs
}

fn move_vertical(map: &mut Vec<Vec<char>>, l: isize, c: isize, vl: isize) {
    let left_col = if map_access!(map, l, c) == BOX_L {
        c
    } else {
        c - 1
    };

    // println!("vert({},{},{}, lc = {} '{}')", l, c, vl, left_col, map_access!(map, l, c));

    if can_move_vertical(&map, l, c, vl) {
        if is_box(map, l + vl, left_col) {
            move_vertical(map, l + vl, left_col, vl);
        }

        if is_box(map, l + vl, left_col + 1) {
            move_vertical(map, l + vl, left_col + 1, vl);
        }

        map_access!(map, l + vl, left_col) = BOX_L;
        map_access!(map, l + vl, left_col + 1) = BOX_R;
        map_access!(map, l, left_col) = EMPTY;
        map_access!(map, l, left_col + 1) = EMPTY;
    }
}

fn compute(map: &Vec<Vec<char>>, target: char) -> usize {
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map_access!(map, i, j) == target {
                res += i * 100 + j;
            }
        }
    }
    res
}

impl DaySolution for Day15 {
    fn part_one(&self) -> String {
        let mut map = self.map.clone();

        let mut rob_l: isize = map.iter().position(|l| l.contains(&ROBOT)).unwrap() as isize;
        let mut rob_c: isize = map[rob_l as usize]
            .iter()
            .position(|c| c == &ROBOT)
            .unwrap() as isize;

        //println!("Robot starts at {},{}", rob_l, rob_c);

        for mov in &self.moves {
            //println!("--- MOVE: {} ---", mov);
            match mov {
                '>' => {
                    (rob_l, rob_c) = move_one(&mut map, 0, 1, rob_l, rob_c);
                }
                '<' => {
                    (rob_l, rob_c) = move_one(&mut map, 0, -1, rob_l, rob_c);
                }
                'v' => {
                    (rob_l, rob_c) = move_one(&mut map, 1, 0, rob_l, rob_c);
                }
                '^' => {
                    (rob_l, rob_c) = move_one(&mut map, -1, 0, rob_l, rob_c);
                }
                _ => {
                    panic!("Invalid move: {}", mov);
                }
            }
            // print(&map);
        }

        compute(&map, BOX).to_string()
    }

    fn part_two(&self) -> String {
        let mut map = vec![];

        let mut rob_l = 0isize;
        let mut rob_c = 0isize;

        for l in 0..self.map.len() {
            let mut new_line = vec![];
            for c in 0..self.map[0].len() {
                match self.map[l][c] {
                    BOX => {
                        new_line.push(BOX_L);
                        new_line.push(BOX_R);
                    }
                    EMPTY => {
                        new_line.push(EMPTY);
                        new_line.push(EMPTY);
                    }
                    ROBOT => {
                        new_line.push(ROBOT);
                        new_line.push(EMPTY);
                        rob_l = l as isize;
                        rob_c = c as isize * 2;
                    }
                    WALL => {
                        new_line.push(WALL);
                        new_line.push(WALL);
                    }
                    _ => {
                        panic!("Invalid map tile: {}", self.map[l][c]);
                    }
                }
            }
            map.push(new_line);
        }

        // println!("Robot starts at {},{}", rob_l, rob_c);
        // print(&map);

        for mov in &self.moves {
            // println!("--- ROBOT @ {},{} ---- MOVE: {} ---", rob_l, rob_c, mov);
            // print(&map);
            match mov {
                '>' => {
                    (rob_l, rob_c) = move_two(&mut map, 0, 1, rob_l, rob_c);
                }
                '<' => {
                    (rob_l, rob_c) = move_two(&mut map, 0, -1, rob_l, rob_c);
                }
                'v' => {
                    (rob_l, rob_c) = move_two(&mut map, 1, 0, rob_l, rob_c);
                }
                '^' => {
                    (rob_l, rob_c) = move_two(&mut map, -1, 0, rob_l, rob_c);
                }
                _ => {
                    panic!("Invalid move: {}", mov);
                }
            }
            if !validate(&map) {
                panic!("validation failed");
            }
        }

        compute(&map, '[').to_string()
    }
}

fn validate(map: &Vec<Vec<char>>) -> bool {
    for l in 0..map.len() - 1 {
        for c in 0..map[0].len() - 1 {
            if map_access!(map, l, c) == BOX_L && map_access!(map, l, c + 1) != BOX_R {
                return false;
            }
        }
    }
    true
}
