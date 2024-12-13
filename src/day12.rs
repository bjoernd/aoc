use crate::{DaySolution, FromInput};

pub struct Day12 {
    map: Vec<Vec<char>>,
}

impl FromInput for Day12 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![];
        for l in _lines {
            let mut mapline = vec!['.'];
            for c in l.chars() {
                mapline.push(c);
            }
            mapline.push('.');
            map.push(mapline);
        }
        map.insert(0, vec!['.'; map[0].len()]);
        map.push(vec!['.'; map[0].len()]);
        Day12 { map }
    }
}

impl Day12 {
    fn print(&self) {
        for line in &self.map {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }
}

fn is_known_bound(l: usize, c: usize, bounds: &Vec<((usize, usize), (usize, usize))>) -> bool {
    for b in bounds {
        let l1 = b.0 .0;
        let c1 = b.0 .1;
        let l2 = b.1 .0;
        let c2 = b.1 .1;

        if l == l1 && l1 == l2 && c1 <= c && c <= c2 {
            return true;
        }
        if c == c1 && c1 == c2 && l1 <= l && l <= l2 {
            return true;
        }
    }
    false
}

impl DaySolution for Day12 {
    fn part_one(&self) -> String {
        let mut sum = 0;
        //self.print();

        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        let mut to_visit = vec![];

        for l in 0..self.map.len() {
            for c in 0..self.map[0].len() {
                if visited[l][c] {
                    continue;
                }
                if self.map[l][c] == '.' {
                    visited[l][c] = true;
                    continue;
                }

                let mut area = 0usize;
                let mut bounds = 0usize;
                let area_id = self.map[l][c];
                to_visit.push((l, c));

                while !to_visit.is_empty() {
                    let (x, y) = to_visit.pop().unwrap();
                    if self.map[x][y] == area_id {
                        if visited[x][y] {
                            continue;
                        }

                        area += 1;

                        if self.map[x - 1][y] != area_id {
                            bounds += 1;
                        } else {
                            to_visit.push((x - 1, y));
                        }

                        if self.map[x + 1][y] != area_id {
                            bounds += 1;
                        } else {
                            to_visit.push((x + 1, y));
                        }

                        if self.map[x][y - 1] != area_id {
                            bounds += 1;
                        } else {
                            to_visit.push((x, y - 1));
                        }

                        if self.map[x][y + 1] != area_id {
                            bounds += 1;
                        } else {
                            to_visit.push((x, y + 1));
                        }

                        visited[x][y] = true;
                    }
                }

                //println!("Area {}: {}x{}={}", area_id, area, bounds, area*bounds);
                sum += area * bounds;
                area = 0usize;
                bounds = 0usize;
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0;
        //self.print();

        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        let mut to_visit = vec![];

        for l in 0..self.map.len() {
            for c in 0..self.map[0].len() {
                if visited[l][c] {
                    continue;
                }
                if self.map[l][c] == '.' {
                    visited[l][c] = true;
                    continue;
                }

                let mut area = 0usize;
                let mut up_bounds = Vec::<((usize, usize), (usize, usize))>::new();
                let mut down_bounds = Vec::<((usize, usize), (usize, usize))>::new();
                let mut left_bounds = Vec::<((usize, usize), (usize, usize))>::new();
                let mut right_bounds = Vec::<((usize, usize), (usize, usize))>::new();
                let area_id = self.map[l][c];
                to_visit.push((l, c));

                while !to_visit.is_empty() {
                    let (x, y) = to_visit.pop().unwrap();
                    if self.map[x][y] == area_id {
                        if visited[x][y] {
                            continue;
                        }

                        area += 1;

                        if self.map[x - 1][y] != area_id {
                            // upper bound
                            if !is_known_bound(x - 1, y, &up_bounds) {
                                let mut l1 = x - 1;
                                let mut c1 = y;
                                let mut l2 = l1;
                                let mut c2 = c1;

                                while self.map[l1 + 1][c1 - 1] == area_id
                                    && self.map[l1][c1 - 1] != area_id
                                {
                                    c1 -= 1;
                                }
                                while self.map[l1 + 1][c2 + 1] == area_id
                                    && self.map[l2][c2 + 1] != area_id
                                {
                                    c2 += 1;
                                }

                                //println!("  [up] New bound: {},{} .. {},{}", l1, c1, l2, c2);
                                up_bounds.push(((l1, c1), (l2, c2)));
                            }
                        } else {
                            to_visit.push((x - 1, y));
                        }

                        if self.map[x + 1][y] != area_id {
                            if !is_known_bound(x + 1, y, &down_bounds) {
                                let mut l1 = x + 1;
                                let mut c1 = y;
                                let mut l2 = l1;
                                let mut c2 = c1;

                                while self.map[l1 - 1][c1 - 1] == area_id
                                    && self.map[l1][c1 - 1] != area_id
                                {
                                    c1 -= 1;
                                }
                                while self.map[l1 - 1][c2 + 1] == area_id
                                    && self.map[l2][c2 + 1] != area_id
                                {
                                    c2 += 1;
                                }

                                //println!("  [down] New bound: {},{} .. {},{}", l1, c1, l2, c2);
                                down_bounds.push(((l1, c1), (l2, c2)));
                            }
                        } else {
                            to_visit.push((x + 1, y));
                        }

                        if self.map[x][y - 1] != area_id {
                            if !is_known_bound(x, y - 1, &left_bounds) {
                                let mut l1 = x;
                                let mut c1 = y - 1;
                                let mut l2 = l1;
                                let mut c2 = c1;

                                while self.map[l1 - 1][c1 + 1] == area_id
                                    && self.map[l1 - 1][c1] != area_id
                                {
                                    l1 -= 1;
                                }
                                while self.map[l2 + 1][c2 + 1] == area_id
                                    && self.map[l2 + 1][c2] != area_id
                                {
                                    l2 += 1;
                                }

                                //println!("  [left] New bound: {},{} .. {},{}", l1, c1, l2, c2);
                                left_bounds.push(((l1, c1), (l2, c2)));
                            }
                        } else {
                            to_visit.push((x, y - 1));
                        }

                        if self.map[x][y + 1] != area_id {
                            if !is_known_bound(x, y + 1, &right_bounds) {
                                let mut l1 = x;
                                let mut c1 = y + 1;
                                let mut l2 = l1;
                                let mut c2 = c1;

                                while self.map[l1 - 1][c1 - 1] == area_id
                                    && self.map[l1 - 1][c1] != area_id
                                {
                                    l1 -= 1;
                                }
                                while self.map[l2 + 1][c2 - 1] == area_id
                                    && self.map[l2 + 1][c2] != area_id
                                {
                                    l2 += 1;
                                }

                                //println!("  [right] New bound: {},{} .. {},{}", l1, c1, l2, c2);
                                right_bounds.push(((l1, c1), (l2, c2)));
                            }
                        } else {
                            to_visit.push((x, y + 1));
                        }

                        visited[x][y] = true;
                    }
                }

                let bounds =
                    up_bounds.len() + down_bounds.len() + left_bounds.len() + right_bounds.len();

                //println!("Area {}: {}x{}={}", area_id, area, bounds, area*bounds);
                sum += area * bounds;
                area = 0usize;
                up_bounds.clear();
                down_bounds.clear();
                right_bounds.clear();
                left_bounds.clear();
            }
        }

        sum.to_string()
    }
}
