//! # This should work for now. Not the fanciest way to do it, but it works.
use crate::error::stylize::apply_border;
use crate::util::hyperstr::{LIGHT_RED, RED, RESET};
use std::process::exit;

pub mod stylize;

fn warn(text: &str) -> std::fmt::Result {
    let new = String::from(LIGHT_RED) + "WARNING: " + (RESET) + text;
    eprintln!("{}", apply_border(&new, LIGHT_RED));
    Ok(())
}

fn err(s: &str, c: &str, text: &str) -> std::fmt::Result {
    let new = String::from(c) + s + (c) + text;
    eprintln!("{}", apply_border(&new, c));
    exit(1);
}

pub enum ErrorLevel {
    Config,
    Warning,
    Error,
}

pub struct LightError {
    pub message: String,
    pub level: ErrorLevel,
}

impl LightError {
    pub fn new(message: String, level: ErrorLevel) -> Self {
        Self { message, level }
    }
}

impl std::fmt::Display for LightError {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.level {
            ErrorLevel::Config => err("CONFIG: ", LIGHT_RED, &self.message),
            ErrorLevel::Warning => warn(&self.message),
            ErrorLevel::Error => err("ERROR: ", RED, &self.message),
        }
    }
}

impl std::fmt::Debug for LightError {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.level {
            ErrorLevel::Config => err("CONFIG: ", LIGHT_RED, &self.message),
            ErrorLevel::Warning => warn(&self.message),
            ErrorLevel::Error => err("ERROR: ", RED, &self.message),
        }
    }
}

impl std::error::Error for LightError {}

#[cfg(test)]
mod tests {
    use crate::error::stylize::apply_border;

    use crate::util::hyperstr::LIGHT_RED;
    use crate::Ansi;

    fn soft_err(s: &str, c: &str, text: &str) -> String {
        let new = String::from(c) + s + (c) + text;
        apply_border(&new, c)
    }

    #[test]
    fn test_error() {
        let err = soft_err("ERROR: ", LIGHT_RED, "This is a test error.");

        println!("{}", err);

        assert_eq!(
            err.strip_ansi_colors(),
            "╭──────────────────────────────╮
│ ERROR: This is a test error. │
╰──────────────────────────────╯"
        );
    }
}
