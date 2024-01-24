advent_of_code::solution!(5);

fn count_steps_to_cycle_exit(input: &str, is_part_2: bool) -> Option<u32> {
    let mut jumps = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    let mut ip = 0;
    let mut steps = 0;

    while ip < jumps.len() {
        let offset = jumps[ip];

        if is_part_2 && offset >= 3 {
            jumps[ip] -= 1;
        } else {
            jumps[ip] += 1;
        }

        if offset < 0 {
            ip = ip.checked_sub(offset.unsigned_abs() as usize)?;
        } else {
            ip = ip.checked_add(offset as usize)?;
        }

        steps += 1;
    }

    Some(steps)
}

pub fn part_one(input: &str) -> Option<u32> {
    count_steps_to_cycle_exit(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    count_steps_to_cycle_exit(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
