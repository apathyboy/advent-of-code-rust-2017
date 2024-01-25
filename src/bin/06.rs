use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut banks = input
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut states = HashSet::from([banks.clone()]);

    loop {
        // find max bank value and index
        let (i, &val) = banks
            .iter()
            .enumerate()
            .max_by(|(i1, a), (i2, b)| a.cmp(b).then(i2.cmp(i1)))?;

        // set stored value at max bank index to 0
        banks[i] = 0;

        // start from index + 1 (wrapping as necesary) and distribute 1 to each bank until the value has been consumed
        let mut idx = i;

        for _ in 0..val {
            idx = (idx + 1).rem_euclid(banks.len());
            banks[idx] += 1;
        }

        // if current banks state is in the cached states exit the loop
        if states.contains(&banks) {
            break;
        }

        // otherwise cache the banks state and continue
        states.insert(banks.clone());
    }

    Some(states.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut banks = input
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let mut states = HashSet::from([banks.clone()]);
    let mut cycle_start = None;

    loop {
        // find max bank value and index
        let (i, &val) = banks
            .iter()
            .enumerate()
            .max_by(|(i1, a), (i2, b)| a.cmp(b).then(i2.cmp(i1)))?;

        // set stored value at max bank index to 0
        banks[i] = 0;

        // start from index + 1 (wrapping as necesary) and distribute 1 to each bank until the value has been consumed
        let mut idx = i;

        for _ in 0..val {
            idx = (idx + 1).rem_euclid(banks.len());
            banks[idx] += 1;
        }

        // if current banks state is in the cached states exit the loop
        if cycle_start.is_none() && states.contains(&banks) {
            cycle_start = Some(banks.clone());
            states.clear();
        } else if cycle_start.is_some() && cycle_start == Some(banks.clone()) {
            break;
        }

        // otherwise cache the banks state and continue
        states.insert(banks.clone());
    }

    Some(states.len())
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
        assert_eq!(result, Some(4));
    }
}
