use crate::data::UNKNOWN;
use std::env;

pub struct ShellInfo<String> {
    pub shell: String,
    pub shell_version: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn shell(mut data: &mut ShellInfo<String>) {
    data.shell = env::var("SHELL").unwrap_or_else(|_| UNKNOWN.to_string());
}
