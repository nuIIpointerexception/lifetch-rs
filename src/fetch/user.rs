use crate::data::UNKNOWN;
use crate::util;
use std::env;

pub struct UserInfo<String> {
    pub username: String,
    pub hostname: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn user(mut data: &mut UserInfo<String>) {
    data.username = env::var("USER").unwrap_or_else(|_| UNKNOWN.to_string());
    data.hostname = util::data::get_data("/etc/hostname", 4096)
        .unwrap()
        .replace('\n', "")
}
