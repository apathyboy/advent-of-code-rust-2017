use advent_of_code::{knot_hash, to_hex_string, twist};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let mut current_pos = 0;
    let mut skip_length = 0;
    let result = twist(
        &lengths,
        (0..256).collect::<Vec<_>>(),
        &mut current_pos,
        &mut skip_length,
    );

    Some(result[0] * result[1])
}

pub fn part_two(input: &str) -> Option<String> {
    Some(to_hex_string(&knot_hash(input.trim())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_twist() {
        let lengths = vec![3, 4, 1, 5];
        let mut current_pos = 0;
        let mut skip_length = 0;
        let result = twist(
            &lengths,
            (0..5).collect::<Vec<_>>(),
            &mut current_pos,
            &mut skip_length,
        );

        assert_eq!(result, vec![3, 4, 2, 1, 0]);
    }

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272")]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd")]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e")]
    fn test_knot_hash(#[case] input: &str, #[case] expected: &str) {
        let result = knot_hash(input);
        assert_eq!(to_hex_string(&result), String::from(expected));
    }
}
