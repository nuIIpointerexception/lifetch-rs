use crate::util;
use crate::util::mth::{floor, mhz_to_ghz};

pub struct CpuInfo<String> {
    pub mhz: String,
    pub ghz: String,
    pub bogomips: String,
    pub cores: String,
    pub threads: String,
    pub vendor: String,
    pub model: String,
    pub family: String,
    pub stepping: String,
    pub model_name: String,
    pub flags: String,
    pub cache_size: String,
    pub cpu_load: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn cpu(mut data: &mut CpuInfo<String>) {
    for line in util::data::get_data("/proc/cpuinfo", 1000).unwrap().lines() {
        if line.starts_with("cpu MHz") {
            let mut j = line.split(':');
            j.next();
            data.mhz = j.next().unwrap().trim().to_string();
            data.ghz = mhz_to_ghz(data.mhz.clone());
        }
        if line.starts_with("model name") {
            data.model_name = line.split(':').nth(1).unwrap().trim().to_string();
        }
        if line.starts_with("vendor_id") {
            let vendor = line.split(':').nth(1).unwrap().trim();
            match vendor {
                "GenuineIntel" => data.vendor = "Intel".to_string(),
                "AuthenticAMD" => data.vendor = "AMD".to_string(),
                _ => data.vendor = vendor.to_string(),
            }
        }

        if line.starts_with("cpu cores") {
            data.cores = line.split(':').nth(1).unwrap().trim().to_string();
        }

        if line.starts_with("bogomips") {
            data.bogomips = line.split(':').nth(1).unwrap().trim().to_string();
        }

        if line.starts_with("cpu family") {
            data.family = line.split(':').nth(1).unwrap().trim().to_string();
        }

        if line.starts_with("cpu threads") {
            data.threads = line.split(':').nth(1).unwrap().trim().to_string();
        }
    }
    data.cpu_load = get_cpu_load()
}

pub fn get_cpu_load() -> String {
    let s = util::data::get_data("/proc/loadavg", 4096).unwrap();
    let a: f32 = s.split_whitespace().next().unwrap().parse().unwrap();
    floor(a * 100.0, 100_f32).to_string()
}
