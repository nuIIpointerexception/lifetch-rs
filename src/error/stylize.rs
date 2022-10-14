use crate::util::hyperstr::RESET;
use crate::Ansi;

/// # A messy way to apply a border to a string.
///
/// # Examples
///
/// ```
/// use helio::error::stylize::border;
///
/// let s = "test";
/// let b = border(s, 1);
/// println!("{}", b);
/// ```
///
pub fn apply_border(text: &str, color: &str) -> String {
    let lines = text.split('\n').count();
    if lines == 1 {
        return single(text, color);
    }
    multi(text, color)
}

fn single(text: &str, color: &str) -> String {
    let top_left_corner = "╭";
    let top_right_corner = "╮";
    let bottom_left_corner = "╰";
    let bottom_right_corner = "╯";
    let vertical_edge_left = "│ ";
    let vertical_edge_right = " │";
    let horizontal_edge = "─";
    let mut output = String::new();
    let len = text.strip_ansi_colors().len();
    output.push_str(color);
    output.push_str(top_left_corner);
    for _ in 0..len + 2 {
        output.push_str(horizontal_edge);
    }
    output.push_str(top_right_corner);
    output.push('\n');
    output.push_str(vertical_edge_left);
    output.push_str(RESET);
    output.push_str(text);
    output.push_str(color);
    output.push_str(vertical_edge_right);
    output.push('\n');
    output.push_str(bottom_left_corner);
    for _ in 0..len + 2 {
        output.push_str(horizontal_edge);
    }
    output.push_str(bottom_right_corner);
    output
}

fn multi(text: &str, color: &str) -> String {
    let top_left_corner = "╭";
    let top_right_corner = "╮";
    let bottom_left_corner = "╰";
    let bottom_right_corner = "╯";
    let vertical_edge_left = "│ ";
    let vertical_edge_right = " │";
    let horizontal_edge = "─";
    let mut output = String::new();
    let mut len = 0;
    for line in text.split('\n') {
        if line.strip_ansi_colors().len() > len {
            len = line.strip_ansi_colors().len();
        }
    }
    output.push_str(color);
    output.push_str(top_left_corner);
    for _ in 0..len + 2 {
        output.push_str(horizontal_edge);
    }
    output.push_str(top_right_corner);
    output.push('\n');
    for line in text.split('\n') {
        output.push_str(vertical_edge_left);
        output.push_str(RESET);
        output.push_str(line);
        for _ in 0..len - line.strip_ansi_colors().len() {
            output.push(' ');
        }
        output.push_str(color);
        output.push_str(vertical_edge_right);
        output.push('\n');
    }
    output.push_str(bottom_left_corner);
    for _ in 0..len + 2 {
        output.push_str(horizontal_edge);
    }
    output.push_str(bottom_right_corner);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::hyperstr::RED;

    #[test]
    fn test_border() {
        let text = "Hello, world!";
        let output = apply_border(text, RED);
        println!("{}", output);
        // Really, why is it linting it like this? 😠
        assert_eq!(
            output.strip_ansi_colors(),
            "╭───────────────╮
│ Hello, world! │
╰───────────────╯"
        );
    }

    #[test]
    fn test_border_multi() {
        // Even worse linting here. 🥶
        let text = "Hello, world!

This is a test.";
        let output = apply_border(text, RED);
        println!("{}", output);
        assert_eq!(
            output.strip_ansi_colors(),
            "╭─────────────────╮
│ Hello, world!   │
│                 │
│ This is a test. │
╰─────────────────╯"
        );
    }
}
