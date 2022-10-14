mod default;

use std::collections::HashMap as Map;

use crate::error::{ErrorLevel, LightError};
use crate::util::hyperstr::N;
use crate::util::hyperstr::{GRAY, GREEN, LIGHT_RED, RED, RESET};
use image::imageops::FilterType;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fs;
use std::path::Path;

/// # The `Helio` struct simply contains a nested hashmap of the loaded configuration and the default symbols.
/// ## Example:
///```rust
///use configparser::helio::Helio;
///
///let mut config = Helio::new();
///```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Helio {
    map: Map<String, Map<String, Option<String>>>,
    default: String,
    comment: char,
    delimit: char,
    bool: HashMap<bool, &'static str>,
}

impl Helio {
    pub fn new() -> Helio {
        Helio {
            map: Map::new(),
            default: "default".to_owned(),
            comment: '#',
            delimit: '=',
            bool: HashMap::from([(true, "true"), (false, "false")]),
        }
    }

    pub fn load<T: AsRef<Path>>(
        &mut self,
        path: T,
    ) -> Result<Map<String, Map<String, Option<String>>>, LightError> {
        self.create(path.as_ref())?;
        /*if !path.as_ref().exists() { TODO: USE THIS FOR RELEASE!
            self.create(path.as_ref())?;
        }*/
        self.map = match fs::read_to_string(&path) {
            Err(why) => Err(LightError::new(
                format!(
                    "Could not read config at {GRAY}'{RED}{}{GRAY}'{RESET}: {GRAY}{}",
                    path.as_ref().display(),
                    why
                ),
                ErrorLevel::Error,
            ))?,
            Ok(s) => match self.parse(s) {
                Err(why) => Err(LightError::new(
                    format!(
                        "Could not parse config at {GRAY}'{RED}{}{GRAY}'{RESET}: {GRAY}{}",
                        path.as_ref().display(),
                        why
                    ),
                    ErrorLevel::Error,
                ))?,
                Ok(map) => map,
            },
        };

        Ok(self.map.clone())
    }

    #[allow(dead_code)]
    pub fn create<T: AsRef<Path>>(
        &mut self,
        path: T,
    ) -> Result<Map<String, Map<String, Option<String>>>, LightError> {
        let default = default::DEFAULT.to_owned();
        self.map = match fs::write(&path, default.clone()) {
            Err(why) => {
                return Err(LightError::new(
                    format!("couldn't create {}: {}", &path.as_ref().display(), why),
                    ErrorLevel::Config,
                ))
            }
            Ok(_) => match self.parse(default) {
                Err(why) => {
                    return Err(LightError::new(
                        format!("couldn't write {}: {}", &path.as_ref().display(), why),
                        ErrorLevel::Config,
                    ))
                }
                Ok(map) => map,
            },
        };
        Ok(self.map.clone())
    }

    fn parse(&self, input: String) -> Result<Map<String, Map<String, Option<String>>>, LightError> {
        let mut map: Map<String, Map<String, Option<String>>> = Map::new();
        let mut section = self.default.clone();
        let mut current_key: Option<String> = None;

        let out = |val: &str| val.to_owned();

        for (num, raw_line) in input.lines().enumerate() {
            let line = match raw_line.find(|c: char| self.comment == c) {
                Some(idx) => &raw_line[..idx],
                None => raw_line,
            };

            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            match (trimmed.find('['), trimmed.rfind(']')) {
                (Some(0), Some(end)) => {
                    section = out(trimmed[1..end].trim());

                    continue;
                }
                (Some(0), None) => {
                    return Err(LightError::new(
                        format!("Missing closing bracket on line {}", num + 1),
                        ErrorLevel::Config,
                    ))
                }
                _ => {}
            }

            if line.starts_with(char::is_whitespace) {
                let key = match current_key.as_ref() {
                    Some(x) => x,
                    _ => {
                        return Err(LightError::new(
                            format!("Missing key for value on line {}", num),
                            ErrorLevel::Error,
                        ))
                    }
                };

                let value_map = map.entry(section.clone()).or_insert_with(Map::new);

                let value = value_map
                    .entry(key.clone())
                    .or_insert_with(|| Some(String::new()));

                match value {
                    Some(x) => {
                        x.push_str(N);
                        x.push_str(trimmed);
                    }
                    None => {
                        *value = Some(format!("{}{}", N, trimmed));
                    }
                }

                continue;
            }

            let value_map = map.entry(section.clone()).or_insert_with(Map::new);

            match trimmed.find(&self.delimit.to_string()) {
                Some(delimiter) => {
                    let key = out(trimmed[..delimiter].trim());

                    if key.is_empty() {
                        return Err(LightError::new(
                            format!("Missing key for value on line: {LIGHT_RED}{}", num),
                            ErrorLevel::Config,
                        ));
                    }

                    current_key = Some(key.clone());
                    let value = trimmed[delimiter + 1..].trim().to_owned();
                    value_map.insert(key, Some(value));
                }
                None => {
                    let key = out(trimmed);
                    current_key = Some(key.clone());

                    value_map.insert(key, None);
                }
            }
        }

        Ok(map)
    }

