use std::collections::{HashSet, VecDeque};

use advent_of_code::{knot_hash, to_binary_string, to_hex_string};
use glam::IVec2;

advent_of_code::solution!(14);

fn find_region_members(map: &[IVec2], pos: &IVec2) -> HashSet<IVec2> {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    let mut members = HashSet::new();
    let mut to_visit = VecDeque::from([*pos]);

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();

        members.insert(next);

        for dir in dirs {
            let check = next + dir;
            if map.contains(&check) && !members.contains(&check) {
                to_visit.push_back(check);
            }
        }
    }

    members
}

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

pub fn part_two(input: &str) -> Option<usize> {
    let key = input.trim();

    let map = (0..128)
        .flat_map(|i| {
            let hash = to_hex_string(&knot_hash(&format!("{key}-{i}")));
            let hash = hash
                .chars()
                .map(|c| usize::from_str_radix(&c.to_string(), 16).unwrap())
                .collect::<Vec<_>>();

            let binary = to_binary_string(&hash);

            binary
                .chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c == '1' {
                        Some(IVec2::new(j as i32, i))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut regions: Vec<HashSet<IVec2>> = Vec::new();

    for pos in map.iter() {
        if !regions.iter().any(|r| r.contains(&pos)) {
            regions.push(find_region_members(&map, &pos));
        }
    }

    Some(regions.len())
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
