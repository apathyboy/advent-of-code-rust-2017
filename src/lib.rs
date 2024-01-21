pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_line_as_digits(line: &str) -> Vec<u32> {
    line.trim().chars().filter_map(|c| c.to_digit(10)).collect()
}
