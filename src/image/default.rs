use std::str;

use crate::config::cache::Image;
use image::imageops::resize;
use image::imageops::FilterType;
use image::io::Reader;
use image::ImageError;
use image::RgbaImage;

pub struct Default {
    pub image: Image,
}

impl Default {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    /// Process and print an image
    /// # Parameters:
    /// - `file`: Path to the image
    /// - `height`: Height of the image in characters
    /// - `res`: Are we using the half pixel mode ?
    pub fn process(file: &str, height: u32, filter: FilterType) -> String {
        let image: Result<RgbaImage, ImageError> =
            Ok(Reader::open(file).unwrap().decode().unwrap().to_rgba8());
        let img = match image {
            Ok(img) => img,
            Err(e) => {
                println!("{}", e);
                return String::new();
            }
        };
        let w = img.width();
        let h = img.height();
        let img = resize(&img, 2 * w * height / h, height * 2, filter);
        let mut output = String::new();
        for i in (0..img.height() - 1).step_by(2) {
            for j in 0..img.width() {
                let upper = img.get_pixel(j, i); // Upper
                let lower = img.get_pixel(j, i + 1); // Lower
                if (*upper)[3] == 0 && (*lower)[3] == 0 {
                    // Both transparent
                    output.push(' ');
                } else if (*upper)[3] == 0 {
                    // Upper transparent
                    output.push_str(&format!(
                        "\x1b[48;2;{};{};{}m▀",
                        (*lower)[0],
                        (*lower)[1],
                        (*lower)[2]
                    ));
                } else if (*lower)[3] == 0 {
                    // Lower transparent
                    output.push_str(&format!(
                        "\x1b[38;2;{};{};{}m▄",
                        (*upper)[0],
                        (*upper)[1],
                        (*upper)[2]
                    ));
                } else {
                    output.push_str(&format!(
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                        (*upper)[0],
                        (*upper)[1],
                        (*upper)[2],
                        (*lower)[0],
                        (*lower)[1],
                        (*lower)[2]
                    ));
                }
            }
            output.push_str("\x1b[0m\n");
        }
        output
    }
}
