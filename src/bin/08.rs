use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
enum Operation {
    Increment,
    Decrement,
}

impl Operation {
    fn new(input: &str) -> Self {
        match input {
            "inc" => Operation::Increment,
            "dec" => Operation::Decrement,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug)]
enum Comparison {
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    EqualTo,
    NotEqualTo,
}

impl Comparison {
    fn new(input: &str) -> Self {
        match input {
            ">" => Comparison::GreaterThan,
            "<" => Comparison::LessThan,
            ">=" => Comparison::GreaterThanOrEqualTo,
            "<=" => Comparison::LessThanOrEqualTo,
            "==" => Comparison::EqualTo,
            "!=" => Comparison::NotEqualTo,
            _ => panic!("Invalid comparison"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    target: String,
    operation: Operation,
    value: i32,
    operand1: String,
    comparison: Comparison,
    operand2: i32,
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();

    let target = parts[0].to_string();
    let operation = Operation::new(parts[1]);
    let value = parts[2].parse::<i32>().ok()?;
    let operand1 = parts[4].to_string();
    let comparison = Comparison::new(parts[5]);
    let operand2 = parts[6].parse::<i32>().ok()?;

    Some(Instruction {
        target,
        operation,
        value,
        operand1,
        comparison,
        operand2,
    })
}

fn run_instruction(instruction: &Instruction, registers: &mut HashMap<String, i32>) {
    let operand1 = *registers.get(&instruction.operand1).unwrap();
    let target = registers.get_mut(&instruction.target).unwrap();

    if match instruction.comparison {
        Comparison::EqualTo => operand1 == instruction.operand2,
        Comparison::GreaterThan => operand1 > instruction.operand2,
        Comparison::LessThan => operand1 < instruction.operand2,
        Comparison::GreaterThanOrEqualTo => operand1 >= instruction.operand2,
        Comparison::LessThanOrEqualTo => operand1 <= instruction.operand2,
        Comparison::NotEqualTo => operand1 != instruction.operand2,
    } {
        *target += match instruction.operation {
            Operation::Decrement => -instruction.value,
            Operation::Increment => instruction.value,
        };
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().filter_map(parse_instruction).collect();
    let mut registers: HashMap<String, i32> = HashMap::new();

    for instruction in instructions.iter() {
        if !registers.contains_key(&instruction.target) {
            registers.insert(instruction.target.clone(), 0);
        }

        if !registers.contains_key(&instruction.operand1) {
            registers.insert(instruction.operand1.clone(), 0);
        }
    }

    instructions
        .iter()
        .for_each(|i| run_instruction(i, &mut registers));

    registers.into_values().max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions: Vec<Instruction> = input.lines().filter_map(parse_instruction).collect();
    let mut registers: HashMap<String, i32> = HashMap::new();

    for instruction in instructions.iter() {
        if !registers.contains_key(&instruction.target) {
            registers.insert(instruction.target.clone(), 0);
        }

        if !registers.contains_key(&instruction.operand1) {
            registers.insert(instruction.operand1.clone(), 0);
        }
    }

    let mut max_val = 0;

    for instruction in instructions.iter() {
        run_instruction(instruction, &mut registers);

        let current_max = registers.clone().into_values().max()?;

        if current_max > max_val {
            max_val = current_max;
        }
    }

    Some(max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
