pub mod cache;
pub mod helio;

#[cfg(test)]
pub(crate) mod helio_tests {
    use crate::config::helio::Helio;
    use crate::{to_vector, util};

    #[test]
    pub fn collect_modules() {
        let mut cfg = util::data::get_env("HOME").unwrap();
        cfg.push_str("/.config/lightfetch/config.ini");
        let mut config = Helio::new();
        config
            .load(cfg.as_str())
            .expect("Test config file not found");
        let vec = to_vector(config.get_str("FETCH", "text").unwrap());
        println!("{:?}", vec);
        println!("{}", vec[2]);
    }
}
