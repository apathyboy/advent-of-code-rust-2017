pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_line_as_digits(line: &str) -> Vec<u32> {
    line.trim().chars().filter_map(|c| c.to_digit(10)).collect()
}

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

pub fn twist(
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

pub fn to_hex_string(input: &[usize]) -> String {
    input
        .iter()
        .fold(String::new(), |acc, &i| acc + &format!("{:02x}", i))
}

pub fn to_binary_string(input: &[usize]) -> String {
    input
        .iter()
        .fold(String::new(), |acc, &i| acc + &format!("{:04b}", i))
}

pub fn knot_hash(input: &str) -> Vec<usize> {
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
