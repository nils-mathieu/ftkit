/// Reads a single line from *stdin*.
pub fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("failed to read from stdin");

    // Trim starting whitespaces.
    let start = result
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)
        .unwrap_or_default();

    // SAFETY:
    //  We made sure that `start` is on a character boundary.
    unsafe { result.as_bytes_mut().copy_within(start.., 0) };
    result.truncate(result.len() - start);

    // Trim eventual trailing line feed.
    let end = result
        .char_indices()
        .rev()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(result.len());
    result.truncate(end);

    result
}

/// Reads a number from *stdin*.
pub fn read_number() -> i32 {
    read_line()
        .parse()
        .expect("the provided value is not a number")
}
