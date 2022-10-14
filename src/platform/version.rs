use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Version {
    pub version: String,
}

impl Version {
    /// Get the version of the OS.
    #[allow(dead_code)]
    pub fn gen(&mut self) -> Self {
        Self { version: get() }
    }
}

#[cfg(target_os = "linux")]
fn get() -> String {
    let file = File::open("/proc/version").unwrap();
    let reader = BufReader::new(file);
    let mut line = String::new();

    for l in reader.lines() {
        line = l.unwrap();
    }

    line.split_whitespace().nth(2).unwrap().to_string()
}

/// TODO: Implement for other OSes
#[cfg(target_os = "windows")]
fn get() -> String {
    String::from("Unknown")
}

#[cfg(target_os = "macos")]
fn get() -> String {
    String::from("Unknown")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[cfg(target_os = "linux")]
    /// We use this command to get the version of the OS on the system.
    fn generate_version() -> String {
        let cmd = Command::new("uname")
            .arg("-r")
            .output()
            .expect("Failed to execute command");

        String::from_utf8(cmd.stdout).unwrap().trim().to_string()
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_version() {
        let version = Version {
            version: String::from(""),
        }
        .gen();

        println!("Version: {}", version.version);

        assert_eq!(version.version, generate_version());
    }
}
