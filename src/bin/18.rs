use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(18);

#[derive(Debug, Clone)]
enum Argument {
    Register(char),
    Value(i64),
}

impl Argument {
    fn new(arg: &str) -> Self {
        if arg.len() == 1 && arg.chars().next().unwrap().is_ascii_alphabetic() {
            Argument::Register(arg.chars().next().unwrap())
        } else {
            Argument::Value(arg.parse::<i64>().unwrap())
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd { arg1: Argument },
    Set { arg1: Argument, arg2: Argument },
    Add { arg1: Argument, arg2: Argument },
    Mul { arg1: Argument, arg2: Argument },
    Mod { arg1: Argument, arg2: Argument },
    Rcv { arg1: Argument },
    Jgz { arg1: Argument, arg2: Argument },
}

impl Instruction {
    fn snd(&self, arg1: &Argument, computer: &mut Duet) -> Option<i64> {
        let freq = match arg1 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        computer.instruction_pointer += 1;
        computer.send_counter += 1;

        Some(freq)
    }

    fn set(&self, arg1: &Argument, arg2: &Argument, computer: &mut Duet) -> Option<i64> {
        let addr = match arg1 {
            Argument::Register(register) => register,
            _ => panic!("Arg1 must be a register address"),
        };
        let val = match arg2 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        *computer.registers.get_mut(addr).unwrap() = val;

        computer.instruction_pointer += 1;

        None
    }

    fn add(&self, arg1: &Argument, arg2: &Argument, computer: &mut Duet) -> Option<i64> {
        let addr = match arg1 {
            Argument::Register(register) => register,
            _ => panic!("Arg1 must be a register address"),
        };
        let val = match arg2 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        *computer.registers.get_mut(addr).unwrap() += val;

        computer.instruction_pointer += 1;

        None
    }

    fn mul(&self, arg1: &Argument, arg2: &Argument, computer: &mut Duet) -> Option<i64> {
        let addr = match arg1 {
            Argument::Register(register) => register,
            _ => panic!("Arg1 must be a register address"),
        };
        let val = match arg2 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        *computer.registers.get_mut(addr).unwrap() *= val;

        computer.instruction_pointer += 1;

        None
    }

    fn modulo(&self, arg1: &Argument, arg2: &Argument, computer: &mut Duet) -> Option<i64> {
        let addr = match arg1 {
            Argument::Register(register) => register,
            _ => panic!("Arg1 must be a register address"),
        };
        let val = match arg2 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        *computer.registers.get_mut(addr).unwrap() =
            computer.registers.get(addr).unwrap().rem_euclid(val);

        computer.instruction_pointer += 1;

        None
    }

    fn rcv(&self, arg1: &Argument, computer: &mut Duet) -> Option<i64> {
        let val = match arg1 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        computer.waiting = false;

        if computer.is_v1 {
            computer.instruction_pointer += 1;

            if val != 0 && !computer.input_queue.is_empty() {
                computer.input_queue.pop_front()
            } else {
                None
            }
        } else if !computer.input_queue.is_empty() {
            computer.instruction_pointer += 1;
            let received = computer.input_queue.pop_front();

            if let Argument::Register(register) = arg1 {
                *computer.registers.get_mut(register).unwrap() = received?;
            }

            received
        } else {
            computer.waiting = true;
            None
        }
    }

    fn jgz(&self, arg1: &Argument, arg2: &Argument, computer: &mut Duet) -> Option<i64> {
        let val = match arg1 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };
        let jump = match arg2 {
            Argument::Register(register) => *computer.registers.get(register).unwrap(),
            Argument::Value(value) => *value,
        };

        if val > 0 {
            computer.instruction_pointer += jump;
        } else {
            computer.instruction_pointer += 1;
        }

        None
    }

    fn execute(&self, computer: &mut Duet) -> Option<i64> {
        match self {
            Instruction::Snd { arg1 } => self.snd(arg1, computer),
            Instruction::Set { arg1, arg2 } => self.set(arg1, arg2, computer),
            Instruction::Add { arg1, arg2 } => self.add(arg1, arg2, computer),
            Instruction::Mul { arg1, arg2 } => self.mul(arg1, arg2, computer),
            Instruction::Mod { arg1, arg2 } => self.modulo(arg1, arg2, computer),
            Instruction::Rcv { arg1 } => self.rcv(arg1, computer),
            Instruction::Jgz { arg1, arg2 } => self.jgz(arg1, arg2, computer),
        }
    }
}

struct Duet {
    instruction_pointer: i64,
    registers: HashMap<char, i64>,
    program: Vec<Instruction>,
    input_queue: VecDeque<i64>,
    exited: bool,
    send_counter: usize,
    waiting: bool,
    is_v1: bool,
}

impl Duet {
    fn new(registers: HashMap<char, i64>, program: Vec<Instruction>) -> Self {
        Self {
            instruction_pointer: 0,
            registers,
            program,
            input_queue: VecDeque::new(),
            exited: false,
            send_counter: 0,
            waiting: false,
            is_v1: false,
        }
    }

