use crate::{set1::challenge2::fixed_xor, set2::challenge10::aes_128_ecb_encrypt};

use openssl;
use rand::prelude::*;

pub fn aes_128_ecb_encrypt_block(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8> {
    let cipher = openssl::symm::Cipher::aes_128_ecb();

    let mut c = openssl::symm::Crypter::new(cipher, openssl::symm::Mode::Encrypt, key, iv).unwrap();
    c.pad(false);

    let mut out = vec![0; data.len() + cipher.block_size()];
    let count = c.update(data, &mut out).unwrap();
    out.truncate(count);

    out
}

pub fn aes_128_cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let chunks = plaintext.chunks(16);
    let mut last = iv.to_vec();
    chunks
        .flat_map(|chunk| {
            let new_plaintext = fixed_xor(chunk, &last);
            last = aes_128_ecb_encrypt_block(key, None, &new_plaintext);

            last.clone()
        })
        .collect()
}

pub fn rand_aes_128_key() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 16];

    rng.fill(&mut key);

    key
}

pub fn rand_encrypt(plaintext: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut data = vec![];
    let key = rand_aes_128_key();

    while rng.gen::<f32>() < 0.8 {
        data.push(rng.gen::<u8>());
    }

    data.append(&mut plaintext.to_vec());

    while rng.gen::<f32>() < 0.8 {
        data.push(rng.gen::<u8>());
    }

    if rand::random() {
        let mut iv = [0u8; 16];
        rng.fill(&mut iv);

        aes_128_cbc_encrypt(&key, &iv, &data)
    } else {
        aes_128_ecb_encrypt(&key, None, &data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::set2::{challenge10::aes_128_cbc_decrypt, challenge9::pkcs_7_pad};

    #[test]
    fn aes_128_cbc() {
        let mut plaintext = b"Colorless green ideas sleep furiously.".to_vec();
        pkcs_7_pad(&mut plaintext, 48);

        let key = b"YELLOW SUBMARINE";
        let iv = [0u8; 16];

        let encrypted = aes_128_cbc_encrypt(key, &iv, &plaintext);
        assert_eq!(aes_128_cbc_decrypt(key, &iv, &encrypted), plaintext);
    }
}
