use primal::is_prime;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let mut num = input.lines().next().unwrap()[6..].parse::<u64>().unwrap();

    num -= 2;

    Some(num * num)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut num = input.lines().next().unwrap()[6..].parse::<u64>().unwrap();
    num = num * 100 + 100000;

    let mut non_primes = 0;

    for i in (num..=num + 17000).step_by(17) {
        if !is_prime(i) {
            non_primes += 1;
        }
    }

    Some(non_primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
