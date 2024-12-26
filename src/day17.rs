use std::str::FromStr;

use crate::{DaySolution, FromInput};
use itertools::Itertools;
use num::FromPrimitive;
extern crate num;

#[derive(Clone, Copy, Debug)]
struct CPUState {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instruction_index: u64,
}

#[derive(FromPrimitive, Debug)]
enum Opcodes {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl CPUState {
    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "[{:4x}] A {:8} B {:8} C {:8}",
            self.instruction_index, self.reg_a, self.reg_b, self.reg_c
        );
    }
}

#[derive(Clone, Debug)]
pub struct Day17 {
    cpu_state: CPUState,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Day17 {
    fn combo_to_value(&self, combo: u8) -> i64 {
        match combo {
            0..=3 => combo as i64,
            4 => self.cpu_state.reg_a as i64,
            5 => self.cpu_state.reg_b as i64,
            6 => self.cpu_state.reg_c as i64,
            7 => {
                panic!("Reserved combo value");
            }
            _ => {
                panic!("Way too high combo value");
            }
        }
    }

    fn execute_one(&mut self) {
        let op = Opcodes::from_u8(self.program[self.cpu_state.instruction_index as usize]).unwrap();
        let operand = self.program[self.cpu_state.instruction_index as usize + 1];

        // println!("  OP {:?} operand {}", op, operand);

        match op {
            Opcodes::ADV => {
                let combo = self.combo_to_value(operand);
                let numerator = self.cpu_state.reg_a;
                // let denom = 1 << combo;
                self.cpu_state.reg_a = numerator >> combo;
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::BXL => {
                let op1 = self.cpu_state.reg_b;
                self.cpu_state.reg_b = op1 ^ operand as i64;
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::BST => {
                let val = self.combo_to_value(operand);
                // println!("  BST op {:x} -> {:x}", val, val & 7);
                self.cpu_state.reg_b = val & 7;
                // println!("  BST res {:x}", self.cpu_state.reg_b);
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::JNZ => {
                if self.cpu_state.reg_a != 0 {
                    self.cpu_state.instruction_index = operand as u64;
                } else {
                    self.cpu_state.instruction_index += 2;
                }
            }
            Opcodes::BXC => {
                self.cpu_state.reg_b = self.cpu_state.reg_b ^ self.cpu_state.reg_c;
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::OUT => {
                let val = self.combo_to_value(operand) & 0x7;
                self.output.push(val as u8);
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::BDV => {
                let combo = self.combo_to_value(operand);
                let numerator = self.cpu_state.reg_a;
                // let denom = 1 << combo;
                self.cpu_state.reg_b = numerator >> combo;
                self.cpu_state.instruction_index += 2;
            }
            Opcodes::CDV => {
                let combo = self.combo_to_value(operand);
                let numerator = self.cpu_state.reg_a;
                // let denom = 1 << combo;
                self.cpu_state.reg_c = numerator >> combo;
                self.cpu_state.instruction_index += 2;
            }
        }
    }

    fn execute(&mut self) {
        while self.cpu_state.instruction_index < self.program.len() as u64 {
            self.execute_one();
        }
    }
}

impl FromInput for Day17 {
    fn from_lines(_lines: impl Iterator<Item = String>) -> Self {
        let mut reg_a = 0i64;
        let mut reg_b = 0i64;
        let mut reg_c = 0i64;
        let mut prog = vec![];

        for l in _lines {
            if l == "" {
                continue;
            }

            let split_l = l.split(" ").collect_vec();
            let determinator = split_l[1];

            match determinator {
                "A:" => {
                    reg_a = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                "B:" => {
                    reg_b = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                "C:" => {
                    reg_c = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                _ => {
                    prog = split_l[1]
                        .split(",")
                        .map(|x| u8::from_str_radix(x, 10).unwrap())
                        .collect_vec();
                }
            }
        }

        Day17 {
            cpu_state: CPUState {
                reg_a,
                reg_b,
                reg_c,
                instruction_index: 0,
            },
            program: prog,
            output: vec![],
        }
    }
}

fn compute(octets: &Vec<u8>, depth: usize) -> bool {
    let mut input = 0i64;
    for o in octets {
        input <<= 3;
        input |= *o as i64;
    }

    let mut computer = Day17 {
        cpu_state: CPUState {
            reg_a: input,
            reg_b: 0,
            reg_c: 0,
            instruction_index: 0,
        },
        program: vec![2, 4, 1, 1, 7, 5, 4, 0, 0, 3, 1, 6, 5, 5, 3, 0],
        output: vec![],
    };

    computer.execute();

    // println!("{} {}", computer.program.len(), depth);

    if computer.program.len() >= depth {
        if depth == computer.program.len() && computer.output[computer.output.len() - depth..]
        == computer.program[computer.program.len() - depth..] {
            println!("SUCCESS! {}", input);
            return true;
        }

        computer.output[computer.output.len() - depth..]
            == computer.program[computer.program.len() - depth..]
    } else {
        false
    }
}

fn try_level(octets_so_far: Vec<u8>) -> bool {
    for i in 0u8..8 {
        let mut new_v = octets_so_far.clone();
        new_v.push(i);
        let l = new_v.len();
        if compute(&new_v, l) {
            if try_level(new_v.clone()) {
                println!("Found {:?} @ {}", new_v, l);
                return true;
            }
        }
    }

    false
}

impl DaySolution for Day17 {
    fn part_one(&self) -> String {
        let mut computer = self.clone();

        computer.execute();
        computer.output.iter().join(",")
    }

    fn part_two(&self) -> String {
        /*  PROG: 2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0
           OUT:  2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0

           regB = regA & 0x7
           regB = regB XOR 1

           regC = regA >> *regB

           regB = regB XOR regC
           regB = regB XOR 6

           OUT regB & 0x7

           regA = regA >> 3
           JNZ 0

        */

        try_level(vec![]);

        String::from_str("NA").unwrap()
    }
}
