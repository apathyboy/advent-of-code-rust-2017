advent_of_code::solution!(16);

fn perform_move(input: &[char], step: &str) -> Vec<char> {
    match &step[0..1] {
        "s" => {
            let amt = step[1..].parse::<usize>().unwrap();
            let left = &input[0..input.len() - amt].to_vec();
            let right = &input[input.len() - amt..].to_vec();

            right.iter().chain(left.iter()).cloned().collect()
        }
        "x" => {
            let mut result = Vec::from(input);
            let (a, b) = step[1..].split_once('/').unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();

            result[a] = input[b];
            result[b] = input[a];

            result
        }
        "p" => {
            let mut result = Vec::from(input);
            let a = step.chars().nth(1).unwrap();
            let b = step.chars().nth(3).unwrap();

            let (a, _) = input.iter().enumerate().find(|(_, &c)| c == a).unwrap();
            let (b, _) = input.iter().enumerate().find(|(_, &c)| c == b).unwrap();

            result[a] = input[b];
            result[b] = input[a];

            result
        }
        _ => panic!("invalid move"),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let dance_moves = input.trim().split(',').collect::<Vec<_>>();
    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    for step in dance_moves {
        programs = perform_move(&programs, step);
    }

    Some(programs.iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let dance_moves = input.trim().split(',').collect::<Vec<_>>();
    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    for _ in 0..1000000000_u32.rem_euclid(24) {
        for step in dance_moves.iter() {
            programs = perform_move(&programs, step);
        }
    }

    Some(programs.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(("s1", vec!['a', 'b','c', 'd', 'e']), vec!['e', 'a', 'b', 'c', 'd'])]
    #[case(("x3/4", vec!['e', 'a', 'b', 'c', 'd']), vec!['e', 'a', 'b', 'd', 'c'])]
    #[case(("pe/b", vec!['e', 'a', 'b', 'd', 'c']), vec!['b', 'a', 'e', 'd', 'c'])]
    fn test_perform_move(#[case] input: (&str, Vec<char>), #[case] expected: Vec<char>) {
        let result = perform_move(&input.1, input.0);

        assert_eq!(result, expected);
    }
}
