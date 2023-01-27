/// Reads a single line from the standard input.
///
/// The terminating `\n` character is preserved, but will be absent on end of input.
///
/// # Panics
///
/// This function panics if an error occurs whilst reading the standard input of the program.
///
/// # Examples
///
/// Reading a single line:
///
/// ```no_run
/// let line = ftkit::read_line();
/// println!("You just wrote: {}", line.trim());
/// ```
///
/// Reading lines until the End-Of-File:
///
/// ```no_run
/// loop {
///     let line = ftkit::read_line();
///     if line.is_empty() {
///         break;
///     }
///     println!("You just wrote: {}", line.trim());
/// }
/// ```
pub fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("failed to read from stdin");
    result
}

/// Reads a number from the standard input. The function loops indefinitely until a valid number is
/// provided. If the End-Of-File is reached, the function panics.
///
/// # Panics
///
/// This function panics if it fails to read from the standard input of the program.
///
/// # Examples
///
/// ```no_run
/// println!("How old are you?");
/// let age = ftkit::read_number();
/// println!("Oh? So you are {age} year(s) old?");
/// ```
pub fn read_number() -> i32 {
    loop {
        let s = read_line();
        assert!(!s.is_empty(), "EOF reached :(");
        if let Ok(val) = s.trim().parse() {
            break val;
        }
    }
}
