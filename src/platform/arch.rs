pub struct Arch {
    pub arch: String,
}

impl Arch {
    /// This function generates the architecture of the operating system.
    #[allow(dead_code)]
    pub fn gen(&mut self) -> Self {
        Self { arch: get() }
    }
}

#[cfg(target_arch = "x86_64")]
fn get() -> String {
    String::from("x86_64")
}

#[cfg(target_arch = "x86")]
fn get_arch() -> String {
    String::from("x86")
}

#[cfg(target_arch = "arm")]
fn get_arch() -> String {
    String::from("arm")
}

#[cfg(target_arch = "aarch64")]
fn get_arch() -> String {
    String::from("aarch64")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch() {
        let arch = Arch {
            arch: String::from(""),
        }
        .gen();

        println!("Arch: {}", arch.arch);

        assert_eq!(arch.arch, "x86_64");
    }
}
