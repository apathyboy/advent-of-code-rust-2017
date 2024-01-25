use itertools::Itertools;

advent_of_code::solution!(7);

struct Program {
    name: String,
    weight: u32,
    total_weight: u32,
    supporting: Vec<String>,
}

impl Program {
    fn new(name: &str, weight: u32, supporting: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            weight,
            total_weight: 0,
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
    programs.iter().find(|&program| {
        !programs
            .iter()
            .any(|p| p.supporting.contains(&program.name))
    })
}

fn get_total_weight(name: &str, programs: &[Program]) -> Option<u32> {
    let program = programs.iter().find(|p| p.name == name)?;

    let mut supporting_weight = 0;

    for supported_program in program.supporting.iter() {
        supporting_weight += get_total_weight(supported_program, programs)?;
    }

    Some(supporting_weight + program.weight)
}

pub fn part_one(input: &str) -> Option<String> {
    let programs: Vec<Program> = input.lines().filter_map(parse_program).collect();

    let bottom_program = find_bottom_program(&programs)?;

    Some(bottom_program.name.clone())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut programs: Vec<Program> = input.lines().filter_map(parse_program).collect();
    let lookup: Vec<Program> = input.lines().filter_map(parse_program).collect();

    for program in programs.iter_mut() {
        program.total_weight = get_total_weight(&program.name, &lookup)?;
    }

    let mut weights: Vec<(u32, Vec<(u32, u32)>)> = Vec::new();

    for program in programs.iter() {
        if program.supporting.is_empty() {
            continue;
        }

        let mut supported_weights = Vec::new();

        for supported in program.supporting.iter() {
            let supported_program = programs.iter().find(|p| p.name == *supported)?;
            supported_weights.push((supported_program.weight, supported_program.total_weight));
        }

        weights.push((
            program.weight,
            supported_weights.into_iter().unique().collect::<Vec<_>>(),
        ));
    }

    let target = weights
        .iter()
        .filter(|(_, supporting)| supporting.iter().map(|(_, w)| w).unique().count() > 1)
        .min_by(|(_, a), (_, b)| {
            a.iter()
                .map(|(_, t)| *t)
                .min()
                .unwrap()
                .cmp(b.iter().map(|(_, t)| t).min().unwrap())
        })?;

    let r = target
        .1
        .iter()
        .map(|(_, w)| *w)
        .unique()
        .sorted()
        .collect::<Vec<_>>();
    let offset = r[1] - r[0];

    let (t, _) = target.1.iter().max_by(|(_, a), (_, b)| a.cmp(b))?;

    Some(t - offset)
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
        assert_eq!(result, Some(60));
    }
}
