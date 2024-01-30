use std::collections::HashMap;

advent_of_code::solution!(25);

struct State {
    zero_rule: (u8, i64, char),
    one_rule: (u8, i64, char),
}

fn parse(input: &str) -> (usize, HashMap<char, State>) {
    let steps = input.lines().nth(1).unwrap();
    let steps = steps[36..steps.len() - 7].parse::<usize>().unwrap();

    let mut states = HashMap::new();

    for state in input.lines().skip(2).collect::<Vec<_>>().chunks(10) {
        let state_label = state[1][9..].chars().next().unwrap();

        let zero_write = state[3][22..].chars().next().unwrap().to_digit(10).unwrap() as u8;
        let zero_move = if state[4].contains("right") { 1 } else { -1 };
        let zero_next_state = state[5][26..].chars().next().unwrap();

        let one_write = state[7][22..].chars().next().unwrap().to_digit(10).unwrap() as u8;
        let one_move = if state[8].contains("right") { 1 } else { -1 };
        let one_next_state = state[9][26..].chars().next().unwrap();

        states.insert(
            state_label,
            State {
                zero_rule: (zero_write, zero_move, zero_next_state),
                one_rule: (one_write, one_move, one_next_state),
            },
        );
    }

    (steps, states)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (steps, states) = parse(input);
    let mut cur_state = 'A';
    let mut cursor = 0;
    let mut tape: HashMap<i64, u8> = HashMap::new();

    for _ in 0..steps {
        let cur_val = tape.entry(cursor).or_insert(0);
        let state = states.get(&cur_state)?;

        if *cur_val == 0 {
            *cur_val = state.zero_rule.0;
            cursor += state.zero_rule.1;
            cur_state = state.zero_rule.2;
        } else {
            *cur_val = state.one_rule.0;
            cursor += state.one_rule.1;
            cur_state = state.one_rule.2;
        }
    }

    Some(tape.iter().filter(|(_, &val)| val == 1).count())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
