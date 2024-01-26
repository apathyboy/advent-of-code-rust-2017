advent_of_code::solution!(9);

fn score_stream(stream: &str) -> Option<(usize, usize)> {
    let mut depth = 0;
    let mut score = 0;
    let mut in_garbage = false;
    let mut garbage = 0;
    let mut ignore = false;

    for c in stream.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        if c == '!' {
            ignore = true;
            continue;
        }

        if c == '<' && !in_garbage {
            in_garbage = true;
            continue;
        }

        if c == '>' && in_garbage {
            in_garbage = false;
            continue;
        }

        if in_garbage {
            garbage += 1;
            continue;
        }

        if c == '}' {
            depth -= 1;
            continue;
        }

        if c == '{' {
            depth += 1;
            score += depth;
        }
    }

    Some((score, garbage))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (score, _) = score_stream(input.trim())?;

    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, garbage) = score_stream(input.trim())?;

    Some(garbage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("<>", 0)]
    #[case("<random characters>", 17)]
    #[case("<<<<>", 3)]
    #[case("<{!>}>", 2)]
    #[case("<!!>", 0)]
    #[case("<!!!>>", 0)]
    #[case("<{o\"i!a,<{i<a>", 10)]
    fn test_count_garbage(#[case] input: &str, #[case] expected: usize) {
        let (_, result) = score_stream(input).unwrap();
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case("{}", 1)]
    #[case("{{{}}}", 6)]
    #[case("{{},{}}", 5)]
    #[case("{{{},{},{{}}}}", 16)]
    #[case("{<a>,<a>,<a>,<a>}", 1)]
    #[case("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9)]
    #[case("{{<!!>},{<!!>},{<!!>},{<!!>}},", 9)]
    #[case("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)]
    fn test_score_stream(#[case] input: &str, #[case] expected: usize) {
        let (result, _) = score_stream(input).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }
}
