use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(22);

#[derive(Debug, PartialEq, Clone, Copy)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug)]
struct VirusCarrier {
    node: IVec2,
    direction: IVec2,
}

impl VirusCarrier {
    fn new() -> Self {
        Self {
            node: IVec2::new(0, 0),
            direction: IVec2::new(0, -1),
        }
    }
}

fn parse_map(input: &str) -> Option<HashMap<IVec2, NodeState>> {
    let side = input.lines().count() as i32;
    let half_side = side / 2;

    Some(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    if c == '#' {
                        (
                            IVec2::new(-half_side + x as i32, -half_side + y as i32),
                            NodeState::Infected,
                        )
                    } else {
                        (
                            IVec2::new(-half_side + x as i32, -half_side + y as i32),
                            NodeState::Clean,
                        )
                    }
                })
            })
            .collect(),
    )
}

fn turn_left(dir: &IVec2) -> IVec2 {
    match (dir.x, dir.y) {
        (0, -1) => IVec2::new(-1, 0), // Up to left
        (-1, 0) => IVec2::new(0, 1),  // Left to down
        (0, 1) => IVec2::new(1, 0),   // Down to right
        (1, 0) => IVec2::new(0, -1),  // Right to up
        _ => panic!("Invalid direction"),
    }
}

fn turn_right(dir: &IVec2) -> IVec2 {
    match (dir.x, dir.y) {
        (0, -1) => IVec2::new(1, 0),  // Up to left
        (-1, 0) => IVec2::new(0, -1), // Left to down
        (0, 1) => IVec2::new(-1, 0),  // Down to right
        (1, 0) => IVec2::new(0, 1),   // Right to up
        _ => panic!("Invalid direction"),
    }
}

fn burst(virus_carrier: &mut VirusCarrier, map: &mut HashMap<IVec2, NodeState>) -> bool {
    let node_state = *map.entry(virus_carrier.node).or_insert(NodeState::Clean);

    virus_carrier.direction = match node_state {
        NodeState::Infected => turn_right(&virus_carrier.direction),
        NodeState::Clean => turn_left(&virus_carrier.direction),
        _ => panic!("Invalid node state"),
    };

    let new_state = match node_state {
        NodeState::Infected => NodeState::Clean,
        NodeState::Clean => NodeState::Infected,
        _ => panic!("Invalid node state"),
    };

    *map.get_mut(&virus_carrier.node).unwrap() = new_state;

    virus_carrier.node += virus_carrier.direction;

    new_state == NodeState::Infected
}

fn burst2(virus_carrier: &mut VirusCarrier, map: &mut HashMap<IVec2, NodeState>) -> bool {
    let node_state = *map.entry(virus_carrier.node).or_insert(NodeState::Clean);

    virus_carrier.direction = match node_state {
        NodeState::Clean => turn_left(&virus_carrier.direction),
        NodeState::Weakened => virus_carrier.direction,
        NodeState::Infected => turn_right(&virus_carrier.direction),
        NodeState::Flagged => virus_carrier.direction * -1,
    };

    let new_state = match node_state {
        NodeState::Clean => NodeState::Weakened,
        NodeState::Weakened => NodeState::Infected,
        NodeState::Infected => NodeState::Flagged,
        NodeState::Flagged => NodeState::Clean,
    };

    *map.get_mut(&virus_carrier.node).unwrap() = new_state;

    virus_carrier.node += virus_carrier.direction;

    new_state == NodeState::Infected
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input)?;
    let mut virus_carrier = VirusCarrier::new();

    let mut infections = 0;

    for _ in 0..10000 {
        if burst(&mut virus_carrier, &mut map) {
            infections += 1;
        }
    }

    Some(infections)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_map(input)?;
    let mut virus_carrier = VirusCarrier::new();

    let mut infections = 0;

    for _ in 0..10000000 {
        if burst2(&mut virus_carrier, &mut map) {
            infections += 1;
        }
    }

    Some(infections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5587));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2511944));
    }
}
