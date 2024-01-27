advent_of_code::solution!(13);

struct FirewallLayer {
    depth: usize,
    range: usize,
}

impl FirewallLayer {
    fn new(depth: usize, range: usize) -> Self {
        Self { depth, range }
    }
}

fn parse_layer(line: &str) -> Option<FirewallLayer> {
    let (depth, range) = line.split_once(": ")?;

    Some(FirewallLayer::new(depth.parse().ok()?, range.parse().ok()?))
}

fn parse(input: &str) -> Vec<FirewallLayer> {
    input.lines().filter_map(parse_layer).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let layers = parse(input);

    let severity = layers
        .iter()
        .filter(|&layer| layer.depth.rem_euclid(2 * (layer.range - 1)) == 0)
        .map(|layer| layer.depth * layer.range)
        .sum();

    Some(severity)
}

pub fn part_two(input: &str) -> Option<usize> {
    let layers = parse(input);

    (0..)
        .filter(|delay| {
            !layers
                .iter()
                .any(|layer| (delay + layer.depth).rem_euclid(2 * (layer.range - 1)) == 0)
        })
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