    pub fn get_str(&self, section: &str, key: &str) -> Result<String, LightError> {
        match self.map.get(section) {
            Some(x) => match x.get(key) {
                Some(y) => match y {
                    Some(z) => Ok(z.clone().replace('"', "")),
                    None => Err(LightError::new(
                        format!(
                            "Section {GRAY}'{GREEN}[{}]{GRAY}'{RESET}: Key {LIGHT_RED}'{}'{RESET} has no value",
                            section, key
                        ),
                        ErrorLevel::Config,
                    )),
                },
                None =>
                    Err(LightError::new(format!(
                        "Section {GRAY}'{GREEN}[{}]{GRAY}'{RESET}: Key {LIGHT_RED}'{}'{RESET} not found",
                        section, key
                    ),  ErrorLevel::Config)),
            },
            None =>
                Err(LightError::new(format!(
                    "Section {GRAY}'{GREEN}[{}]{GRAY}'{RESET} not found",
                    section
                ),  ErrorLevel::Config)),
        }
    }

    pub fn get_bool(&self, section: &str, key: &str) -> Result<bool, LightError> {
        let value = self.get_str(section, key).unwrap();

        match value.parse::<bool>() {
            Ok(x) => Ok(x),
            _ => {
               Err(LightError::new(
                    format!(
                        "Section {GRAY}'{GREEN}{}{GRAY}'{RESET}: Key {GRAY}'{LIGHT_RED}{}{GRAY}'{RESET} is not a boolean",
                        section, key
                    ),
                    ErrorLevel::Config,
                ))

            }
        }
    }

    pub fn get_int(&self, section: &str, key: &str) -> Result<u32, LightError> {
        let value = self.get_str(section, key).unwrap();

        match value.parse::<u32>() {
            Ok(x) => Ok(x),
            _ => {
                Err(LightError::new(
                    format!(
                        "Section {GRAY}'{GREEN}{}{GRAY}'{RESET}: Key {GRAY}'{LIGHT_RED}{}{GRAY}'{RESET} is not an integer",
                        section, key
                    ),
                    ErrorLevel::Config,
                ))
            }
        }
    }

    pub fn get_filter(&self, section: &str, key: &str) -> Result<FilterType, LightError> {
        let filter = self.get_str(section, key);

        match filter.as_ref().unwrap().replace('"', "").as_str() {
            "Nearest" => Ok(FilterType::Nearest),
            "Gaussian" => Ok(FilterType::Gaussian),
            "Triangle" => Ok(FilterType::Triangle),
            "Catmull" => Ok(FilterType::CatmullRom),
            "Lanczos" => Ok(FilterType::Lanczos3),
            _ => Err(LightError::new(
                format!(
                    "Filter type {LIGHT_RED}'{}'{RESET} invalid {GRAY}| Available: 'Nearest', 'Gaussian', 'Triangle', 'Catmull', 'Lanczos'{RESET}",
                    filter.unwrap()
                ),
                ErrorLevel::Config,
            )),
        }
    }

    pub fn set(
        &mut self,
        section: &str,
        key: &str,
        value: Option<String>,
    ) -> Option<Option<String>> {
        match self.map.get_mut(&section.to_owned()) {
            Some(secondary) => secondary.insert(key.to_owned(), value),
            None => {
                let mut value_map: Map<String, Option<String>> = Map::new();
                value_map.insert(key.to_owned(), value);
                self.map.insert(section.to_owned(), value_map);
                None
            }
        }
    }

    pub fn set_str(
        &mut self,
        section: &str,
        key: &str,
        value: Option<&str>,
    ) -> Option<Option<String>> {
        self.set(section, key, value.map(String::from))
    }
}
