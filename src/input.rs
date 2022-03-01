/// Reads a single line from *stdin*.
pub fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("failed to read from stdin");
    result
}
