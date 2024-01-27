advent_of_code::solution!(10);

fn skip_and_reverse_wraparound(mut vec: Vec<usize>, n: usize, i: usize) -> Vec<usize> {
    let len = vec.len();
    if len == 0 || i == 0 {
        return vec;
    }

    let start = n % len;
    let end = (start + i) % len;

    if start < end {
        vec[start..start + i].reverse();
    } else {
        // Wrap around case
        let mut temp = vec[start..].to_vec();
        temp.extend_from_slice(&vec[..end]);
        temp.reverse();
        vec[start..].copy_from_slice(&temp[..len - start]);
        vec[..end].copy_from_slice(&temp[len - start..]);
    }

    vec
}

fn twist(
    lengths: &[usize],
    mut list: Vec<usize>,
    current_pos: &mut usize,
    skip_length: &mut usize,
) -> Vec<usize> {
    for length in lengths {
        list = skip_and_reverse_wraparound(list, *current_pos, *length);

        *current_pos += *length + *skip_length;

        *skip_length += 1;
    }

    list
}

fn to_hex(input: &[usize]) -> String {
    input
        .iter()
        .fold(String::new(), |acc, &i| acc + &format!("{:02x}", i))
}

fn knot_hash(input: &str) -> Vec<usize> {
    let lengths: Vec<usize> = input
        .chars()
        .map(|c| c as usize)
        .chain(vec![17, 31, 73, 47, 23])
        .collect();

    let mut list = (0..256).collect::<Vec<_>>();
    let mut current_pos = 0;
    let mut skip_length = 0;

    for _ in 0..64 {
        list = twist(&lengths, list, &mut current_pos, &mut skip_length);
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .collect()
}

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
    Some(to_hex(&knot_hash(input.trim())))
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
        assert_eq!(to_hex(&result), String::from(expected));
    }
}
