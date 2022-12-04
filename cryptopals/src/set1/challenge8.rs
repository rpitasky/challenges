use std::collections::HashSet;

pub fn is_likely_encrypted_with_ecb(ciphertext: &[u8]) -> bool {
    let mut unique = HashSet::new();
    !ciphertext.chunks(16).all(move |chunk| unique.insert(chunk))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let mut ciphertexts = include_str!("data/challenge8.txt")
            .lines()
            .map(|line| byte_vec::from(line).unwrap());

        let candidate_position = ciphertexts
            .position(|ciphertext| is_likely_encrypted_with_ecb(&ciphertext))
            .unwrap();

        assert_eq!(candidate_position, 132);
    }
}
