pub fn from(string: &str) -> Result<Vec<u8>, core::num::ParseIntError> {
    (0..string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&string[i..i + 2], 16))
        .collect()
}
