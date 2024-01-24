use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(3);

fn get_value(memory: &HashMap<IVec2, usize>, pos: &IVec2) -> usize {
    let dirs = [
        IVec2::new(1, 0),
        IVec2::new(1, -1),
        IVec2::new(0, -1),
        IVec2::new(-1, -1),
        IVec2::new(-1, 0),
        IVec2::new(-1, 1),
        IVec2::new(0, 1),
        IVec2::new(1, 1),
    ];

    let mut val = 0;

    for dir in dirs {
        let test_pos = *pos + dir;

        let test_val = memory.get(&test_pos);

        if test_val.is_some() {
            val += *test_val.unwrap();
        }
    }

    if val == 0 {
        val = 1;
    }

    val
}

pub fn part_one(input: &str) -> Option<u32> {
    let target_memory_id = input.trim().parse::<usize>().unwrap();
    let mut ring_max: usize = 0;
    let mut ring_idx: usize = 0;
    let mut stride: usize = 0;
    let mut cur_dir: usize = 3;
    let mut cur = IVec2::new(0, 0);

    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(1, 0),
    ];

    for _ in 1..target_memory_id {
        if ring_idx == ring_max {
            ring_idx = 1;
            ring_max += 8;
            stride += 2;
        } else {
            ring_idx += 1;
        }

        if ring_idx == 1 {
            cur_dir = 3;
        } else if ring_idx == 2 {
            cur_dir = 0;
        } else if (ring_idx - 1).rem_euclid(stride) == 0 {
            cur_dir += 1;
        }

        cur += dirs[cur_dir];
    }

    Some(cur.x.unsigned_abs() + cur.y.unsigned_abs())
}

pub fn part_two(input: &str) -> Option<usize> {
    let target_memory_id = input.trim().parse::<usize>().unwrap();
    let mut ring_max: usize = 0;
    let mut ring_idx: usize = 0;
    let mut stride: usize = 0;
    let mut cur_dir: usize = 3;
    let mut cur = IVec2::new(0, 0);

    let mut memory = HashMap::new();

    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(1, 0),
    ];

    let mut value = 1;

    for _ in 1.. {
        value = get_value(&memory, &cur);

        if value > target_memory_id {
            break;
        }

        memory.insert(cur, value);

        if ring_idx == ring_max {
            ring_idx = 1;
            ring_max += 8;
            stride += 2;
        } else {
            ring_idx += 1;
        }

        if ring_idx == 1 {
            cur_dir = 3;
        } else if ring_idx == 2 {
            cur_dir = 0;
        } else if (ring_idx - 1).rem_euclid(stride) == 0 {
            cur_dir += 1;
        }

        cur += dirs[cur_dir];
    }

    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", 0)]
    #[case("12", 3)]
    #[case("23", 2)]
    #[case("1024", 31)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("0", 1)]
    #[case("1", 2)]
    #[case("747", 806)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
