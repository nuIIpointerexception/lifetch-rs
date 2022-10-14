pub mod unicode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_support() {
        println!("Unicode support: {}", unicode::supports_unicode());
        assert!(unicode::supports_unicode());
    }
}
