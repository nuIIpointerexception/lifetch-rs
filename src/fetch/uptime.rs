use crate::util;

pub struct UptimeInfo<String> {
    pub uptime_raw: String,
    pub days: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub z_days: String,
    pub z_hours: String,
    pub z_minutes: String,
    pub z_seconds: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn uptime(mut data: &mut UptimeInfo<String>) {
    let udata = util::data::get_data("/proc/uptime", 50).unwrap();
    let uptime_text = udata.split_whitespace().next().unwrap();
    let uptime_raw = uptime_text.parse::<f64>().unwrap();
    let days = filter_null((uptime_raw / 86400.0).floor());
    let hours = filter_null(((uptime_raw / 3600.0) % 24.0).floor());
    let minutes = filter_null(((uptime_raw / 60.0) % 60.0).floor());
    let seconds = filter_null((uptime_raw % 60.0).floor());
    data.uptime_raw = uptime_text.to_string();
    data.days = days.to_string();
    data.hours = hours.to_string();
    data.minutes = minutes.to_string();
    data.seconds = seconds.to_string();
    data.z_days = time_prefix(days);
    data.z_hours = time_prefix(hours);
    data.z_minutes = time_prefix(minutes);
    data.z_seconds = time_prefix(seconds);
}

pub fn filter_null(s: f64) -> String {
    if s == 0.0 {
        return "".to_string();
    }
    s.to_string()
}

pub fn time_prefix(s: String) -> String {
    let s = s;
    let mut new: String = String::new();
    if s.len() == 1 {
        new.push('0');
        new.push_str(&s);
        return new;
    }
    s
}
