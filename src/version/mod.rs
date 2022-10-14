use crate::util::hyperstr::{GRAY, RESET, YELLOW};
use crate::{random, CATS};
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn show_version() {
    println!(
        "{YELLOW}⚡{GRAY}LIGHTFETCH v {YELLOW}{}",
        env!("CARGO_PKG_VERSION")
    );

    println!("{GRAY}Made with {YELLOW}💛{GRAY} by bwte");
    println!("Github: https://github.com/bwte/lightfetch");
    println!("\n{GRAY}License: {YELLOW}MIT{RESET}",);

    random::set_seed(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );
    let r = random::rand(0, CATS.len() - 1);

    println!("{YELLOW}{}{RESET}", CATS[r]);
    exit(0);
}
