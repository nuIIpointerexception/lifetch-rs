use crate::util;

/// # Returns data of the current distro.
///
/// # Examples
/// ## name -> "Gentoo"
/// ## pretty_name -> "Gentoo/Linux"
/// ## build -> "rolling"
/// ## id -> "gentoo"
/// ## architecture -> "x86_64"
/// ## kernel -> "5.4.0-gentoo"
///
pub struct DistroInfo<String> {
    pub name: String,
    pub pretty_name: String,
    pub build: String,
    pub id: String,
    pub architecture: String,
    pub kernel: String,
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn distro(mut data: &mut DistroInfo<String>) {
    for i in util::data::get_data("/etc/os-release", 0)
        .unwrap()
        .split('\n')
    {
        let mut j = i.split('=');

        match j.next().unwrap() {
            "NAME" => data.name = j.next().unwrap().replace('"', ""),
            "PRETTY_NAME" => data.pretty_name = j.next().unwrap().replace('"', ""),
            "ID" => data.id = j.next().unwrap().trim().to_string(),
            "BUILD_ID" => data.build = j.next().unwrap().trim().to_string(),
            _ => (),
        }
    }

    let k_data = util::data::get_data("/proc/sys/kernel/osrelease", 4096).unwrap();
    data.kernel = k_data.trim().to_string();
}
