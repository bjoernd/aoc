use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day4 {
    field: Vec<Vec<char>>,
}

impl FromInput for Day4 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut field = vec![];
        for l in _lines {
            let mut line = vec![];
            for c in l.chars() {
                line.push(c);
            }
            field.push(line);
        }
        Day4 { field }
    }
}

fn probe_direction(
    field: &Vec<Vec<char>>,
    line: isize,
    col: isize,
    dl: isize,
    dc: isize,
    expect: char,
) -> i32 {
    let new_l = line + dl;
    let new_c = col + dc;

    if new_l < 0 {
        return 0;
    }
    if new_c < 0 {
        return 0;
    }
    if new_l as usize >= field.len() {
        return 0;
    }
    if new_c as usize >= field[0].len() {
        return 0;
    }

    if field[new_l as usize][new_c as usize] == expect {
        match expect {
            'M' => return probe_direction(field, new_l, new_c, dl, dc, 'A'),
            'A' => return probe_direction(field, new_l, new_c, dl, dc, 'S'),
            'S' => {
                return 1;
            }
            _ => {
                return 0;
            }
        }
    }
    0
}

fn probe(field: &Vec<Vec<char>>, line: isize, col: isize) -> i32 {
    assert!(field[line as usize][col as usize] == 'X');

    probe_direction(field, line, col, 0, 1, 'M')
        + probe_direction(field, line, col, 0, -1, 'M')
        + probe_direction(field, line, col, 1, 0, 'M')
        + probe_direction(field, line, col, -1, 0, 'M')
        + probe_direction(field, line, col, 1, 1, 'M')
        + probe_direction(field, line, col, -1, 1, 'M')
        + probe_direction(field, line, col, 1, -1, 'M')
        + probe_direction(field, line, col, -1, -1, 'M')
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;

        for li in 0..self.field.len() {
            for ci in 0..self.field[0].len() {
                if self.field[li][ci] == 'X' {
                    let r = probe(&self.field, li as isize, ci as isize) as usize;
                    sum += r;
                    //if r > 0 { print!("{}", r); } else { print!("."); }
                } /*else { print!("."); }*/
            }
            //println!();
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;

        for li in 1..self.field.len() - 1 {
            for ci in 1..self.field[0].len() - 1 {
                if self.field[li][ci] == 'A' {
                    let diag1 = (self.field[li - 1][ci - 1] == 'M'
                        && self.field[li + 1][ci + 1] == 'S')
                        || (self.field[li - 1][ci - 1] == 'S' && self.field[li + 1][ci + 1] == 'M');
                    let diag2 = (self.field[li - 1][ci + 1] == 'M'
                        && self.field[li + 1][ci - 1] == 'S')
                        || (self.field[li - 1][ci + 1] == 'S' && self.field[li + 1][ci - 1] == 'M');
                    if diag1 && diag2 {
                        sum += 1;
                    }
                }
            }
        }

        sum.to_string()
    }
}
