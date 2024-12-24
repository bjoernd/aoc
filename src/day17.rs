use itertools::Itertools;
use num::FromPrimitive;
use crate::{DaySolution, FromInput};
extern crate num;

#[derive(Clone, Copy, Debug)]
struct CPUState {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instruction_index: u64,
}

#[derive(FromPrimitive,Debug)]
enum Opcodes {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7
}

impl CPUState {
    fn print(&self) {
        println!("[{:4x}] A {:8} B {:8} C {:8}", self.instruction_index, self.reg_a, self.reg_b, self.reg_c);
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
            7 => { panic!("Reserved combo value"); },
            _ => { panic!("Way too high combo value"); },
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
            },
            Opcodes::BXL => {
                let op1 = self.cpu_state.reg_b;
                self.cpu_state.reg_b = op1 ^ operand as i64;
                self.cpu_state.instruction_index += 2;
            },
            Opcodes::BST => {
                let val = self.combo_to_value(operand);
                // println!("  BST op {:x} -> {:x}", val, val & 7);
                self.cpu_state.reg_b = val & 7;
                // println!("  BST res {:x}", self.cpu_state.reg_b);
                self.cpu_state.instruction_index += 2;
            },
            Opcodes::JNZ => {
                if self.cpu_state.reg_a != 0 {
                    self.cpu_state.instruction_index = operand as u64;
                } else {
                    self.cpu_state.instruction_index += 2;
                }
            },
            Opcodes::BXC => {
                self.cpu_state.reg_b = self.cpu_state.reg_b ^ self.cpu_state.reg_c;
                self.cpu_state.instruction_index += 2;
            },
            Opcodes::OUT => {
                let val = self.combo_to_value(operand) & 0x7;
                self.output.push(val as u8);
                self.cpu_state.instruction_index += 2;
            },
            Opcodes::BDV => {
                let combo = self.combo_to_value(operand);
                let numerator = self.cpu_state.reg_a;
                // let denom = 1 << combo;
                self.cpu_state.reg_b = numerator >> combo;
                self.cpu_state.instruction_index += 2;
            },
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
        let mut regA = 0i64;
        let mut regB = 0i64;
        let mut regC = 0i64;
        let mut prog = vec![];

        for l in _lines {
            if l == "" {
                continue;
            }

            let split_l = l.split(" ").collect_vec();
            let determinator = split_l[1];

            match determinator {
                "A:" => {
                    regA = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                "B:" => {
                    regB = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                "C:" => {
                    regC = i64::from_str_radix(split_l[2], 10).unwrap();
                }
                _ => {
                    prog = split_l[1]
                        .split(",")
                        .map(|x| u8::from_str_radix(x, 10).unwrap())
                        .collect_vec();
                }
            }
        }

        Day17{ cpu_state: CPUState{ reg_a: regA, reg_b: regB, reg_c: regC, instruction_index: 0}, program: prog, output: vec![]}
    }
}

impl DaySolution for Day17 {
    fn part_one(&self) -> String {
        let mut computer = self.clone();

        computer.execute();
        computer.output.iter().join(",")
    }

    fn part_two(&self) -> String {
        let mut try_a = 0;

        loop {
            let mut computer = self.clone();
            computer.cpu_state.reg_a = try_a;

            if try_a & 0xFFFFFF == 0 {
                println!("{}", try_a);
            }

            computer.execute();

            if computer.program.len() == computer.output.len() {
                println!("??? {:?} {:?}", computer.program, computer.output);
                let mut matched = true;

                for i in 0..computer.program.len() {
                    if computer.program[i] != computer.output[i] {
                        matched = false;
                    }
                }

                if matched { break; }
            }

            try_a += 1;
        }

        try_a.to_string()
    }
}
