pub fn log_with_header_len(msg: &str, len: usize) {
    let dashes = "-".repeat((len - msg.len() - 2) / 2);
    println!("{} {} {}", dashes, msg, dashes);
}

pub fn log_with_header(msg: &str) {
    log_with_header_len(msg, 80);
}
