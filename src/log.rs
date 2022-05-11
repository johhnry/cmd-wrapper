use std::time;

pub fn log_with_header_len(msg: &str, len: usize) {
    let dashes = "-".repeat((len - msg.len() - 2) / 2);
    println!("{} {} {}", dashes, msg, dashes);
}

pub fn log_with_header(msg: &str) {
    log_with_header_len(msg, 80);
}

pub fn format_duration(duration: &time::Duration) -> String {
    let secs = duration.as_secs();

    // Format to 0h 0m 0s when duration greater than 1s
    if secs >= 1 {
        let minutes = (secs / 60) % 60;
        let hours = (secs / 60) / 60;
        return format!("{}h {}m {}s", hours, minutes, secs);
    }

    return format!("{:.2?}", duration);
}
