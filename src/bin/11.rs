use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(11);

fn axial_distance(a: &IVec2, b: &IVec2) -> usize {
    ((a.x - b.x).unsigned_abs() as usize
        + (a.x + a.y - b.x - b.y).unsigned_abs() as usize
        + (a.y - b.y).unsigned_abs() as usize)
        / 2
}

fn follow(path: &[&str]) -> Option<(usize, usize)> {
    let dirs = HashMap::from([
        ("n", IVec2::new(1, -1)),
        ("ne", IVec2::new(1, 0)),
        ("se", IVec2::new(0, 1)),
        ("s", IVec2::new(-1, 1)),
        ("sw", IVec2::new(-1, 0)),
        ("nw", IVec2::new(0, -1)),
    ]);

    let mut current_position = IVec2::new(0, 0);
    let mut max_distance = 0;

    for step in path {
        current_position += *dirs.get(step)?;

        let distance = axial_distance(&IVec2::new(0, 0), &current_position);

        if distance > max_distance {
            max_distance = distance;
        }
    }

    Some((
        axial_distance(&IVec2::new(0, 0), &current_position),
        max_distance,
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let path: Vec<&str> = input.trim().split(',').collect();

    let (end_distance, _) = follow(&path)?;

    Some(end_distance)
}

pub fn part_two(input: &str) -> Option<usize> {
    let path: Vec<&str> = input.trim().split(',').collect();

    let (_, max_distance) = follow(&path)?;

    Some(max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["ne","ne","ne"], 3)]
    #[case(vec!["ne","ne","sw","sw"], 0)]
    #[case(vec!["ne","ne","s","s"], 2)]
    #[case(vec!["se","sw","se","sw","sw"], 3)]
    fn test_follow(#[case] input: Vec<&str>, #[case] expected: usize) {
        let (result, _) = follow(&input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
