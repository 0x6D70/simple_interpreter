pub fn report_error(message: &str, file_name: &str, line: usize) {
    eprintln!("\x1b[91mERROR\x1b[0m {} in {}:{}", message, file_name, line);
}

#[allow(dead_code)]
pub fn report_warning(message: &str, file_name: &str, line: usize) {
    println!(
        "\x1b[93mWARNING\x1b[0m {} in {}:{}",
        message, file_name, line
    );
}