    fn run_next_instruction(&mut self) -> Option<i64> {
        if self.instruction_pointer < 0 || self.instruction_pointer >= self.program.len() as i64 {
            self.exited = true;
            return None;
        }

        let instruction = self.program[self.instruction_pointer as usize].clone();

        let output = instruction.execute(self);

        if output.is_some() {
            match instruction {
                Instruction::Snd { arg1: _ } => return output,
                Instruction::Rcv { arg1: _ } => return None,
                _ => panic!("invalid instruction return"),
            };
        }

        None
    }

    fn run(&mut self) -> Option<i64> {
        loop {
            if self.instruction_pointer < 0 || self.instruction_pointer >= self.program.len() as i64
            {
                break;
            }

            let instruction = self.program[self.instruction_pointer as usize].clone();

            let output = instruction.execute(self);

            if output.is_some() {
                match instruction {
                    Instruction::Snd { arg1: _ } => self.input_queue.push_front(output?),
                    Instruction::Rcv { arg1: _ } => return output,
                    _ => panic!("invalid instruction return"),
                };
            }
        }

        None
    }
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let (name, rest) = line.split_once(' ').unwrap();

    match name {
        "snd" => Some(Instruction::Snd {
            arg1: Argument::new(rest),
        }),
        "set" => {
            let (a, b) = rest.split_once(' ').unwrap();

            Some(Instruction::Set {
                arg1: Argument::new(a),
                arg2: Argument::new(b),
            })
        }
        "add" => {
            let (a, b) = rest.split_once(' ').unwrap();

            Some(Instruction::Add {
                arg1: Argument::new(a),
                arg2: Argument::new(b),
            })
        }
        "mul" => {
            let (a, b) = rest.split_once(' ').unwrap();

            Some(Instruction::Mul {
                arg1: Argument::new(a),
                arg2: Argument::new(b),
            })
        }
        "mod" => {
            let (a, b) = rest.split_once(' ').unwrap();

            Some(Instruction::Mod {
                arg1: Argument::new(a),
                arg2: Argument::new(b),
            })
        }
        "rcv" => Some(Instruction::Rcv {
            arg1: Argument::new(rest),
        }),
        "jgz" => {
            let (a, b) = rest.split_once(' ').unwrap();

            Some(Instruction::Jgz {
                arg1: Argument::new(a),
                arg2: Argument::new(b),
            })
        }
        _ => panic!("invalid instruction"),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let registers = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
        ('h', 0),
        ('i', 0),
        ('j', 0),
        ('k', 0),
        ('l', 0),
        ('m', 0),
        ('n', 0),
        ('o', 0),
        ('p', 0),
        ('q', 0),
        ('r', 0),
        ('s', 0),
        ('t', 0),
        ('u', 0),
        ('v', 0),
        ('w', 0),
        ('x', 0),
        ('y', 0),
        ('z', 0),
    ]);
    let program = input.lines().filter_map(parse_instruction).collect();
    let mut duet = Duet::new(registers, program);
    duet.is_v1 = true;

    duet.run()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut registers = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
        ('h', 0),
        ('i', 0),
        ('j', 0),
        ('k', 0),
        ('l', 0),
        ('m', 0),
        ('n', 0),
        ('o', 0),
        ('p', 0),
        ('q', 0),
        ('r', 0),
        ('s', 0),
        ('t', 0),
        ('u', 0),
        ('v', 0),
        ('w', 0),
        ('x', 0),
        ('y', 0),
        ('z', 0),
    ]);
    let program: Vec<Instruction> = input.lines().filter_map(parse_instruction).collect();
    let mut p0 = Duet::new(registers.clone(), program.clone());

    *registers.get_mut(&'p').unwrap() = 1;

    let mut p1 = Duet::new(registers.clone(), program.clone());

    loop {
        let mut output0: Option<i64> = None;
        let mut output1: Option<i64> = None;

        if !p0.exited {
            output0 = p0.run_next_instruction();
        }

        if !p1.exited {
            output1 = p1.run_next_instruction();
        }

        if (p0.exited && p1.exited) || (p0.waiting && p1.waiting) {
            break;
        }

        if !p0.exited && output1.is_some() {
            p0.input_queue.push_back(output1?);
        }

        if !p1.exited && output0.is_some() {
            p1.input_queue.push_back(output0?);
        }
    }

    Some(p1.send_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
