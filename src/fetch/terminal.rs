use crate::util;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process;

pub struct TerminalInfo<String> {
    pub terminal: String,
    pub terminal_version: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn terminal(mut data: &mut TerminalInfo<String>, shell: &str) {
    let mut terminal_pid = get_parent(process::id() as i32);
    if let Ok(mut terminal_name) = util::data::get_data(
        PathBuf::from("/proc")
            .join(terminal_pid.to_string())
            .join("comm"),
        16_385,
    ) {
        while shell.contains(&terminal_name.replace('\n', "").as_str()) {
            let ppid = get_parent(terminal_pid);
            terminal_pid = ppid;

            if let Ok(comm) = util::data::get_data(
                PathBuf::from("/proc").join(ppid.to_string()).join("comm"),
                16_385,
            ) {
                terminal_name = comm;
            }
        }
        data.terminal = terminal_name.replace('\n', "");
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn get_parent(pid: i32) -> i32 {
    let ppid_file = File::open(PathBuf::from("/proc").join(pid.to_string()).join("stat")).unwrap();
    let mut ppid_reader = BufReader::new(ppid_file);
    let mut ppid_line = String::new();
    ppid_reader.read_line(&mut ppid_line).unwrap();
    ppid_line
        .split_whitespace()
        .nth(3)
        .unwrap()
        .parse::<i32>()
        .unwrap()
}
