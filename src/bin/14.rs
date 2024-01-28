use advent_of_code::{knot_hash, to_binary_string};
use glam::UVec2;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let key = input.trim();

    let total_used = (0..128)
        .map(|i| {
            to_binary_string(&knot_hash(&format!("{key}-{i}")))
                .chars()
                .filter(|c| *c == '1')
                .count()
        })
        .sum();

    Some(total_used)
}

pub fn part_two(input: &str) -> Option<u32> {
    let key = input.trim();

    let map = (0..128)
        .flat_map(|i| {
            to_binary_string(&knot_hash(&format!("{key}-{i}")))
                .chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '1' {
                        Some(UVec2::new(j as u32, i))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8108));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1242));
    }
}
