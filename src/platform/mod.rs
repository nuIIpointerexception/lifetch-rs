mod arch;
mod name;
mod version;

pub struct Platform {
    pub name: String,
    pub version: String,
    pub arch: String,
}

#[cfg(target_os = "linux")]
impl Platform {
    pub fn gen(&mut self) -> Self {
        let name = name::Name {
            name: String::from(""),
        }
        .gen()
        .name;
        let version = version::Version {
            version: String::from(""),
        }
        .gen()
        .version;
        let arch = arch::Arch {
            arch: String::from(""),
        }
        .gen()
        .arch;

        Self {
            name,
            version,
            arch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_platform() {
        let version_command = Command::new("uname")
            .arg("-r")
            .output()
            .expect("Failed to execute command");

        let version = String::from_utf8(version_command.stdout)
            .unwrap()
            .trim()
            .to_string();

        let arch_command = Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to execute command");

        let arch = String::from_utf8(arch_command.stdout)
            .unwrap()
            .trim()
            .to_string();

        let name_command = Command::new("uname")
            .arg("-s")
            .output()
            .expect("Failed to execute command");

        let name = String::from_utf8(name_command.stdout)
            .unwrap()
            .trim()
            .to_string();

        let platform = Platform {
            name: String::from(""),
            version: String::from(""),
            arch: String::from(""),
        }
        .gen();

        println!("Platform: {}", platform.name);
        println!("Version: {}", platform.version);
        println!("Arch: {}", platform.arch);

        assert_eq!(platform.name, name);
        assert_eq!(platform.version, version);
        assert_eq!(platform.arch, arch);
    }
}
