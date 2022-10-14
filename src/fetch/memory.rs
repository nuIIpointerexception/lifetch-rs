use crate::util;
use crate::util::mth::format_gb_str;
use std::ops::Sub;

pub struct MemInfo<String> {
    pub total: String,
    pub free: String,
    pub avail: String,
    pub used: String,
    pub cached: String,
    pub buffers: String,

    pub swap_used: String,
    pub swap_free: String,
    pub swap_total: String,
}

impl MemInfo<String> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut String> {
        let iter = vec![
            &mut self.total,
            &mut self.free,
            &mut self.avail,
            &mut self.used,
            &mut self.cached,
            &mut self.buffers,
            &mut self.swap_used,
            &mut self.swap_free,
            &mut self.swap_total,
        ];
        iter.into_iter()
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn memory(mut data: &mut MemInfo<String>) {
    let mut memory_info = MemInfo {
        total: "".to_string(),
        free: "".to_string(),
        avail: "".to_string(),
        used: "".to_string(),
        cached: "".to_string(),
        buffers: "".to_string(),
        swap_used: "".to_string(),
        swap_free: "".to_string(),
        swap_total: "".to_string(),
    };
    for line in util::data::get_data("/proc/meminfo", 4096).unwrap().lines() {
        let mut split = line.split(':');
        let key = split.next().unwrap().trim().to_string();
        let value = split
            .next()
            .unwrap()
            .replace("kB", "")
            .replace(' ', "")
            .trim()
            .to_string();
        let value = value.trim();
        let value = value.parse::<u64>().unwrap();
        let value = value / 1024;
        let value = value.to_string();
        match key.as_ref() {
            "MemTotal" => memory_info.total = value,
            "MemFree" => memory_info.free = value,
            "MemAvailable" => memory_info.avail = value,
            "Cached" => memory_info.cached = value,
            "Buffers" => memory_info.buffers = value,
            "SwapTotal" => memory_info.swap_total = value,
            "SwapFree" => memory_info.swap_free = value,
            _ => (),
        }
    }
    // Do those later, we can calculate them ourselves:
    memory_info.used = memory_info
        .total
        .parse::<f64>()
        .unwrap()
        .sub(memory_info.free.parse::<f64>().unwrap())
        .to_string();
    memory_info.swap_used = memory_info
        .swap_total
        .parse::<u64>()
        .unwrap()
        .sub(memory_info.swap_free.parse::<u64>().unwrap())
        .to_string();

    // Finally apply the formatting:
    for i in memory_info.iter_mut() {
        *i = format_gb_str(i.parse::<f64>().unwrap());
    }
    data.avail = memory_info.avail.clone();
    data.buffers = memory_info.buffers.clone();
    data.cached = memory_info.cached.clone();
    data.free = memory_info.free.clone();
    data.total = memory_info.total.clone();
    data.used = memory_info.used.clone();
    data.swap_free = memory_info.swap_free.clone();
    data.swap_total = memory_info.swap_total.clone();
    data.swap_used = memory_info.swap_used.clone();
}
