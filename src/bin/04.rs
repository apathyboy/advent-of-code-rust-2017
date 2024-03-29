use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(4);

fn is_passphrase_v2_valid(passphrase: &&str) -> bool {
    let words = passphrase
        .split_ascii_whitespace()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();

    words.par_iter().all(|word| {
        let anagrams = word
            .chars()
            .permutations(word.len())
            .map(|p| p.iter().collect::<String>())
            .collect::<Vec<_>>();

        words.iter().filter(|&word| anagrams.contains(word)).count() == 1
    })
}

fn is_passphrase_valid(passphrase: &&str) -> bool {
    let words = passphrase.split_ascii_whitespace().collect::<Vec<_>>();

    words
        .iter()
        .all(|word| words.iter().filter(|&w| *w == *word).count() == 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().filter(is_passphrase_valid).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().filter(is_passphrase_v2_valid).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcde fghij", true)]
    #[case("abcde xyz ecdab", false)]
    #[case("a ab abc abd abf abj", true)]
    #[case("iiii oiii ooii oooi oooo", true)]
    #[case("oiii ioii iioi iiio", false)]
    fn test_is_passphrase_v2_valid(#[case] input: &str, #[case] expected: bool) {
        let result = is_passphrase_v2_valid(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("aa bb cc dd ee", true)]
    #[case("aa bb cc dd aa", false)]
    #[case("aa bb cc dd aaa", true)]
    fn test_is_passphrase_valid(#[case] input: &str, #[case] expected: bool) {
        let result = is_passphrase_valid(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
