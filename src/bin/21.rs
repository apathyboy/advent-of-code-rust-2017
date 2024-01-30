advent_of_code::solution!(21);

struct Rule {
    from: Vec<String>,
    to: Vec<String>,
}

impl Rule {
    fn new(from: Vec<String>, to: Vec<String>) -> Self {
        Self { from, to }
    }
}

const START: [&str; 3] = [".#.", "..#", "###"];

fn flip(square: &[String]) -> Vec<String> {
    square
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
}

fn rotate(square: &[String]) -> Vec<String> {
    let size = square.len();
    let mut rotated = Vec::new();

    for col in 0..size {
        let mut new_row = String::new();
        for row in (0..size).rev() {
            new_row.push(square[row].chars().nth(col).unwrap());
        }
        rotated.push(new_row);
    }

    rotated
}

fn match_rule(square: &[String], rules: &[Rule]) -> Option<Vec<String>> {
    let mut square = square.to_vec();

    for _ in 0..4 {
        let rule = rules.iter().find(|r| r.from == square);

        if rule.is_some() {
            return Some(rule?.to.clone());
        }

        square = rotate(&square);
    }

    square = flip(&square);

    for _ in 0..4 {
        let rule = rules.iter().find(|r| r.from == square);

        if rule.is_some() {
            return Some(rule?.to.clone());
        }

        square = rotate(&square);
    }

    panic!("No rules for this piece");
}

fn reassemble_chunks(
    chunks: &[Vec<Vec<String>>],
    width: usize,
    height: usize,
    chunk_size: usize,
) -> Vec<String> {
    let mut grid = vec![String::with_capacity(width); height];

    for (chunk_row, chunk_cols) in chunks.iter().enumerate() {
        for chunk in chunk_cols.iter() {
            for (row_within_chunk, line) in chunk.iter().enumerate() {
                let row = chunk_row * chunk_size + row_within_chunk;
                grid[row].push_str(line);
            }
        }
    }

    grid
}

fn assemble(chunks: &[Vec<Vec<String>>]) -> Vec<String> {
    reassemble_chunks(
        chunks,
        chunks.len() * chunks[0][0][0].len(),
        chunks.len() * chunks[0][0][0].len(),
        chunks[0][0][0].len(),
    )
}

fn split_2x2(image: &[String]) -> Vec<Vec<Vec<String>>> {
    let height = image.len();
    let width = image.first().map_or(0, |s| s.len());

    let chunks = (0..height)
        .step_by(2)
        .map(|y| {
            (0..width)
                .step_by(2)
                .map(move |x| {
                    image
                        .iter()
                        .skip(y)
                        .take(2)
                        .map(|row| row.chars().skip(x).take(2).collect())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>();

    chunks
}

fn split_3x3(image: &[String]) -> Vec<Vec<Vec<String>>> {
    let height = image.len();
    let width = image.first().map_or(0, |s| s.len());

    let chunks = (0..height)
        .step_by(3)
        .map(|y| {
            (0..width)
                .step_by(3)
                .map(move |x| {
                    image
                        .iter()
                        .skip(y)
                        .take(3)
                        .map(|row| row.chars().skip(x).take(3).collect())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>();

    chunks
}

fn split(image: &[String]) -> Vec<Vec<Vec<String>>> {
    if image.len().rem_euclid(2) == 0 {
        split_2x2(image)
    } else if image.len().rem_euclid(3) == 0 {
        split_3x3(image)
    } else {
        panic!("Can't split image");
    }
}

fn enhance(image: &[String], rules: &[Rule]) -> Vec<String> {
    let mut chunks = split(image);

    for chunk in chunks.iter_mut().flatten() {
        *chunk = match_rule(chunk, rules).unwrap();
    }

    assemble(&chunks)
}

fn parse_rule(line: &str) -> Option<Rule> {
    let (from, to) = line.split_once(" => ")?;

    let from = from.split('/').map(|s| s.to_string()).collect::<Vec<_>>();
    let to = to.split('/').map(|s| s.to_string()).collect::<Vec<_>>();

    Some(Rule::new(from, to))
}

fn parse_rules(input: &str) -> Vec<Rule> {
    input.lines().filter_map(parse_rule).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let rules = parse_rules(input);

    let mut image = START.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    for _ in 0..5 {
        image = enhance(&image, &rules);
    }

    let pixels_on = image
        .iter()
        .map(|row| row.chars().filter(|c| *c == '#').count())
        .sum();

    Some(pixels_on)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rules = parse_rules(input);

    let mut image = START.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    for _ in 0..18 {
        image = enhance(&image, &rules);
    }

    let pixels_on = image
        .iter()
        .map(|row| row.chars().filter(|c| *c == '#').count())
        .sum();

    Some(pixels_on)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let image = vec!["#..".to_string(), "#.#".to_string(), "##.".to_string()];
        let expected = vec!["###".to_string(), "#..".to_string(), ".#.".to_string()];

        let result: Vec<String> = rotate(&image);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_2x2() {
        let image = vec![
            "#..#".to_string(),
            "....".to_string(),
            "....".to_string(),
            "#..#".to_string(),
        ];

        let result = split_2x2(&image);

        assert_eq!(result[0][0][0], String::from("#."));
        assert_eq!(result[0][0][1], String::from(".."));

        assert_eq!(result[0][1][0], String::from(".#"));
        assert_eq!(result[0][1][1], String::from(".."));

        assert_eq!(result[1][0][0], String::from(".."));
        assert_eq!(result[1][0][1], String::from("#."));

        assert_eq!(result[1][1][0], String::from(".."));
        assert_eq!(result[1][1][1], String::from(".#"));
    }

    #[test]
    fn test_split_3x3() {
        let image = vec![
            "##.##.".to_string(),
            "#..#..".to_string(),
            "......".to_string(),
            "##.##.".to_string(),
            "#..#..".to_string(),
            "......".to_string(),
        ];

        let result = split_3x3(&image);

        assert_eq!(result[0][0][0], String::from("##."));
        assert_eq!(result[0][0][1], String::from("#.."));
        assert_eq!(result[0][0][2], String::from("..."));

        assert_eq!(result[0][1][0], String::from("##."));
        assert_eq!(result[0][1][1], String::from("#.."));
        assert_eq!(result[0][1][2], String::from("..."));

        assert_eq!(result[1][0][0], String::from("##."));
        assert_eq!(result[1][0][1], String::from("#.."));
        assert_eq!(result[1][0][2], String::from("..."));

        assert_eq!(result[1][1][0], String::from("##."));
        assert_eq!(result[1][1][1], String::from("#.."));
        assert_eq!(result[1][1][2], String::from("..."));
    }

    #[test]
    fn test_enhance() {
        let rules = parse_rules(&advent_of_code::template::read_file("examples", DAY));
        let mut image = START.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        image = enhance(&image, &rules);

        let result = image
            .iter()
            .map(|row| row.chars().filter(|c| *c == '#').count())
            .sum::<usize>();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_enhance_2x() {
        let rules = parse_rules(&advent_of_code::template::read_file("examples", DAY));
        let mut image = START.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        image = enhance(&image, &rules);
        image = enhance(&image, &rules);

        let result = image
            .iter()
            .map(|row| row.chars().filter(|c| *c == '#').count())
            .sum::<usize>();

        assert_eq!(result, 12);
    }
}
