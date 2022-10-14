use std::borrow::Borrow;
use crate::config::cache::Image;

mod ascii;
mod default;
mod iterm;
mod kitty;

pub struct ImageBuilder {
    pub image: Image,
}

impl ImageBuilder {
    pub fn init(image: Image) -> Self {
        Self { image }
    }
    pub fn get_mode(&self, mode: &str) -> String {
        match mode {
            _ => default::Default::new(self.image).
        }
    }
}
