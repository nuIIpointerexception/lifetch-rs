use crate::variable::iterator::{Type, VariableIterator};
use std::collections::HashMap;

pub struct Creator<'t> {
    tokens: Vec<Type<'t>>,
}

impl<'t> Creator<'t> {
    #[allow(dead_code)]
    pub fn init(text: &'t str, start: &'t str, end: &'t str) -> Self {
        Self {
            tokens: VariableIterator::new(text, start, end).collect(),
        }
    }

    #[allow(dead_code)]
    pub fn process_variables(&self, var: &HashMap<&str, &str>) -> Option<String> {
        let mut result = String::new();

        for segment in &self.tokens {
            match segment {
                Type::Default(s) => result.push_str(s),
                Type::Variable(s) => match var.get(s.to_uppercase().as_str()) {
                    Some(value) => result.push_str(value),
                    None => {
                        println!("Placeholder '{}' seems to be incorrect.", s);
                    }
                },
            }
        }

        Some(result)
    }

    #[allow(dead_code)]
    pub fn process_case(&self, i: u8) -> Option<String> {
        let mut result = String::new();

        for segment in &self.tokens {
            match segment {
                Type::Default(s) => result.push_str(s),
                Type::Variable(s) => match i {
                    0 => result.push_str(s.to_uppercase().as_str()),
                    1 => result.push_str(s.to_lowercase().as_str()),
                    _ => {
                        println!("Case '{}' seems to be incorrect.", i);
                    }
                },
            }
        }

        Some(result)
    }
}
