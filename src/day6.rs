use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day6
{
    map: Vec<Vec<char>>,
    pl: usize,
    pc: usize
}

impl FromInput for Day6 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        for l in _lines {
            let mut line = vec![];
            line.push(' ');
            line.append(&mut l.chars().collect());
            line.push(' ');
            map.push(line);
        }
        map.insert(0, vec![' '; map[0].len()]);
        map.push(vec![' '; map[0].len()]);

        let mut pos_l = 0 as usize;
        let mut pos_c = 0 as usize;

        for line in 0..map.len() {
            if map[line].contains(&'^') {
                pos_l = line;
                for col in 0..map[line].len() {
                    if map[line][col] == '^' {
                        pos_c = col;
                    }
                }
            }
        }

        Day6 { map, pl : pos_l, pc: pos_c }
    }
}

fn is_looping(map: &Vec<Vec<char>>, start_l: isize, start_c: isize, dl: isize, dc: isize) -> bool {

    let mut dl = dl;
    let mut dc = dc;
    let mut pl = start_l;
    let mut pc = start_c;

    let mut turn_locations: Vec<(isize, isize, isize, isize)> = vec![];

    while map[pl as usize][pc as usize] != ' ' {
        // new position
        pl += dl;
        pc += dc;

        if map[pl as usize][pc as usize] == '#' {
            pl -= dl;
            pc -= dc;

            if turn_locations.contains( &(pl, pc, dl, dc) ) {
                /* we turned here before -> it's a loop! */
                return true;
            }
            turn_locations.push( (pl, pc, dl, dc) );

            if dl == -1 && dc == 0 { /* up -> right */
                dl = 0;
                dc = 1;
            } else if dl == 0 && dc == 1 { /* right -> down */
                dl = 1;
                dc = 0;
            } else if dl == 1 && dc == 0 { /* down -> left */
                dl = 0;
                dc = -1;
            } else { /* left -> up */
                dl = -1;
                dc = 0;
            }
        }        
    }

    false
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        
        /* we start facing upwards */
        let mut dl: isize = -1;
        let mut dc: isize = 0;
        let mut pl: isize = self.pl as isize;
        let mut pc: isize = self.pc as isize;

        let lines = self.map.len();
        let cols = self.map[0].len();

        let mut visited = vec![vec![' ';cols];lines];
        visited[pl as usize][pc as usize] = 'X';

        while self.map[pl as usize][pc as usize] != ' ' {
            visited[pl as usize][pc as usize] = 'X';
            // new position
            pl += dl;
            pc += dc;

            if self.map[pl as usize][pc as usize] == '#' {
                pl -= dl;
                pc -= dc;

                if dl == -1 && dc == 0 { /* up -> right */
                    dl = 0;
                    dc = 1;
                } else if dl == 0 && dc == 1 { /* right -> down */
                    dl = 1;
                    dc = 0;
                } else if dl == 1 && dc == 0 { /* down -> left */
                    dl = 0;
                    dc = -1;
                } else { /* left -> up */
                    dl = -1;
                    dc = 0;
                }
            }
        }

        for i in 0..lines {
            for j in 0..cols {
                if visited[i][j] == 'X' {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        
        for l in 1..self.map.len()-1 {
            for c in 1..self.map[0].len()-1 {
                if self.map[l][c] == '#' { continue; }
                let mut new_map = self.map.clone();
                new_map[l][c] = '#';
                if is_looping(&new_map, self.pl as isize, self.pc as isize, -1, 0) {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }
}
