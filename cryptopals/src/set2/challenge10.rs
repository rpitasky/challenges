use crate::set1::challenge2::fixed_xor;

pub fn aes_128_ecb_decrypt_block(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8> {
    let cipher = openssl::symm::Cipher::aes_128_ecb();

    let mut c = openssl::symm::Crypter::new(cipher, openssl::symm::Mode::Decrypt, key, iv).unwrap();
    c.pad(false);

    let mut out = vec![0; data.len() + cipher.block_size()];
    let count = c.update(data, &mut out).unwrap();
    out.truncate(count);

    out
}

pub fn aes_128_ecb_encrypt(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8> {
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    openssl::symm::encrypt(cipher, key, iv, data).unwrap()
}

pub fn aes_128_cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let chunks = ciphertext.chunks(16);
    let mut last = iv;
    chunks
        .flat_map(|chunk| {
            let decrypted = aes_128_ecb_decrypt_block(key, None, chunk);

            let result = fixed_xor(&decrypted, last);
            last = chunk;

            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use openssl;

    #[test]
    fn provided_example() {
        let ciphertext = include_str!("data/challenge10.txt");
        let decoded = openssl::base64::decode_block(ciphertext).unwrap();

        let key = b"YELLOW SUBMARINE";
        let iv = &[0_u8; 16];

        let expected = include_str!("data/challenge10_solution.txt");
        assert_eq!(
            openssl::base64::encode_block(&aes_128_cbc_decrypt(key, iv, &decoded)),
            expected
        );
    }
}
