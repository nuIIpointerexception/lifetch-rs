#[allow(unused)]
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn gpu() -> String {
    String::new()
}
