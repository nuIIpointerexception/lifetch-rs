use crate::fetch::cpu::{cpu, CpuInfo};
use crate::fetch::distro::{distro, DistroInfo};

use crate::fetch::memory::{memory, MemInfo};
use crate::fetch::packages::{packages, PackageInfo};
use crate::fetch::shell::{shell, ShellInfo};
use crate::fetch::terminal::{terminal, TerminalInfo};
use crate::fetch::uptime::{uptime, UptimeInfo};
use crate::fetch::user::{user, UserInfo};

pub struct FetchData {
    pub user: UserInfo<String>,
    pub distro: DistroInfo<String>,
    pub shell: ShellInfo<String>,
    pub terminal: TerminalInfo<String>,
    pub packages: PackageInfo<String>,
    pub uptime: UptimeInfo<String>,
    pub memory: MemInfo<String>,
    pub cpu: CpuInfo<String>,
    pub gpu: String,
}

pub static UNKNOWN: &str = "UNKNOWN";

impl FetchData {
    /// This creates a new FetchData instance.
    /// # Returns:
    /// The instance.
    ///
    pub fn new() -> Self {
        FetchData {
            user: UserInfo {
                username: UNKNOWN.to_string(),
                hostname: UNKNOWN.to_string(),
            },
            distro: DistroInfo {
                name: UNKNOWN.to_string(),
                pretty_name: UNKNOWN.to_string(),
                build: UNKNOWN.to_string(),
                id: UNKNOWN.to_string(),
                architecture: UNKNOWN.to_string(),
                kernel: UNKNOWN.to_string(),
            },
            shell: ShellInfo {
                shell: UNKNOWN.to_string(),
                shell_version: UNKNOWN.to_string(),
            },
            terminal: TerminalInfo {
                terminal: UNKNOWN.to_string(),
                terminal_version: UNKNOWN.to_string(),
            },
            packages: PackageInfo {
                pacman: UNKNOWN.to_string(),
                pacman_version: UNKNOWN.to_string(),
                cargo: UNKNOWN.to_string(),
                cargo_version: UNKNOWN.to_string(),
            },
            uptime: UptimeInfo {
                uptime_raw: UNKNOWN.to_string(),
                days: UNKNOWN.to_string(),
                hours: UNKNOWN.to_string(),
                minutes: UNKNOWN.to_string(),
                seconds: UNKNOWN.to_string(),
                z_days: UNKNOWN.to_string(),
                z_hours: UNKNOWN.to_string(),
                z_minutes: UNKNOWN.to_string(),
                z_seconds: UNKNOWN.to_string(),
            },
            memory: MemInfo {
                total: UNKNOWN.to_string(),
                free: UNKNOWN.to_string(),
                avail: UNKNOWN.to_string(),
                used: UNKNOWN.to_string(),
                buffers: UNKNOWN.to_string(),
                cached: UNKNOWN.to_string(),
                swap_total: UNKNOWN.to_string(),
                swap_free: UNKNOWN.to_string(),
                swap_used: UNKNOWN.to_string(),
            },
            cpu: CpuInfo {
                mhz: UNKNOWN.to_string(),
                ghz: UNKNOWN.to_string(),
                bogomips: UNKNOWN.to_string(),
                cores: UNKNOWN.to_string(),
                threads: UNKNOWN.to_string(),
                vendor: UNKNOWN.to_string(),
                model: UNKNOWN.to_string(),
                family: UNKNOWN.to_string(),
                stepping: UNKNOWN.to_string(),
                model_name: UNKNOWN.to_string(),
                flags: UNKNOWN.to_string(),
                cache_size: UNKNOWN.to_string(),
                cpu_load: UNKNOWN.to_string(),
            },
            gpu: UNKNOWN.to_string(),
        }
    }

    /// Update or get the uptime data.
    /// # Returns:
    /// The Uptime Data.
    ///
    pub fn get_uptime(&mut self) {
        uptime(&mut self.uptime)
    }

    /// Update or get the terminal data.
    ///
    /// # Returns:
    /// The Terminal Data.
    ///
    pub fn get_terminal(&mut self) {
        terminal(&mut self.terminal, &self.shell.shell)
    }

    /// Update or get the package data.
    /// # Returns:
    /// The Package Data.
    ///
    pub fn get_packages(&mut self) {
        packages(&mut self.packages)
    }

    /// Update or get the user data.
    /// # Returns:
    /// The User Data.
    ///
    pub fn get_user(&mut self) {
        user(&mut self.user)
    }

    /// Update or get the shell data.
    /// # Returns:
    /// The Shell Data.
    ///
    pub fn get_shell(&mut self) {
        shell(&mut self.shell)
    }

    /// Update or get the distro data.
    /// # Returns:
    /// The Distro Data.
    ///
    pub fn get_distro(&mut self) {
        distro(&mut self.distro)
    }

    /// Update or get the memory data.
    /// # Returns:
    /// The Memory Data.
    ///
    pub fn get_memory(&mut self) {
        memory(&mut self.memory)
    }

    /// Update or get the cpu data.
    /// # Returns:
    /// The Cpu Data.
    ///
    pub fn get_cpu(&mut self) {
        cpu(&mut self.cpu)
    }
}
