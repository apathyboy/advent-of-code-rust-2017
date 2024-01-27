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

fn cross_firewall(
    layers: &[FirewallLayer],
    max_layer_depth: usize,
    start_time: usize,
) -> (usize, bool) {
    let mut severity = 0;
    let mut detected = false;

    for i in start_time..=max_layer_depth + start_time {
        let layer = layers.iter().find(|layer| layer.depth == i - start_time);

        if let Some(layer) = layer {
            let sweeps = i / (layer.range - 1);
            let mut scanner_pos = i.rem_euclid(layer.range - 1);

            if sweeps.rem_euclid(2) == 1 {
                scanner_pos = layer.range - scanner_pos - 1;
            }

            if scanner_pos == 0 {
                severity += layer.depth * layer.range;
                detected = true;
            }
        }
    }

    (severity, detected)
}

pub fn part_one(input: &str) -> Option<usize> {
    let layers = parse(input);

    let max_layer_depth = layers
        .iter()
        .map(|layer| layer.depth)
        .max_by(|&a, &b| a.cmp(&b))?;

    let (severity, _) = cross_firewall(&layers, max_layer_depth, 0);

    Some(severity)
}

pub fn part_two(input: &str) -> Option<usize> {
    let layers = parse(input);

    let max_layer_depth = layers
        .iter()
        .map(|layer| layer.depth)
        .max_by(|&a, &b| a.cmp(&b))?;

    for i in 0.. {
        let (_, detected) = cross_firewall(&layers, max_layer_depth, i);

        if !detected {
            return Some(i);
        }
    }

    None
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
