pub mod width;

use std::ops::Add;

pub trait Unicode {
    fn width(&self) -> usize;
}

impl Unicode for String {
    fn width(&self) -> usize {
        self.chars()
            .map(|c| width::width(c).unwrap_or(0))
            .fold(0, Add::add)
    }
}
