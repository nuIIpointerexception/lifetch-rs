/// Format a number of bytes as a String in GB, or MB, or KB.
///
/// # Arguments:
/// * `value`: The value to format.
///
/// # Returns:
/// A String containing the formatted value.
///
pub fn format_gb_str(value: f64) -> String {
    let precision = 2;
    if value > 1000.0 {
        let mut out = round(value / 1000.0, 1 + precision);
        out.push_str(" GB");
        return out;
    }
    if value > 1.0 {
        let mut out = round(value, 1 + precision);
        out.push_str(" MB");
        return out;
    }
    let mut out = round(value, 1 + precision);
    out.push_str(" KB");
    out
}
/// Rust doesn't have a built-in way to round a number to a certain number of decimal places.
/// So we use this function to do it for us.
///
/// It's not perfect, but it's good enough.
///
///
/// Credits: https://stackoverflow.com/questions/60497397
///
/// # Arguments:
/// * `value`: The number to round.
/// * `precision`: The number of decimal places to round to.
///
/// # Returns:
/// The rounded number as a String.
///
fn round(value: f64, precision: usize) -> String {
    let a = value.abs();
    let precision = if a >= 1. {
        let n = (1. + a.log10().floor()) as usize;
        if n <= precision {
            precision - n
        } else {
            0
        }
    } else if a > 0. {
        let n = -(1. + a.log10().floor()) as usize;
        precision + n
    } else {
        0
    };
    format!("{0:.1$}", value, precision)
}

/// Limit the number so it doesn't go over the set maximum value.
///
/// # Arguments:
/// * `f`: The number to floor
/// * `max`: The maximum value that the function can return.
///
/// # Returns:
/// Returns the limited value if `f` is higher than `max`, otherwise returns the unmodified value.
///
pub fn floor(f: f32, max: f32) -> f32 {
    if f > max {
        max
    } else {
        f
    }
}

/// Convert a String displaying a MHz value to a GHz value.
///
/// # Arguments:
/// * `s`: String - The String to be converted.
///
/// # Returns:
/// A String but as a GHz value.
///
pub fn mhz_to_ghz(s: String) -> String {
    let f: f32 = s.replace(" MHz", " GHz").parse().unwrap();
    format!("{:.2}", f / 1000.0)
}

/// Convert a multiline String to a vector of strings.
///
/// # Arguments
/// * `s`: The String to be converted.
///
/// # Returns
/// A vector of strings.
///
pub fn to_vector(map: String) -> Vec<String> {
    let mut vec = Vec::new();
    for x in map.split('\n') {
        vec.push(x.to_string());
    }
    vec
}
