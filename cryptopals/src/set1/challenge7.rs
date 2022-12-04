pub fn aes_128_ecb_decrypt(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8> {
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    openssl::symm::decrypt(cipher, key, iv, data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use openssl;

    #[test]
    fn provided_example() {
        let ciphertext = include_str!("data/challenge7.txt");
        let decoded = openssl::base64::decode_block(ciphertext).unwrap();

        let key = b"YELLOW SUBMARINE";

        let plaintext = aes_128_ecb_decrypt(key, None, &decoded);

        let expected = include_str!("data/challenge7_solution.txt");
        assert_eq!(openssl::base64::encode_block(&plaintext), expected);
    }
}
