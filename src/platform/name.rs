pub struct Name {
    pub name: String,
}

impl Name {
    /// This function generates the name of the operating system.
    #[allow(dead_code)]
    pub fn gen(&mut self) -> Self {
        Self { name: get() }
    }
}

#[cfg(target_os = "linux")]
fn get() -> String {
    String::from("Linux")
}

#[cfg(target_os = "windows")]
fn get() -> String {
    String::from("Windows")
}

#[cfg(target_os = "macos")]
fn get() -> String {
    String::from("MacOS")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let name = Name {
            name: String::from(""),
        }
        .gen();

        println!("Name: {}", name.name);

        assert_eq!(name.name, "Linux");
    }
}
