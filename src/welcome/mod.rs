use crate::util::hyperstr::{BLACK, BLUE, CYAN, GRAY, GREEN, ITALIC, MAGENTA, RED, RESET, YELLOW};
use std::io::{stdin, Read};

pub(crate) fn show_welcome(cfg: String) {
    println!(
        "{GRAY}┌────────────────────────────── {RESET}Welcome to ... {GRAY}──────────────────────────────┐\n"
    );
    let logo = "
██╗     ██╗ ██████╗ ██╗  ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
██║     ██║██╔════╝ ██║  ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
██║     ██║██║  ███╗███████║   ██║   █████╗  █████╗     ██║   ██║     ███████║
██║     ██║██║   ██║██╔══██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
███████╗██║╚██████╔╝██║  ██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
";

    for char in logo.chars() {
        if '█' == char {
            print!("{YELLOW}█");
        } else {
            print!("{BLACK}{}", char);
        }
    }
    println!(
        "        {YELLOW}⚡ {RESET}An extremely fast cross-compatible system information tool."
    );
    println!("{GRAY}└───────────────────────────────────────────────────────────────────────────┘");

    println!("                         {GRAY}Press Enter to continue...");
    let _ = stdin().read(&mut [0]).unwrap();

    println!(
        "{GRAY}┌────────────────────────────── {RESET} Information {GRAY}──────────────────────────────┐"
    );
    println!(
        "\n This is a brief introduction to lightfetch.
        "
    );
    println!(" lightfetch is a system information tool written in Rust 🦀"); // Crab emoji is not supported in all terminals 😥
    println!(" It is extremely fast and lightweight,\n and is designed to be highly customizable.");
    println!(
        "
             The configuration file is found at: {YELLOW}
             {}{RESET}",
        cfg
    );
    println!(
        "\n {GRAY}You can change the {RED}c{BLUE}o{GREEN}l{YELLOW}o{MAGENTA}r{CYAN}s{GRAY}, {ITALIC}fonts{RESET}{GRAY}, and even the order of the information."
    );
    println!(" It is also possible to add custom information.");
    println!(
        "
             You can find the documentation for the configuration file at: {YELLOW}
             https://github.com/bwte/lightfetch/blob/master/CONFIG.md{RESET}"
    );
    println!("{GRAY}└───────────────────────────────────────────────────────────────────────────┘");
    println!("                         {GRAY}Press Enter to continue...");
    let _ = stdin().read(&mut [0]).unwrap();
}
