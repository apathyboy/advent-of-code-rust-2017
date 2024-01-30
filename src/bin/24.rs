use itertools::Itertools;
use pathfinding::prelude::dfs_reach;

advent_of_code::solution!(24);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Component {
    a: u32,
    b: u32,
    flipped: bool,
}

impl Component {
    fn new(a: u32, b: u32) -> Self {
        Self {
            a,
            b,
            flipped: false,
        }
    }
}

fn parse_component(line: &str) -> Option<Component> {
    let (a, b) = line.split_once('/')?;

    Some(Component::new(a.parse().ok()?, b.parse().ok()?))
}

fn parse(input: &str) -> Vec<Component> {
    input.lines().filter_map(parse_component).collect()
}

fn successors(
    component: &(Vec<Component>, Component),
    components: &[Component],
) -> Vec<(Vec<Component>, Component)> {
    let (path, component) = component;
    let pins = if component.flipped {
        component.a
    } else {
        component.b
    };

    let mut successors = Vec::new();

    for s in components.iter().filter(|c| c.a == pins || c.b == pins) {
        if !path.iter().any(|c| c.a == s.a && c.b == s.b) {
            let mut updated_path = path.clone();
            let mut s = *s;

            if s.b == pins {
                s.flipped = true;
            }

            updated_path.push(s);

            successors.push((updated_path, s));
        }
    }

    successors
}

fn find_valid_bridges(start: &Component, components: &[Component]) -> Vec<Vec<Component>> {
    let mut start = *start;
    start.flipped = start.b == 0;

    let start: (Vec<Component>, Component) = (Vec::from([start]), start);

    dfs_reach(start, |c| successors(c, components))
        .filter(|c| !c.0.is_empty() && (c.0[0].a == 0 || c.0[0].b == 0))
        .map(|c| c.0)
        .collect::<Vec<_>>()
}

fn calculate_bridge_strength(bridge: &[Component]) -> u32 {
    bridge.iter().map(|c| c.a + c.b).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let components = parse(input);

    let zero_pin_components: Vec<&Component> =
        components.iter().filter(|c| c.a == 0 || c.b == 0).collect();

    let mut max_bridge_strength = 0;

    for zero_pin_component in zero_pin_components {
        let bridges = find_valid_bridges(zero_pin_component, &components);

        let max_strength = bridges
            .iter()
            .map(|bridge| calculate_bridge_strength(bridge))
            .max()?;

        if max_strength > max_bridge_strength {
            max_bridge_strength = max_strength;
        }
    }

    Some(max_bridge_strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    let components = parse(input);

    let zero_pin_components: Vec<&Component> =
        components.iter().filter(|c| c.a == 0 || c.b == 0).collect();

    let mut max_bridge_length = 0;
    let mut max_bridge_strength = 0;

    for zero_pin_component in zero_pin_components {
        let bridges = find_valid_bridges(zero_pin_component, &components);
        let longest_bridges = bridges.iter().max_set_by(|a, b| a.len().cmp(&b.len()));

        if longest_bridges[0].len() > max_bridge_length {
            max_bridge_length = longest_bridges[0].len();

            let max_strength = longest_bridges
                .iter()
                .map(|bridge| calculate_bridge_strength(bridge))
                .max()?;

            if max_strength > max_bridge_strength {
                max_bridge_strength = max_strength;
            }
        }
    }

    Some(max_bridge_strength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
