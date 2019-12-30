extern crate itertools;

use itertools::Itertools;
use std::{fs, io};
use std::str::FromStr;
use std::io::Read;

struct Computer {
    ip: usize,
    memory: Vec<i32>,
    memory_orig: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
    halted: bool
}

impl Computer {
    fn new(memory: Vec<i32>) -> Computer {
        Computer {
            ip: 0,
            memory: memory.clone(),
            memory_orig: memory,
            input: Vec::new(),
            output: Vec::new(),
            halted: false,
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_orig.clone();
        self.input.clear();
        self.output.clear();
        self.halted = false;
    }

    fn push_input(&mut self, value: i32) {
        self.input.push(value);
    }

    fn pop_output(&mut self) -> i32 {
        self.output.remove(self.output.len() - 1)
    }

    fn peek(&self, offset: usize, mode: u8) -> i32 {
        let value = match mode {
            0 => self.memory[self.memory[offset] as usize] as i32,
            1 => self.memory[offset] as i32,
            m => panic!("Unknown mode: {}", m),
        };
        value
    }

    fn run(&mut self) {
        loop {
            let (opcode, modes) = Computer::decode(self.memory[self.ip] as i32);
            match opcode {
                1 => {
                    // add
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    let a = self.memory[self.ip + 3] as usize;
                    self.memory[a] = x + y;
                    self.ip += 4;
                }
                2 => {
                    // multiply
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    let a = self.memory[self.ip + 3] as usize;
                    self.memory[a] = x * y;
                    self.ip += 4;
                }
                3 => {
                    // input
                    let a = self.memory[self.ip + 1] as usize;
                    if self.input.is_empty() {
                        break;
                    }
                    self.memory[a] = self.input.remove(0);
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.memory[self.ip + 1] as usize;
                    self.output.push(self.memory[a]);
                    self.ip += 2;
                    break;
                }
                5 => {
                    // jump if true
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x != 0 {
                        self.ip = y as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // jump if false
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x == 0 {
                        self.ip = y as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // less than
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    let a = self.memory[self.ip + 3] as usize;
                    if x < y {
                        self.memory[a] = 1;
                    } else {
                        self.memory[a] = 0;
                    }
                    self.ip += 4;
                }
                8 => {
                    // equals
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    let a = self.memory[self.ip + 3] as usize;
                    if x == y {
                        self.memory[a] = 1;
                    } else {
                        self.memory[a] = 0;
                    }
                    self.ip += 4;
                }
                99 => {
                    self.halted = true;
                    break
                }
                opcode => panic!(format!("Unrecognised opcode: {}, ip={}", opcode, self.ip)),
            }
        }
    }

    fn decode(instruction: i32) -> (i32, [u8; 3]) {
        (
            instruction % 100,
            [
                ((instruction / 100) % 10) as u8,
                ((instruction / 1000) % 10) as u8,
                ((instruction / 10000) % 10) as u8,
            ],
        )
    }
}

fn parse_instructions(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|s| i32::from_str(s).unwrap())
        .collect()
}

fn part1(instructions: &Vec<i32>) -> i32 {
    (0..5)
        .permutations(5)
        .map(|phases| {
            let mut signal = 0;
            let mut cpu = Computer::new(instructions.clone());

            for c in 0..phases.len() {
                cpu.input.push(phases[c]);
                cpu.input.push(signal);
                cpu.run();
                signal = cpu.output.pop().expect("Expected output");
                cpu.reset();
            }
            signal
        })
        .max()
        .unwrap()
}

fn part2(instructions: &Vec<i32>) -> i32 {
    let mut cpus: Vec<Computer> = (0..5)
        .map(|_| Computer::new(instructions.clone()))
        .collect();

    let mut max = 0;

    (5..10)
        .permutations(5)
        .map(|phase| {
            for i in 0..phase.len() {
                cpus[i].reset();
                cpus[i].input.push(phase[i]);
            }

            let mut signal = 0;
            'HALT: loop {
                for i in 0..cpus.len() {
                    cpus[i].push_input(signal);
                    cpus[i].run();
                    if cpus[i].halted { break 'HALT; }
                    signal = cpus[i].pop_output();
                }
                max = std::cmp::max(max, signal);
            }
            signal
        })
        .max()
        .unwrap();
    max
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let instructions = parse_instructions(input);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let instructions = parse_instructions("3,15,3,16,1002,16,10,16,1,16,15,\
            15,4,15,99,0,0".to_string());
        assert_eq!(part1(&instructions), 43210);
    }

    #[test]
    fn test_part1_2() {
        let instructions = parse_instructions("3,23,3,24,1002,24,10,24,1002,23,\
            -1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string());
        assert_eq!(part1(&instructions), 54321);
    }

    #[test]
    fn test_part1_3() {
        let instructions = parse_instructions("3,31,3,32,1002,32,10,32,1001,31,\
            -2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string());
        assert_eq!(part1(&instructions), 65210);
    }

    #[test]
    fn test_part2_1() {
        let instructions = parse_instructions("3,26,1001,26,-4,26,3,27,1002,27,2,\
        27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string());
        assert_eq!(part2(&instructions), 139629729);
    }

    #[test]
    fn test_part2_2() {
        let instructions = parse_instructions("3,52,1001,52,-5,52,3,53,1,52,56,\
        54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,\
        2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string());
        assert_eq!(part2(&instructions), 18216);
    }
}