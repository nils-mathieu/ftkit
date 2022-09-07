/// Reads a single line from the standard input.
///
/// # Panics
///
/// This function panics if an error occurs whilst reading the standard input of the program.
///
/// # Examples
///
/// ```ignore
/// let line = ftkit::read_line();
/// println!("You just wrote: {line}");
/// ```
pub fn read_line() -> String {
    print!(">_ ");

    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("failed to read from stdin");

    // Trim starting whitespaces.
    let start = result
        .find(|c: char| !c.is_whitespace())
        .unwrap_or_default();

    // SAFETY:
    //  We made sure that `start` is on a character boundary.
    unsafe { result.as_bytes_mut().copy_within(start.., 0) };
    result.truncate(result.len() - start);

    // Trim eventual trailing line feed.
    let mut end = result
        .rfind(|c: char| !c.is_whitespace())
        .unwrap_or(result.len());
    // `rfind` returns the index of the first maching character. What we want is the index of the
    // first non-maching character. For this reason, we have to find the length of the character
    // and add it to the result.
    match result.as_bytes().get(end) {
        Some(&b) if b & 0x80 == 0x00 => end += 1, // byte starts with 0b0
        Some(&b) if b & 0xe0 == 0xc0 => end += 2, // byte starts with 0b110
        Some(&b) if b & 0xf0 == 0xe0 => end += 3, // byte starts with 0b1110
        Some(&b) if b & 0xf8 == 0xf0 => end += 4, // byte starts with 0b11110
        _ => (), // Either the data is not valid UTF-8 or we reached the end of the string.
    }
    result.truncate(end);

    result
}

/// Reads a number from the standard input. The function loops indefinitely until a valid number is
/// provided.
///
/// # Panics
///
/// This function panics if it fails to read from the standard input of the program.
///
/// # Examples
///
/// ```ignore
/// println!("How old are you?");
/// let age = ftkit::read_number();
/// println!("Oh? So you are {age} year(s) old?");
/// ```
pub fn read_number() -> i32 {
    loop {
        if let Ok(val) = read_line().parse() {
            break val;
        }
    }
}
