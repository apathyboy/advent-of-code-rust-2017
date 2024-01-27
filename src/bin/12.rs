use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(12);

fn parse_line(line: &str) -> Option<(u32, Vec<u32>)> {
    let (program, neighbors) = line.split_once(" <-> ")?;

    let program = program.parse::<u32>().ok()?;
    let neighbors: Vec<u32> = neighbors
        .split(", ")
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    Some((program, neighbors))
}

fn find_containing_group(pipes: &HashMap<u32, Vec<u32>>, val: u32) -> HashSet<u32> {
    let mut nums = HashSet::new();
    let mut to_visit = VecDeque::from([val]);

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();

        nums.insert(next);

        for neighbor in pipes.get(&next).unwrap() {
            if !nums.contains(neighbor) {
                to_visit.push_back(*neighbor);
            }
        }
    }

    nums
}

fn next_not_in_discovered(
    pipes: &HashMap<u32, Vec<u32>>,
    discovered_groups: &[HashSet<u32>],
) -> Option<u32> {
    pipes
        .keys()
        .find(|&key| !discovered_groups.iter().any(|v| v.contains(key)))
        .cloned()
}

pub fn part_one(input: &str) -> Option<usize> {
    let pipes = input
        .lines()
        .filter_map(parse_line)
        .collect::<HashMap<_, _>>();

    let group = find_containing_group(&pipes, 0);

    Some(group.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let pipes = input
        .lines()
        .filter_map(parse_line)
        .collect::<HashMap<_, _>>();

    let mut discovered_groups = Vec::new();

    while let Some(program) = next_not_in_discovered(&pipes, &discovered_groups) {
        discovered_groups.push(find_containing_group(&pipes, program));
    }

    Some(discovered_groups.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
