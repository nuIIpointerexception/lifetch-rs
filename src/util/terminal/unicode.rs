/// Check for the current Terminal's Unicode support.
///
/// This works for Windows, Linux, Mac, and other Unix systems.
///
/// # Returns:
/// True if Unicode is supported, false otherwise.
///
/// # Example:
/// ```
/// use unicode::supports_unicode;
/// let unicode = supports_unicode();
/// assert!(unicode);
/// ```

#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn supports_unicode() -> bool {
    true
}

#[cfg(all(unix, not(target_os = "macos")))]
#[allow(dead_code)]
pub fn supports_unicode() -> bool {
    let lang = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "".into())
        .to_lowercase();
    lang.contains("utf-8") || lang.contains("utf8")
}
#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn supports_unicode() -> bool {
    std::env::var("CI").is_ok()
            || std::env::var("WT_SESSION").is_ok() // Windows Terminal
            || std::env::var("ConEmuTask") == Ok("{cmd:Cmder}".into()) // ConEmu and cmder
            || std::env::var("TERM_PROGRAM") == Ok("vscode".into()) // VSCode
            || std::env::var("TERM") == Ok("xterm-256color".into()) // XTerm
            || std::env::var("TERM") == Ok("alacritty".into()) // Alacritty
}

#[cfg(all(not(unix), not(windows)))]
#[allow(dead_code)]
pub fn supports_unicode() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_support() {
        println!("Unicode support: {}", supports_unicode());
    }
}
