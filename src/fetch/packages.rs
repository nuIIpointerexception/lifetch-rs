use crate::util;
use std::fs::read_dir;
use std::path::Path;

pub struct PackageInfo<String> {
    pub pacman: String,
    pub pacman_version: String,
    pub cargo: String,
    pub cargo_version: String,
}

/// Get the total amount of packages installed.
///
///# Returns:
/// A `String` containing the total amount of packages installed.
///
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn packages(mut data: &mut PackageInfo<String>) {
    data.cargo = cargo().to_string();
    data.pacman = pacman().to_string();
}

/// Get the total amount of packages installed via pacman.
///
/// # Returns:
/// A `i32` number of the total amount of packages installed via pacman.
///
pub fn pacman() -> i32 {
    let mut packages = 0;
    let pacman_dir = Path::new("/var/lib/pacman/local");

    if let Ok(read_dir) = read_dir(pacman_dir) {
        packages = read_dir.count();
    }

    packages as i32
}

/// Get the total amount of packages installed via cargo.
///
/// # Returns:
/// A `i32` number of the total amount of packages installed via cargo.
///
pub fn cargo() -> i32 {
    let mut packages = 0;
    let cargo_dir = Path::new("/home")
        .join(util::data::get_env("HOME").unwrap())
        .join(".cargo");
    if cargo_dir.is_dir() {
        if let Ok(read_dir) = read_dir(cargo_dir) {
            packages = read_dir.count();
        };
    }
    packages as i32
}
