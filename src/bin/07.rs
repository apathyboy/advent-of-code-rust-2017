use itertools::Itertools;

advent_of_code::solution!(7);

struct Program {
    name: String,
    weight: u32,
    supporting: Vec<String>,
}

impl Program {
    fn new(name: &str, weight: u32, supporting: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            weight,
            supporting,
        }
    }
}

fn parse_program(line: &str) -> Option<Program> {
    let (name, rest) = line.split_once(' ').unwrap();

    let (weight, supporting) = if rest.contains("->") {
        let (weight, supporting) = rest.split_once(" -> ").unwrap();
        let weight = &weight[1..weight.len() - 1];
        let supporting = supporting
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        (weight.parse::<u32>().ok()?, supporting)
    } else {
        let weight = &rest[1..rest.len() - 1];

        (weight.parse::<u32>().ok()?, Vec::new())
    };

    Some(Program::new(name, weight, supporting))
}

fn find_bottom_program(programs: &[Program]) -> Option<&Program> {
    for program in programs.iter() {
        if !programs
            .iter()
            .any(|p| p.supporting.contains(&program.name))
        {
            return Some(program);
        }
    }

    None
}

fn get_total_weight(name: &str, programs: &[Program]) -> Option<u32> {
    let program = programs.iter().find(|p| &p.name == name)?;

    let mut supporting_weight = 0;

    for supported_program in program.supporting.iter() {
        supporting_weight += get_total_weight(&supported_program, programs)?;
    }

    Some(supporting_weight + program.weight)
}

pub fn part_one(input: &str) -> Option<String> {
    let programs: Vec<Program> = input.lines().filter_map(parse_program).collect();

    let bottom_program = find_bottom_program(&programs)?;

    Some(bottom_program.name.clone())
}

pub fn part_two(input: &str) -> Option<u32> {
    let programs: Vec<Program> = input.lines().filter_map(parse_program).collect();

    let bottom_program = find_bottom_program(&programs)?;

    let weights: Vec<u32> = bottom_program
        .supporting
        .iter()
        .filter_map(|name| get_total_weight(&name, &programs))
        .unique()
        .sorted()
        .collect();

    weights[1].checked_sub(weights[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("tknk")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
