use log::warn;

pub fn find_matches(
    lines: impl IntoIterator<Item = String>,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> () {
    for line in lines.into_iter() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Err(err) => warn!("could not write the line: {}", err),
                Ok(_) => (),
            }
        }
    }
}
