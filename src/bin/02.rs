use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let checksum = input
        .lines()
        .filter_map(|line| {
            if let MinMax(min, max) = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .minmax()
            {
                Some(max - min)
            } else {
                None
            }
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let checksum = input
        .lines()
        .filter_map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .permutations(2)
                .find_map(|p| {
                    if p[0].rem_euclid(p[1]) == 0 {
                        Some(p[0] / p[1])
                    } else {
                        None
                    }
                })
        })
        .sum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
