use crate::create_spaces;
use std::collections::HashMap;

/// # A couple of colors in case we need them later on.
///
/// ## ANSI colors should be supported by all terminals that support colors.
///
pub static BLACK: &str = "\x1b[30m";
pub static RED: &str = "\x1b[31m";
pub static GREEN: &str = "\x1b[32m";
pub static YELLOW: &str = "\x1b[33m";
pub static BLUE: &str = "\x1b[34m";
pub static MAGENTA: &str = "\x1b[35m";
pub static CYAN: &str = "\x1b[36m";
pub static WHITE: &str = "\x1b[37m";
pub static RESET: &str = "\x1b[0m";
pub static BOLD: &str = "\x1b[1m";
pub static ITALIC: &str = "\x1b[3m";
pub static UNDERLINE: &str = "\x1b[4m";
pub static BLINK: &str = "\x1b[5m";
pub static REVERSE: &str = "\x1b[7m";
pub static HIDDEN: &str = "\x1b[8m";
pub static CROSSED: &str = "\x1b[9m";
pub static GRAY: &str = "\x1b[90m";
pub static LIGHT_RED: &str = "\x1b[91m";
pub static LIGHT_GREEN: &str = "\x1b[92m";
pub static LIGHT_YELLOW: &str = "\x1b[93m";
pub static LIGHT_BLUE: &str = "\x1b[94m";
pub static LIGHT_MAGENTA: &str = "\x1b[95m";
pub static LIGHT_CYAN: &str = "\x1b[96m";
pub static LIGHT_GRAY: &str = "\x1b[97m";

/// # Windows handles new lines differently.
#[cfg(windows)]
pub static N: &str = "\r\n";
#[cfg(not(windows))]
pub static N: &str = "\n";

/// # Outputs a HashMap with ASCII color values.
///
/// # Returns:
/// The HashMap with the colors.
///
pub fn colormap() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();
    table.insert("BLACK", BLACK);
    table.insert("RED", RED);
    table.insert("GREEN", GREEN);
    table.insert("YELLOW", YELLOW);
    table.insert("BLUE", BLUE);
    table.insert("MAGENTA", MAGENTA);
    table.insert("CYAN", CYAN);
    table.insert("WHITE", WHITE);
    table.insert("GRAY", GRAY);
    table.insert("LIGHT_RED", LIGHT_RED);
    table.insert("LIGHT_GREEN", LIGHT_GREEN);
    table.insert("LIGHT_YELLOW", LIGHT_YELLOW);
    table.insert("LIGHT_BLUE", LIGHT_BLUE);
    table.insert("LIGHT_MAGENTA", LIGHT_MAGENTA);
    table.insert("LIGHT_CYAN", LIGHT_CYAN);
    table.insert("LIGHT_GRAY", LIGHT_GRAY);
    table.insert("RESET", RESET);
    table.insert("BOLD", BOLD);
    table.insert("UNDERLINE", UNDERLINE);
    table.insert("REVERSE", REVERSE);
    table.insert("BLINK", BLINK);
    table.insert("INVISIBLE", HIDDEN);
    table.insert("CROSSED", CROSSED);
    table.insert("ITALIC", ITALIC);
    table.insert("R", RESET);
    table.insert("B", BOLD);
    table.insert("U", UNDERLINE);
    table.insert("R", REVERSE);
    table.insert("BL", BLINK);
    table.insert("IN", HIDDEN);
    table.insert("C", CROSSED);
    table.insert("I", ITALIC);
    table
}

/// # Checks if the art is centered.
/// ## If it isn't centered, it will be centered by adding placeholders.
///
/// # Arguments:
/// * `art`: The art to check.
/// * `fetch_lines`: The number of lines the fetch text has.
/// * `art_lines`: The number of lines the art has.
///
/// # Returns:
/// The centered art.
///
pub fn ascii_check(center: bool, art: &mut String, fetch_lines: usize, art_lines: usize) {
    if center && fetch_lines > art_lines {
        let diff = (fetch_lines - art_lines) / 2;
        for _ in 0..diff {
            art.push_str(create_spaces(24).as_str());
            art.push('\n');
        }
    }
}

pub trait Ansi {
    fn strip_ansi_colors(&self) -> String;
}

impl Ansi for str {
    /// # Strips ANSI colors from a string.
    /// ## This is useful for getting the length of a string without the ANSI colors.
    ///
    /// # Arguments:
    /// * `self`: The string to strip the ANSI colors from.
    ///
    /// # Returns:
    /// The string without the ANSI colors.
    ///
    /// # Example:
    /// ```
    /// use ascii::Ansi;
    ///
    /// let string = "Hello, \x1b[31mworld\x1b[0m!";
    /// let stripped = string.strip_ansi_colors();
    ///
    /// assert_eq!(stripped, "Hello, world!");
    ///
    /// ```
    ///
    /// # Note:
    /// ## This function is really bad and should be replaced with a better one.
    /// ## Though this should work fine for now.
    fn strip_ansi_colors(&self) -> String {
        self // Formatting.
            .replace("\x1b[0m", "")
            .replace("\x1b[1m", "")
            .replace("\x1b[4m", "")
            .replace("\x1b[7m", "")
            .replace("\x1b[5m", "")
            .replace("\x1b[8m", "")
            .replace("\x1b[9m", "")
            .replace("\x1b[3m", "")
            // Colors.
            .replace("\x1b[30m", "")
            .replace("\x1b[31m", "")
            .replace("\x1b[32m", "")
            .replace("\x1b[33m", "")
            .replace("\x1b[34m", "")
            .replace("\x1b[35m", "")
            .replace("\x1b[36m", "")
            .replace("\x1b[37m", "")
            // Background colors.
            .replace("\x1b[90m", "")
            .replace("\x1b[91m", "")
            .replace("\x1b[92m", "")
            .replace("\x1b[93m", "")
            .replace("\x1b[94m", "")
            .replace("\x1b[95m", "")
            .replace("\x1b[96m", "")
            .replace("\x1b[97m", "")
    }
}
