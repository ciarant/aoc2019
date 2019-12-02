use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let instructions: Vec<usize> = input.split(",").map(|s| usize::from_str(s).unwrap()).collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &Vec<usize>) {
    let mut copy = instructions.clone();

    copy[1] = 12;
    copy[2] = 2;

    let _ = run(&mut copy);
    println!("{}", copy[0]);
}

fn part2(instructions: &Vec<usize>) {
    for noun in 0..99usize {
        for verb in 0..99usize {
            let mut code = instructions.clone();
            code[1] = noun;
            code[2] = verb;
            let _ = run(&mut code);
            if code[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn run(instructions: &mut Vec<usize>) -> Result<(), String> {
    let mut ip = 0usize;

    loop {
        match instructions[ip] {
            1 => {
                let x = instructions[ip + 1];
                let y = instructions[ip + 2];
                let a = instructions[ip + 3];
                instructions[a] = instructions[x] + instructions[y];
                ip += 4;
            }
            2 => {
                let x = instructions[ip + 1];
                let y = instructions[ip + 2];
                let a = instructions[ip + 3];
                instructions[a] = instructions[x] * instructions[y];
                ip += 4;
            }
            99 => return Ok(()),
            opcode => return Err(format!("Unrecognised opcode: {}", opcode))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut instructions = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let _ = run(instructions.as_mut());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn test2() {
        let mut instructions = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];
        let _ =  run(instructions.as_mut());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn test3() {
        let mut instructions = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        let _ =  run(instructions.as_mut());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn test4() {
        let mut instructions = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let _ =  run(instructions.as_mut());
        assert_eq!(expected, instructions);
    }

    #[test]
    fn test5() {
        let mut instructions = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let _ =  run(instructions.as_mut());
        assert_eq!(expected, instructions);
    }
}
