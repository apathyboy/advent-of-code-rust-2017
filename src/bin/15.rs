use itertools::Itertools;

advent_of_code::solution!(15);

const GENERATOR_A_FACTOR: u64 = 16807;
const GENERATOR_B_FACTOR: u64 = 48271;
const DIVISOR: u64 = 2147483647;

fn generate(seed: u64, factor: u64) -> u64 {
    (seed * factor).rem_euclid(DIVISOR)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a_seed, mut b_seed) = input
        .lines()
        .filter_map(|s| s[24..].parse::<u64>().ok())
        .collect_tuple()?;

    let mut counter = 0;

    for _ in 0..40000000 {
        a_seed = generate(a_seed, GENERATOR_A_FACTOR);
        b_seed = generate(b_seed, GENERATOR_B_FACTOR);

        if (a_seed & 0xFFFF) == (b_seed & 0xFFFF) {
            counter += 1;
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut a_seed, mut b_seed) = input
        .lines()
        .filter_map(|s| s[24..].parse::<u64>().ok())
        .collect_tuple()?;

    let mut counter = 0;

    let mut a_vals = Vec::new();
    let mut b_vals = Vec::new();

    for _ in 0.. {
        a_seed = generate(a_seed, GENERATOR_A_FACTOR);
        b_seed = generate(b_seed, GENERATOR_B_FACTOR);

        if a_seed.rem_euclid(4) == 0 {
            a_vals.push(a_seed);
        }

        if b_seed.rem_euclid(8) == 0 {
            b_vals.push(b_seed);
        }

        if a_vals.len() >= 5000000 && b_vals.len() >= 5000000 {
            break;
        }
    }

    for (a, b) in a_vals.iter().zip(b_vals.iter()) {
        if (a & 0xFFFF) == (b & 0xFFFF) {
            counter += 1;
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(309));
    }
}
