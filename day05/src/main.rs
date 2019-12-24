use std::fs;
use std::str::FromStr;

struct Computer {
    memory: Vec<i32>,
}

impl Computer {
    fn new(memory: Vec<i32>) -> Computer {
        Computer {
            memory: memory.clone(),
        }
    }

    fn peek(&self, value: usize, mode: u8) -> i32 {
        let x = match mode {
            0 => self.memory[self.memory[value] as usize] as i32,
            1 => self.memory[value] as i32,
            m => panic!("Unknown mode: {}", m),
        };
        x
    }

    fn run(&mut self, inputs: Vec<i32>) -> Result<(), String> {
        let mut ip = 0usize;
        let mut input_index = 0;

        loop {
            let (opcode, modes) = Computer::decode(self.memory[ip] as i32);
            match opcode {
                1 => {
                    // add
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    let a = self.memory[ip + 3] as usize;
                    self.memory[a] = x + y;
                    ip += 4;
                }
                2 => {
                    // multiply
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    let a = self.memory[ip + 3] as usize;
                    self.memory[a] = x * y;
                    ip += 4;
                }
                3 => {
                    // input
                    let a = self.memory[ip + 1] as usize;
                    self.memory[a] = inputs[input_index];
                    input_index += 1;
                    ip += 2;
                }
                4 => {
                    // print
                    let a = self.memory[ip + 1] as usize;
                    println!(">>> {}", self.memory[a]);
                    ip += 2;
                }
                5 => {
                    // jump if true
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    if x != 0 {
                        ip = y as usize;
                    } else {
                        ip += 3;
                    }
                }
                6 => {
                    // jump if false
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    if x == 0 {
                        ip = y as usize;
                    } else {
                        ip += 3;
                    }
                }
                7 => {
                    // less than
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    let a = self.memory[ip + 3] as usize;
                    if x < y {
                        self.memory[a] = 1;
                    } else {
                        self.memory[a] = 0;
                    }
                    ip += 4;
                }
                8 => {
                    // equals
                    let x = self.peek(ip + 1, modes[0]);
                    let y = self.peek(ip + 2, modes[1]);
                    let a = self.memory[ip + 3] as usize;
                    if x == y {
                        self.memory[a] = 1;
                    } else {
                        self.memory[a] = 0;
                    }
                    ip += 4;
                }
                99 => return Ok(()),
                opcode => return Err(format!("Unrecognised opcode: {}, ip={}", opcode, ip)),
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

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let instructions: Vec<i32> = input
        .split(",")
        .map(|s| i32::from_str(s).unwrap())
        .collect();

    let mut cpu = Computer::new(instructions.clone());
    println!("Part 1");
    let _ = cpu.run(vec![1]).unwrap();

    let mut cpu = Computer::new(instructions);
    println!("Part 2");
    let _ = cpu.run(vec![5]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        let (opcode, modes) = Computer::decode(1);
        assert_eq!(opcode, 1);
        assert_eq!(modes, [0, 0, 0]);
        let (opcode, modes) = Computer::decode((11102));
        assert_eq!(opcode, 2);
        assert_eq!(modes, [1, 1, 1]);
    }

    #[test]
    fn test1() {
        let mut instructions = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
    }

    #[test]
    fn test2() {
        let mut instructions = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
    }

    #[test]
    fn test3() {
        let mut instructions = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
    }

    #[test]
    fn test4() {
        let mut instructions = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
    }

    #[test]
    fn test5() {
        let mut instructions = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
    }

    #[test]
    fn test6() {
        let mut instructions = vec![3, 0, 4, 0, 99];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![421]);
    }

    #[test]
    fn test7() {
        let mut instructions = vec![1002, 4, 3, 4, 33];
        let mut expected = vec![1002, 4, 3, 4, 99];
        let mut cpu = Computer::new(instructions);
        let _ = cpu.run(vec![]);
        assert_eq!(cpu.memory, expected);
        println!("Hello")
    }
}
