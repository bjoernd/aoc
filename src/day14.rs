use crate::{DaySolution, FromInput};
use itertools::{max, Itertools};
use std::isize;

#[derive(Copy, Clone)]
struct Position {
    l: isize,
    c: isize,
}

#[derive(Copy, Clone)]
struct Velocity {
    lv: isize,
    cv: isize,
}

pub struct Day14 {
    bots: Vec<(Position, Velocity)>,
    lines: usize,
    cols: usize,
}

impl FromInput for Day14 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut bots = vec![];
        for l in _lines {
            let (pos, velo) = l.split(" ").collect_tuple().unwrap();
            let position = pos
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| isize::from_str_radix(x, 10).unwrap())
                .collect_vec();
            let velocity = velo
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|x| isize::from_str_radix(x, 10).unwrap())
                .collect_vec();

            bots.push((
                Position {
                    l: position[1],
                    c: position[0],
                },
                Velocity {
                    lv: velocity[1],
                    cv: velocity[0],
                },
            ));
        }
        Day14 {
            bots,
            lines: 103,
            cols: 101,
        }
    }
}

fn print(bots: &Vec<(Position, Velocity)>, lines: usize, cols: usize) {
    for l in 0..lines {
        for c in 0..cols {
            let count = bots
                .iter()
                .filter(|x| x.0.c == c as isize && x.0.l == l as isize)
                .count();
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
}

fn move_one(
    bots: &Vec<(Position, Velocity)>,
    lines: usize,
    cols: usize,
) -> Vec<(Position, Velocity)> {
    let mut result = vec![];
    for bot in bots {
        let mut new_pos = Position {
            l: bot.0.l + bot.1.lv,
            c: bot.0.c + bot.1.cv,
        };
        if new_pos.c < 0 {
            new_pos.c += cols as isize;
        }
        if new_pos.l < 0 {
            new_pos.l += lines as isize;
        }
        if new_pos.c >= cols as isize {
            new_pos.c -= cols as isize;
        }
        if new_pos.l >= lines as isize {
            new_pos.l -= lines as isize;
        }
        result.push((new_pos, bot.1));
    }
    result
}

fn count(bots: &Vec<(Position, Velocity)>, lines: usize, cols: usize) -> usize {
    let middle_line = lines as isize / 2;
    let middle_col = cols as isize / 2;

    let nw = bots
        .iter()
        .filter(|x| x.0.c < middle_col && x.0.l < middle_line)
        .count();
    let ne = bots
        .iter()
        .filter(|x| x.0.c > middle_col && x.0.l < middle_line)
        .count();
    let sw = bots
        .iter()
        .filter(|x| x.0.c < middle_col && x.0.l > middle_line)
        .count();
    let se = bots
        .iter()
        .filter(|x| x.0.c > middle_col && x.0.l > middle_line)
        .count();

    nw * ne * sw * se
}

fn has_contiguous_bots(bots: &Vec<(Position, Velocity)>, lines: usize, cols: usize) -> bool {
    let mut max_consecutive = 0;
    for l in 0..lines as isize {
        let mut bots_in_line = bots.iter().filter(|x| x.0.l == l).collect_vec();
        bots_in_line.sort_by(|x, y| x.0.c.cmp(&y.0.c));
        let mut consecutive = 0;
        let mut last_c = 0;
        for b in bots_in_line {
            if b.0.c == last_c + 1 {
                consecutive += 1;
            } else {
                if max_consecutive < consecutive {
                    max_consecutive = consecutive;
                }
                consecutive = 0;
            }
            last_c = b.0.c;
        }
    }
    max_consecutive > 10
}

impl DaySolution for Day14 {
    fn part_one(&self) -> String {
        let mut bots = self.bots.clone();

        for i in 0..100 {
            bots = move_one(&bots, self.lines, self.cols);
        }
        print(&bots, self.lines, self.cols);
        count(&bots, self.lines, self.cols).to_string()
    }

    fn part_two(&self) -> String {
        let mut res = 0_usize;
        let mut bots = self.bots.clone();

        for i in 0..100000 {
            bots = move_one(&bots, self.lines, self.cols);
            if has_contiguous_bots(&bots, self.lines, self.cols) {
                res = i + 1;
                print(&bots, self.lines, self.cols);
                break;
            }
        }

        res.to_string()
    }
}
