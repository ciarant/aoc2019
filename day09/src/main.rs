use std::str::FromStr;
use std::io::Read;
use std::io;

struct Computer {
    ip: i64,
    memory: Vec<i64>,
    memory_orig: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    halted: bool,
    base: i64
}

enum Status {
    AwaitingInput,
    Halted
}

impl Computer {
    fn new(memory: Vec<i64>) -> Computer {
        let mut memory = memory.clone();
        memory.extend(vec![0i64; 10000]);
        Computer {
            ip: 0,
            memory: memory.clone(),
            memory_orig: memory,
            input: Vec::new(),
            output: Vec::new(),
            halted: false,
            base: 0
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_orig.clone();
        self.input.clear();
        self.output.clear();
        self.halted = false;
        self.base = 0;
    }

    fn peek(&mut self, address: i64, mode: u8) -> i64 {
        let address = self.get_address(address, mode);
        self.memory[address as usize]
    }

    fn poke(&mut self, address: i64, value: i64, mode: u8) -> i64{
        let address = self.get_address(address, mode);
        self.memory[address as usize] = value;
        address
    }

    fn get_address(&mut self, address: i64, mode: u8) -> i64 {
        match mode {
            0 => self.memory[address as usize],
            1 => address,
            2 => self.memory[address as usize] + self.base,
            m => panic!("Unknown mode: {}", m)
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

    fn run(&mut self) -> Status {
        loop {
            let (opcode, modes) = Computer::decode(self.memory[self.ip as usize] as i32);
            match opcode {
                1 => {
                    // add
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    self.poke(self.ip + 3, x + y, modes[2]);
                    self.ip += 4;
                }
                2 => {
                    // multiply
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    self.poke(self.ip + 3, x * y, modes[2]);
                    self.ip += 4;
                }
                3 => {
                    // input
                    if self.input.is_empty() {
                        return Status::AwaitingInput
                    }
                    let value = self.input.remove(0);
                    self.poke(self.ip + 1, value, modes[0]);
                    self.ip += 2;
                }
                4 => {
                    // output
                    let a = self.peek(self.ip + 1, modes[0]);
                    self.output.push(a);
                    self.ip += 2;
                }
                5 => {
                    // jump if true
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x != 0 {
                        self.ip = y;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // jump if false
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x == 0 {
                        self.ip = y;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // set less than
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x < y {
                        self.poke(self.ip + 3, 1, modes[2]);
                    } else {
                        self.poke(self.ip + 3, 0, modes[2]);
                    }
                    self.ip += 4;
                }
                8 => {
                    // set equal to
                    let x = self.peek(self.ip + 1, modes[0]);
                    let y = self.peek(self.ip + 2, modes[1]);
                    if x == y {
                        self.poke(self.ip + 3, 1, modes[2]);
                    } else {
                        self.poke(self.ip + 3, 0, modes[2]);
                    }
                    self.ip += 4;
                }
                9 => {
                    // relative base
                    let val= self.peek(self.ip + 1, modes[0]);
                    self.base += val;
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    self.ip += 1;
                    return Status::Halted
                }
                opcode => panic!(format!("Unrecognised opcode: {}, ip={}", opcode, self.ip)),
            }
        }
    }
}

fn parse_instructions(input: String) -> Vec<i64> {
    input
        .split(",")
        .map(|s| i64::from_str(s).unwrap())
        .collect()
}

fn part1(instructions: &Vec<i64>) -> Vec<i64> {
    let mut cpu = Computer::new(instructions.clone());
    cpu.input.push(1);
    cpu.run();
    cpu.output
}

fn part2(instructions: &Vec<i64>) -> Vec<i64> {
    let mut cpu = Computer::new(instructions.clone());
    cpu.input.push(2);
    cpu.run();
    cpu.output
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
//    let input = fs::read_to_string("data/input.txt").expect("Error reading file");
//    let input = fs::read_to_string("data/input.txt").expect("Error reading file");
    let instructions = parse_instructions(input);

    println!("Part 1: {}", part1(&instructions).pop().unwrap());
    println!("Part 2: {}", part2(&instructions).pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_1() {
        let mut input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let instructions = parse_instructions(input.clone());
        let mut cpu = Computer::new(instructions.clone());
        cpu.run();
        let output = cpu.output.iter().map(|i| format!("{},", i.to_string())).collect::<String>();
        input.push(',');
        assert_eq!(output, input);
    }

    #[test]
    fn day09_2() {
        let instructions = vec![1102,34915192,34915192,7,4,7,99,0];
        assert_eq!(part1(&instructions).pop().unwrap().to_string().len(), 16);
    }

    #[test]
    fn day09_3() {
        let instructions = parse_instructions("104,1125899906842624,99".to_string());
        assert_eq!(part1(&instructions).pop().unwrap().clone(), 1125899906842624i64);
    }

    #[test]
    fn test1() {
        let instructions = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test2() {
        let instructions = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test3() {
        let instructions = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test4() {
        let instructions = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test5() {
        let instructions = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test6() {
        let instructions = vec![3, 0, 4, 0, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run(); // TODO assertion
    }

    #[test]
    fn test7() {
        let instructions = vec![1002, 4, 3, 4, 33];
        let expected: Vec<i64> = vec![1002, 4, 3, 4, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.memory[..expected.len()], expected[..]);
    }

    #[test]
    fn test8() {
        let instructions = vec![109, -1, 4, 1, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), -1);
    }

    #[test]
    fn test9() {
        let instructions = vec![109, -1, 104, 1, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
    }

    #[test]
    fn test10() {
        let instructions = vec![109, -1, 204, 1, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 109);
    }

    #[test]
    fn test11() {
        let instructions = vec![109, 1, 9, 2, 204, -6, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 204);
    }

    #[test]
    fn test12() {
        let instructions = vec![109, 1, 109, 9, 204, -6, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 204);
    }

    #[test]
    fn test13() {
        let instructions = vec![109, 1, 209, -1, 204, -106, 99];
        let mut cpu = Computer::new(instructions);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 204);
    }

    #[test]
    fn test14() {
        let instructions = vec![109, 1, 3, 3, 204, 2, 99];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(555);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 555);
    }

    #[test]
    fn test15() {
        let instructions = vec![109, 1, 203, 2, 204, 2, 99];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(555);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 555);
    }

    #[test]
    fn day02_1() {
        let input = fs::read_to_string("../day02/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.memory[1] = 12;
        cpu.memory[2] = 2;
        cpu.run();
        assert_eq!(cpu.memory[0], 4484226);
    }

    #[test]
    fn day02_2() {
        let input = fs::read_to_string("../day02/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.memory[1] = 56;
        cpu.memory[2] = 96;
        cpu.run();
        assert_eq!(cpu.memory[0], 19690720);
    }

    #[test]
    fn day05_0() {
        let instructions = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
        cpu.reset();
        cpu.input.push(7);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
    }

    #[test]
    fn day05_1() {
        let instructions = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
        cpu.reset();
        cpu.input.push(7);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
    }

    #[test]
    fn day05_2() {
        let instructions = vec![3,3,1108,-1,8,3,4,3,99];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
        cpu.reset();
        cpu.input.push(7);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
    }

    #[test]
    fn day05_3() {
        let instructions = vec![3,3,1107,-1,8,3,4,3,99];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(7);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
        cpu.reset();
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
    }

    #[test]
    fn day05_4() {
        let instructions = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(0);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
        cpu.reset();
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
    }

    #[test]
    fn day05_5() {
        let instructions = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(0);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 0);
        cpu.reset();
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1);
    }

    #[test]
    fn day05_6() {
        let instructions = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let mut cpu = Computer::new(instructions);
        cpu.input.push(7);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 999);
        cpu.reset();
        cpu.input.push(8);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1000);
        cpu.reset();
        cpu.input.push(9);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 1001);
    }

    #[test]
    fn day02_part1() {
        let input = fs::read_to_string("../day02/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.memory[1] = 12;
        cpu.memory[2] = 2;
        cpu.run();
        assert_eq!(cpu.memory[0], 4484226);
    }

    #[test]
    fn day02_part2() {
        let input = fs::read_to_string("../day02/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.memory[1] = 56;
        cpu.memory[2] = 96;
        cpu.run();
        assert_eq!(cpu.memory[0], 19690720);
    }

    #[test]
    fn day05_part1() {
        let input = fs::read_to_string("../day05/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.input.push(1);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 15314507);
    }

    #[test]
    fn day05_part2() {
        let input = fs::read_to_string("../day05/data/input.txt").expect("Error reading file");
        let instructions = parse_instructions(input);
        let mut cpu = Computer::new(instructions);
        cpu.input.push(5);
        cpu.run();
        assert_eq!(cpu.output.pop().unwrap(), 652726);
    }
}