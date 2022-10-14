//! Main crate for the `lightfetch` project.
#![recursion_limit = "130"]
extern crate core;

use crate::args::{Args, Argument};
use crate::builder::{create_spaces, Builder};
use crate::config::cache::Cache;
use crate::config::helio::Helio;
use crate::data::FetchData;
use crate::image::process_image;
use crate::modules::module::FetchModule;
use crate::modules::FetchModules;
use crate::util::constants::CATS;
use crate::util::hyperstr::{ascii_check, Ansi};
use crate::util::mth::to_vector;
use crate::util::unicode::Unicode;
use crate::variable::creator::Creator;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod args;
mod builder;
mod config;
mod data;
mod error;
mod fetch;
mod image;
mod modules;
mod platform;
mod random;
mod util;
mod variable;
mod version;
mod welcome;

/// # Lightfetch
/// ## This is my first time actually "finishing" a project.
/// ## On top of that im pretty new to rust and still learning.
/// ## I'm not sure if I'm doing it right, but I'm pretty sure it works. 👀
///
/// For any feature requests, please open up an issue on github.
/// Github: https://github.com/bwte/lightfetch/issues
/// #
/// Made with ❤️ by bwte#6092

/// This function is the main entry point for [lightfetch].
/// Amazing, right?
///
/// # Arguments:
/// * `args`: The arguments passed to the program.
/// # Returns:
/// The best system information tool ever.
/// #
/// # Example:
/// ```
///       |\      _,,,---,,_            bwte@archlinux
/// ZZZzz /,`.-'`'    -.  ;-;;,_        Distro: psst!
///      |,4-  ) )-,_. ,\ (  `'-'       Don't be too loud or
///     '---''(_/--'  `-'\_)            you'll wake him up!
/// ```
///
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arg_builder = Args::new()
        .set_desc("\x1b[33m⚡ \x1b[0mAn extremely fast cross-compatible system information tool.")
        .set_author("\x1b[33m@bwte")
        .set_name("lightfetch\x1b[0m");

    arg_builder.add("--version", 0, "Prints the current version of Lightfetch.");
    arg_builder.add(
        "--welcome",
        0,
        "Display the beautiful welcome message once again.",
    );

    let mut cfg = util::data::get_env("HOME").unwrap();
    cfg.push_str("/.config/lightfetch/config.ini");

    arg_builder.add_arg(
        Argument::new()
            .alias("--config")
            .alias("-c")
            .alias("--c")
            .count(1)
            .help("Run lightfetch with a custom config file location."),
    );

    let flash_args = arg_builder.parse(None);
    // if let Some(err) = flash_args.as_ref() {
    //     println!("{err}");
    //     exit(1);
    // }
    let args = flash_args.unwrap();

    let cfg_arg = args.get("--config");
    if let Some(..) = cfg_arg {
        cfg = cfg_arg.unwrap().val().get(0).unwrap().to_string();
    }

    // Create a config instance!
    let mut config = Helio::new();
    config.load(cfg.as_str()).unwrap();

    if args.has("--version") {
        version::show_version();
    }

    if args.has("--welcome") {
        welcome::show_welcome(cfg);
    }

    let mut mds = FetchModules::new();

    // Start getting the data. (Running in parallel)
    let fetch_data = prepare_data();

    // Get the fetch String.
    let fetch = to_vector(config.get_str("FETCH", "text").unwrap());

    // Create a variable to put the art later inside.
    let mut art_raw = String::new();

    // Art mode.
    let art_mode = config.get_str("ART", "mode").unwrap().to_lowercase();

    // Art path.
    let cfg = config
        .get_str("ART", "path")
        .unwrap()
        .replace('~', &util::data::get_env("HOME").unwrap());

    // Determine if we use image or ascii mode.
    if art_mode == "image" {
        // Image mode is enabled.
        let size = config.get_int("IMAGE", "size").unwrap();
        let filter = config.get_filter("IMAGE", "filter").unwrap();

        let cache = Cache::new(
            config.clone(),
            config
                .get_str("CACHE", "^ path")
                .unwrap()
                .replace('~', &util::data::get_env("HOME").unwrap())
                .as_str(),
        );
        let hash = util::data::get_fake_hash(
            Path::new(cfg.as_str()),
            size,
            config.get_str("IMAGE", "filter").unwrap(),
        );
        if config.get_bool("CACHE", "enable").unwrap() && cache.exists(hash.clone()) {
            // Image is in cache.
            let image = cache.get(hash).unwrap();
            art_raw = image.get_data().to_string();
        } else {
            // Image is not in cache.
            let img = process_image(cfg.as_str(), size, filter);
            art_raw = img.clone();
            if config.get_bool("CACHE", "enable").unwrap() {
                // Cache image.
                let mut path = util::data::get_env("HOME").unwrap();
                path.push_str("/.config/lightfetch/cache/");
                path.push_str(&hash);
                let mut w = File::create(path).unwrap();
                w.write_all(img.as_ref()).unwrap();
                w.flush().unwrap();
            }
        }
    }
    if art_mode == "ascii" {
        // ASCII mode is enabled.
        let mut file = File::open(cfg).expect("Unable to open the file");
        file.read_to_string(&mut art_raw)
            .expect("Unable to read the file");
    }

    let art_lines = art_raw.lines().count();
    let fetch_lines = fetch.len();

    let should_reverse = config.get_bool("FETCH", "reverse").unwrap();

    // Gap between art and modules.
    for module in fetch {
        if !should_reverse {
            let mut str = config.get_str("FETCH", "gap").unwrap();
            str.push_str(module.as_str());
            mds.add_module(FetchModule::new(str.clone()));
        } else {
            let mut str = module;
            let gap = config.get_str("FETCH", "gap").unwrap();
            str.push_str(gap.as_str());
            mds.add_module(FetchModule::new(str.clone()));
        }
    }

    let mut modules = String::new();
    let mut art = String::new();

    let should_center = config.get_bool("GENERAL", "auto center").unwrap();

    // Center the modules.
    if should_center && art_lines > fetch_lines {
        let diff = (art_lines - fetch_lines) / 2;
        for _ in 0..diff {
            modules.push('\n');
        }
    }

    // Check if the art is centered.
    ascii_check(should_center, &mut art, fetch_lines, art_lines);

    // Add modules.
    for m in mds.get_modules() {
        modules.push_str(m.get_string());
        modules.push('\n');
    }

    // Add art.
    art.push_str(art_raw.as_str());

    // Check if the art is centered. (Again)
    ascii_check(should_center, &mut art, fetch_lines, art_lines);
    let mut modules = modules;

    if config // If variables are enabled.
        .get_bool("GENERAL", "enable variables")
        .unwrap()
    {
        modules = variable_creator(
            modules,
            fetch_data,
            config.get_str("GENERAL", "^ prefix").unwrap(),
            config.get_str("GENERAL", "^ suffix").unwrap(),
        );
    }

    if config // If case variables are enabled.
        .get_bool("GENERAL", "enable case variables")
        .unwrap()
    {
        let u_letter = config.get_str("GENERAL", "^ uppercase letter").unwrap();
        let l_letter = config.get_str("GENERAL", "^ lowercase letter").unwrap();

        let prefix = config.get_str("GENERAL", "^ case prefix").unwrap();
        let suffix = config.get_str("GENERAL", "^ case suffix").unwrap();

        // This might look like spaghetti but it's not. Ok it is a little spaghetti.
        // Case formatting variables will get handled here.
        modules = Creator::init(
            &Creator::init(
                &modules,
                prefix
                    .replace("{letter}", &u_letter)
                    .replace('"', "")
                    .as_str(),
                suffix
                    .replace("{letter}", &u_letter)
                    .replace('"', "")
                    .as_str(),
            )
            .process_case(0)
            .unwrap(),
            prefix
                .replace("{letter}", &l_letter)
                .replace('"', "")
                .as_str(),
            suffix
                .replace("{letter}", &l_letter)
                .replace('"', "")
                .as_str(),
        )
        .process_case(1)
        .unwrap();
    }

    if should_reverse {
        std::mem::swap(&mut art, &mut modules);
    }

    let art: Vec<&str> = art.lines().collect();

    let text: Vec<&str> = modules.lines().collect();
    let mut new_text: Vec<String> = Vec::new();

    let fetch_text = Builder::from(vec![art, {
        let mut m_longest = 0;

        // Get the longest module.
        for m in &text {
            if m.contains("{IGNORE}") || !m.contains("{FILL}") {
                continue;
            }
            let line: Vec<&str> = m.split("{FILL}").collect::<Vec<&str>>();
            let stripped = line.first().unwrap().strip_ansi_colors();
            let len = stripped.width();
            if len > m_longest {
                m_longest = len;
            }
        }

        // Calculate the difference in length of the String and the longest module.
        for m in text {
            if m.contains("{FILL}") || m.contains("{IGNORE}") {
                let line: Vec<&str> = m.split("{FILL}").collect::<Vec<&str>>();
                let first = line.first().unwrap().to_string();
                let past = line.get(1).unwrap_or(&"");
                let mut stripped = first.strip_ansi_colors(); // Strip colors and the fill variable.
                let mut output = first;

                while stripped.width() < m_longest {
                    stripped.push(' ');
                    output.push(' ');
                }
                output.push_str(past);
                new_text.push(output.replace("{IGNORE}", ""));
            } else {
                new_text.push(m.to_string());
            }
        }

        new_text.iter().map(|s| s.as_str()).collect()
    }])
    .set_tab(15)
    .column_builder();
    println!("{}", fetch_text);
    Ok(())
}

