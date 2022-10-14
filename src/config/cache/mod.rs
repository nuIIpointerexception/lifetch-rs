use crate::config::helio::Helio;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::Path;

pub struct Image {
    pub data: String,
    pub hash: String,
}

impl Image {
    /// Create a new `Image` instance. Used by the cache.
    ///
    /// # Arguments:
    /// * `data` - The Art converted into an ascii String.
    /// * `hash` - A "hash" of the image.
    ///
    /// # Returns:
    /// A new `Image` instance.
    ///
    /// # Example:
    /// ```
    /// use config::cache::Image;
    ///
    /// let image = Image::new("data".to_string(), "hash".to_string());
    /// ```
    ///
    pub fn new(data: String, hash: String) -> Image {
        Image { data, hash }
    }

    /// Get the image data.
    ///
    /// # Returns:
    /// A `String` containing the image data.
    ///
    /// # Example:
    /// ```
    /// use config::cache::Image;
    ///
    /// let image = Image::new("data".to_string(), "hash".to_string());
    /// assert_eq!(image.get_data(), "data");
    /// ```
    ///
    pub fn get_data(&self) -> &String {
        &self.data
    }
}

pub struct Cache {
    pub config: Helio,
    pub images: HashMap<String, Image>,
    pub size: usize,
}

impl Cache {
    /// Create a new cache instance and add existing images to the cache.
    /// The Images are stored with their hash as key.
    ///
    /// # Arguments
    /// * `config` - A `Helio` Config instance.
    /// * `dir` - A `String` containing the path to the cache directory.
    /// # Returns:
    /// A `Cache` instance.
    /// # Example:
    /// ```
    /// use helio::config::cache::Cache;
    /// let cache = Cache::new();
    /// ```
    ///
    pub fn new(config: Helio, dir: &str) -> Self {
        let mut size: usize = 0;
        let cache_dir = Path::new(dir);
        let mut images = HashMap::new();
        if cache_dir.is_dir() {
            if let Ok(read_dir) = read_dir(cache_dir) {
                for entry in read_dir.filter_map(Result::ok) {
                    if let Ok(mut file) = File::open(entry.path()) {
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap();
                        let hash = entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        images.insert(hash.clone(), Image::new(data, hash));
                        size += 1;
                    }
                }
            };
        }

        Self {
            size,
            images,
            config,
        }
    }

    /// Get a specific image from the cache by its hash.
    ///
    /// # Arguments:
    /// * `hash` - A `String` containing the hash of the image.
    ///
    /// # Returns:
    /// A `Image` instance.
    ///
    /// # Example:
    /// ```
    /// use helio::config::cache::Cache;
    /// let cache = Cache::new();
    /// let image = cache.get_image("hash".to_string());
    /// ```
    ///
    pub fn get(&self, hash: String) -> Option<&Image> {
        self.images.get(hash.as_str())
    }

    /// Check if the cache contains a image with the given hash.
    ///
    /// # Arguments:
    /// * `hash` - A `String` containing the hash of the image.
    ///
    /// # Returns:
    /// A `bool` value.
    ///
    /// # Example:
    /// ```
    /// use helio::config::cache::Cache;
    /// let cache = Cache::new();
    /// let contains = cache.contains("hash".to_string());
    /// ```
    ///
    pub fn exists(&self, hash: String) -> bool {
        self.images.contains_key(hash.as_str())
    }
}

#[cfg(test)]
pub(crate) mod cache_tests {}
