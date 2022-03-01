/// Generates a random number.
pub fn random_number() -> i32 {
    let mut bytes = [0u8; 4];
    getrandom::getrandom(&mut bytes).expect("failed to generate a random number");
    i32::from_ne_bytes(bytes)
}