/// Replaces the placeholders in the String with the data from the FetchData struct.
/// We compare the vector array position of the placeholder with the number of the actually variable to get the correct data.
/// # Arguments:
/// * `String`: The String to replace the placeholders in.
/// * `data`: The FetchData struct to get the data from.
/// # Returns:
/// The String with the placeholders replaced.
///
fn variable_creator(target: String, mds: FetchData, prefix: String, suffix: String) -> String {
    // TODO: BACKGROUND VALUES? cba rn.
    let mut val = util::hyperstr::colormap();

    // Replace the placeholders with the data.
    val.insert("USERNAME", &mds.user.username);
    val.insert("HOSTNAME", &mds.user.hostname);
    val.insert("DISTRO_NAME", &mds.distro.name);
    val.insert("DISTRO_PRETTY_NAME", &mds.distro.pretty_name);
    val.insert("DISTRO_BUILD", &mds.distro.build);
    val.insert("DISTRO_ID", &mds.distro.id);
    val.insert("DISTRO_ARCH", &mds.distro.architecture);
    val.insert("KERNEL", &mds.distro.kernel);
    val.insert("SHELL", &mds.shell.shell);
    val.insert("TERMINAL", &mds.terminal.terminal);
    val.insert("PACKAGES", &mds.packages.pacman);
    val.insert("UPTIME", &mds.uptime.uptime_raw);
    val.insert("CPU_MODEL", &mds.cpu.model_name);
    val.insert("CPU_CORES", &mds.cpu.cores);
    val.insert("FILL", "{FILL}");
    val.insert("IGNORE", "{IGNORE}");

    let string = target;

    let creator = Creator::init(&string, prefix.as_str(), suffix.as_str());

    creator.process_variables(&val).unwrap()
}

/// Prepare the Data for the fetch.
///
/// # Returns:
/// The FetchData struct with the data.
///
fn prepare_data() -> FetchData {
    let mut data = FetchData::new();
    // TODO: Error catching.
    std::thread::scope(|s| {
        s.spawn(|| {
            data.get_packages();
            data.get_shell();
            data.get_user();
            data.get_distro();
            data.get_terminal();
            data.get_uptime();
            data.get_memory();
            data.get_cpu();
        });
    });
    data
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn hello_world() {
        println!("Hello World!");
    }
}
