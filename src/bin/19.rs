use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(19);

fn find_next_pos(
    current_pos: &IVec2,
    current_dir: &IVec2,
    map: &HashMap<IVec2, char>,
) -> Option<(IVec2, IVec2)> {
    let test_pos = *current_pos + *current_dir;
    let test_dir = *current_dir;

    if map.contains_key(&test_pos) {
        return Some((test_pos, test_dir));
    }

    if current_dir.x != 0 {
        let test_dir = IVec2::new(0, 1);
        let test_pos = *current_pos + test_dir;

        if map.contains_key(&test_pos) {
            return Some((test_pos, test_dir));
        }

        let test_dir = IVec2::new(0, -1);
        let test_pos = *current_pos + test_dir;

        if map.contains_key(&test_pos) {
            return Some((test_pos, test_dir));
        }
    } else {
        let test_dir = IVec2::new(1, 0);
        let test_pos = *current_pos + test_dir;

        if map.contains_key(&test_pos) {
            return Some((test_pos, test_dir));
        }

        let test_dir = IVec2::new(-1, 0);
        let test_pos = *current_pos + test_dir;

        if map.contains_key(&test_pos) {
            return Some((test_pos, test_dir));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<String> {
    let map: HashMap<IVec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c != ' ' {
                    Some((IVec2::new(j as i32, i as i32), c))
                } else {
                    None
                }
            })
        })
        .collect();

    let (pos, _) = map.iter().find(|(pos, _)| pos.y == 0)?;

    let mut current_pos = *pos;
    let mut current_dir = IVec2::new(0, 1);

    let mut found: Vec<char> = Vec::new();

    loop {
        let next = find_next_pos(&current_pos, &current_dir, &map);

        if next.is_none() {
            break;
        }

        let (next_pos, next_dir) = next?;
        current_pos = next_pos;
        current_dir = next_dir;

        let tile = *map.get(&next_pos).unwrap();

        if tile.is_ascii_alphabetic() {
            found.push(tile);
        }
    }

    Some(found.iter().collect())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: HashMap<IVec2, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c != ' ' {
                    Some((IVec2::new(j as i32, i as i32), c))
                } else {
                    None
                }
            })
        })
        .collect();

    let (pos, _) = map.iter().find(|(pos, _)| pos.y == 0)?;

    let mut current_pos = *pos;
    let mut current_dir = IVec2::new(0, 1);

    let mut steps = 0;

    loop {
        let next = find_next_pos(&current_pos, &current_dir, &map);

        if next.is_none() {
            break;
        }

        let (next_pos, next_dir) = next?;
        current_pos = next_pos;
        current_dir = next_dir;

        steps += 1;
    }

    Some(steps + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("ABCDEF")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }
}
