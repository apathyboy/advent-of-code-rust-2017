use advent_of_code::parse_line_as_digits;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut digits = parse_line_as_digits(input);

    digits.push(digits[0]);

    let solution = digits
        .as_slice()
        .windows(2)
        .filter_map(|window| {
            if window[0] == window[1] {
                Some(window[0])
            } else {
                None
            }
        })
        .sum();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits = parse_line_as_digits(input);

    let solution = digits
        .iter()
        .enumerate()
        .filter_map(|(i, &digit)| {
            let check_pos = (i + (digits.len() / 2)).rem_euclid(digits.len());

            if digit == digits[check_pos] {
                Some(digit)
            } else {
                None
            }
        })
        .sum();

    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1122", 3)]
    #[case("1111", 4)]
    #[case("1234", 0)]
    #[case("91212129", 9)]
    fn test_part_one(#[case] input: &str, #[case] expected: u32) {
        let result = part_one(input);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("1212", 6)]
    #[case("1221", 0)]
    #[case("123425", 4)]
    #[case("123123", 12)]
    #[case("12131415", 4)]
    fn test_part_two(#[case] input: &str, #[case] expected: u32) {
        let result = part_two(input);
        assert_eq!(result, Some(expected));
    }
}
