use super::challenge2::fixed_xor;
use crate::core::char_frequencies;

pub fn crack_single_byte_xor(ciphertext: &[u8]) -> (f32, Vec<u8>) {
    (0..255)
        .map(|byte| {
            let string = fixed_xor(ciphertext, &[byte].repeat(ciphertext.len()));

            (char_frequencies::score_string(&string), string)
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let ciphertext =
            byte_vec::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();

        let result = crack_single_byte_xor(&ciphertext).1;

        assert_eq!(
            Ok("Cooking MC's like a pound of bacon".to_string()),
            String::from_utf8(result),
        );
    }
}
