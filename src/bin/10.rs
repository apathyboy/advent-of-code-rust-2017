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

fn hash(list_size: usize, lengths: &[usize]) -> usize {
    let mut list = (0..list_size).collect::<Vec<_>>();
    let mut current_pos = 0;

    for (skip_size, length) in lengths.iter().enumerate() {
        list = skip_and_reverse_wraparound(list, current_pos, *length);

        current_pos += *length + skip_size;
    }

    list[0] * list[1]
}

fn sparse_hash(list_size: usize, lengths: &[usize]) -> Vec<usize> {
    let mut list = (0..list_size).collect::<Vec<_>>();
    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in lengths {
            list = skip_and_reverse_wraparound(list, current_pos, *length);

            current_pos += *length + skip_size;

            skip_size += 1;
        }
    }

    list
}

fn dense_hash(sparse: &[usize]) -> Vec<usize> {
    sparse
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .collect()
}

fn to_hex(input: &[usize]) -> String {
    input
        .iter()
        .fold(String::new(), |acc, &i| acc + &format!("{:02x}", i))
}

fn knot_hash(input: &str) -> String {
    let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect();
    let mut append = vec![17, 31, 73, 47, 23];

    lengths.append(&mut append);

    let sparse_result = sparse_hash(256, &lengths);
    let dense_result = dense_hash(&sparse_result);

    to_hex(&dense_result)
}

pub fn part_one(input: &str) -> Option<usize> {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    Some(hash(256, &lengths))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(knot_hash(input.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_hash() {
        let lengths = vec![3, 4, 1, 5];

        let result = hash(5, &lengths);
        assert_eq!(result, 12);
    }

    #[rstest]
    #[case("", "a2582a3a0e66e6e86e3812dcb672a272")]
    #[case("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd")]
    #[case("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d")]
    #[case("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e")]
    fn test_knot_hash(#[case] input: &str, #[case] expected: &str) {
        let result = knot_hash(input);
        assert_eq!(result, String::from(expected));
    }
}
