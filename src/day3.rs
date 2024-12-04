use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day3 {
    lines : Vec<String>
}

impl FromInput for Day3 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut ls = vec![];
        for l in _lines {
            ls.push(l);
        }
        Day3{ lines: ls }
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        let mut sum = 0_usize;
        let do_it = true;

        for l in &self.lines {
            let mut state = " "; /* the start state */
            let mut num_read = 0; /* numericals read */
            let mut mul1 = 0;
            let mut mul2 = 0;
            for c in l.chars() {
                //println!("Char: {} State: {}", c, state);
                match state {
                    " " => {
                        if c == 'm' {
                            state = "m"
                        } else if c == 'd' {
                            state = "d";
                        } else {
                            state = " ";
                        }
                    },
                    "m" => {
                        if c == 'u' {
                            state = "u";
                        } else {
                            state = " ";
                        }
                    },
                    "u" => {
                        if c == 'l' {
                            state = "l";
                        } else {
                            state = " ";
                        }
                    },
                    "l" => {
                        if c == '(' {
                            state = "(";
                            num_read = 0;
                        } else {
                            state = " ";
                        }
                    },
                    "(" => { /* reading up to three numbers */
                        if c.is_numeric() {
                            num_read += 1;
                            if num_read > 3 { state = " "; }
                            else {
                                mul1 *= 10;
                                mul1 += c as i32 - '0' as i32;
                            }
                        } else if c == ',' {
                            if num_read > 0 { state = ","; num_read = 0; }
                            else { state = " "; }
                        } else {
                            state = " ";
                            mul1 = 0;
                            mul2 = 0;
                        }
                    },
                    "," => {
                        if c.is_numeric() {
                            num_read += 1;
                            if num_read > 3 { state = " "; }
                            else {
                                mul2 *= 10;
                                mul2 += c as i32 - '0' as i32;
                            }
                        } else if c == ')' {
                            if num_read > 0 {
                                 state = " ";
                                 //println!("MUL1: {} MUL2: {}", mul1, mul2);
                                 sum += (mul1 * mul2) as usize;
                                 mul1 = 0;
                                 mul2 = 0;
                            }
                            else { state = " "; }
                        } else {
                            state = " ";
                            mul1 = 0;
                            mul2 = 0;
                        }
                    },
                    "d" => {
                        if c == 'o' {
                            state = "o";
                        } else {
                            state = " ";
                        }
                    },
                    _ => {
                        state = " ";
                    }
                }
            }
        };

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let mut sum = 0_usize;
        let mut do_it = true;

        for l in &self.lines {
            let mut state = " "; /* the start state */
            let mut num_read = 0; /* numericals read */
            let mut mul1 = 0;
            let mut mul2 = 0;
            for c in l.chars() {
                //println!("Char: {} State: {}", c, state);
                match state {
                    " " => {
                        if c == 'm' {
                            state = "m"
                        } else if c == 'd' {
                            state = "d";
                        } else {
                            state = " ";
                        }
                    },
                    "m" => {
                        if c == 'u' {
                            state = "u";
                        } else {
                            state = " ";
                        }
                    },
                    "u" => {
                        if c == 'l' {
                            state = "l";
                        } else {
                            state = " ";
                        }
                    },
                    "l" => {
                        if c == '(' {
                            state = "(";
                            num_read = 0;
                        } else {
                            state = " ";
                        }
                    },
                    "(" => { /* reading up to three numbers */
                        if c.is_numeric() {
                            num_read += 1;
                            if num_read > 3 { state = " "; }
                            else {
                                mul1 *= 10;
                                mul1 += c as i32 - '0' as i32;
                            }
                        } else if c == ',' {
                            if num_read > 0 { state = ","; num_read = 0; }
                            else { state = " "; }
                        } else {
                            state = " ";
                            mul1 = 0;
                            mul2 = 0;
                        }
                    },
                    "," => {
                        if c.is_numeric() {
                            num_read += 1;
                            if num_read > 3 { state = " "; }
                            else {
                                mul2 *= 10;
                                mul2 += c as i32 - '0' as i32;
                            }
                        } else if c == ')' {
                            if num_read > 0 {
                                 state = " ";
                                 //println!("MUL1: {} MUL2: {} [active? {}]", mul1, mul2, do_it);
                                 if do_it {
                                    sum += (mul1 * mul2) as usize;
                                 }
                                 mul1 = 0;
                                 mul2 = 0;
                            }
                            else { state = " "; }
                        } else {
                            state = " ";
                            mul1 = 0;
                            mul2 = 0;
                        }
                    },
                    "d" => {
                        if c == 'o' {
                            state = "o";
                        } else {
                            state = " ";
                        }
                    },
                    "o" => {
                        if c == 'n' {
                            state = "DON";
                        } else if c == '(' {
                            state = "DO_OPEN";
                        } else {
                            state = " ";
                        }
                    },
                    "DON" => {
                        if c == '\'' {
                            state = "DON_";
                        } else {
                            state = " ";
                        }
                    },
                    "DON_" => {
                        if c == 't' {
                            state = "DON_T";
                        } else {
                            state = " ";
                        }
                    },
                    "DON_T" => {
                        if c == '(' {
                            state = "DONT_OPEN";
                        } else {
                            state = " ";
                        }
                    },
                    "DO_OPEN" => {
                        if c == ')' {
                            do_it = true;
                        }
                        state = " ";
                    },
                    "DONT_OPEN" => {
                        if c == ')' {
                            do_it = false;
                        }
                        state = " ";
                    }
                    _ => {
                        state = " ";
                    }
                }
            }
        };
        sum.to_string()
    }
}
