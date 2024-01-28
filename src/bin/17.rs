advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let spins = input.trim().parse::<usize>().unwrap();

    let mut buffer = Vec::from([0]);
    let mut idx = 0;

    for i in 1..=2017_usize {
        idx = (idx + spins).rem_euclid(buffer.len()) + 1;

        buffer.insert(idx, i);
    }

    Some(buffer[(idx + 1).rem_euclid(buffer.len())])
}

pub fn part_two(input: &str) -> Option<usize> {
    let spins = input.trim().parse::<usize>().unwrap();

    let mut val = 0;
    let mut idx = 0;

    for i in 1..=50000000_usize {
        idx = (idx + spins).rem_euclid(i) + 1;

        if (idx + spins).rem_euclid(i) + 1 == 1 {
            val = i;
        }
    }

    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(638));
    }
}
