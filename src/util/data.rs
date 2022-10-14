use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::{env, io};

/// # This file is a bit messy, but i can't be bothered to clean it up.

/// Allows you to read environmental variables.
///
/// # Arguments:
/// * `key`: The name of the environment variable to get.
///
/// # Returns:
/// A String containing the contents of the environment variable.
///
pub fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Returns the size of a file in a human-readable format.
///
/// # Arguments:
///
/// * `file`: &mut File - The file to read from.
/// * `size`: The size of the file in bytes.
///
/// # Returns:
/// The file as a String.
///
fn file_data(file: &mut File, size: usize) -> io::Result<String> {
    let mut buf = String::with_capacity(size);
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

/// Get the contents of a file.
///
/// # Arguments:
/// * `file_path`: The path to the file you want to read.
/// * `size`: The size of the file in bytes.
///
/// # Returns:
/// The contents of the file as a String.
///
pub fn get_data<P: AsRef<Path>>(file_path: P, size: usize) -> io::Result<String> {
    let mut file = File::open(file_path.as_ref())?;
    file_data(&mut file, size)
}

/// Gets a fake hash of a file.
///
/// # Arguments:
/// * `file_path`: The path to the file you want to read.
///
/// # Returns:
/// The hash of the file as a String.
///
/// # Example:
/// ```
/// use lightfetch::files::get_hash;
/// let hash = get_hash("/home/bwte/.config/lightfetch/cache/1.png");
/// ```
///
pub fn get_fake_hash(path: &Path, size: u32, filter: String) -> String {
    let mut out = String::with_capacity(32);
    let metadata = File::open(path).unwrap().metadata().unwrap();
    out.push_str(&metadata.len().to_string());
    out.push_str(size.to_string().as_str());
    out.push_str(filter.as_str());
    out
}
