use std::collections::HashMap;

use crate::{
    set1::challenge8::is_likely_encrypted_with_ecb,
    set2::{challenge10::aes_128_ecb_encrypt, challenge11::rand_aes_128_key},
};

use openssl;

static mut KEY: [u8; 16] = [0; 16];

fn assign_key() {
    unsafe {
        KEY = rand_aes_128_key();
    }
}

fn get_secret() -> Vec<u8> {
    let data = include_str!("data/challenge12.txt");

    openssl::base64::decode_block(data).unwrap()
}

pub fn aes_128_ecb_with_secret(data: &[u8]) -> Vec<u8> {
    let secret = get_secret();

    let plaintext = [data, &secret].concat();

    let key = unsafe { KEY };
    aes_128_ecb_encrypt(&key, None, &plaintext)
}

pub fn deduce_block_size<F>(mut encrypter: F) -> usize
where
    F: std::ops::FnMut(&[u8]) -> Vec<u8>,
{
    let mut i = 1;
    let mut last = encrypter(&[1u8].repeat(i));
    loop {
        let next = encrypter(&[1u8].repeat(i + 1));
        if next[0..i] == last[0..i] {
            dbg!(&next, &last);
            break;
        } else {
            i += 1;
        }

        last = next;
    }

    i
}

pub fn deduce_next_byte<F>(mut encrypter: F, blocksize: usize, known_bytes: &[u8]) -> Option<u8>
where
    F: std::ops::FnMut(&[u8]) -> Vec<u8>,
{
    let known_blocks = known_bytes.len() / blocksize;
    let remainder = known_bytes.len() % blocksize;

    let mut dictionary = HashMap::new();

    let start = known_blocks * blocksize;
    for i in 0u8..=255u8 {
        let mut plaintext = [0u8].repeat(blocksize - remainder - 1);
        plaintext.append(&mut known_bytes.to_vec());
        plaintext.push(i);

        let encrypted = encrypter(&plaintext);
        dictionary.insert(encrypted[start..start + blocksize].to_vec(), i);
    }

    let probe = [0u8].repeat(blocksize - remainder - 1);

    let encrypted = encrypter(&probe);
    Some(*dictionary.get(&encrypted[start..start + blocksize])? as u8)
}

pub fn crack_secret() -> Vec<u8> {
    let blocksize = deduce_block_size(aes_128_ecb_with_secret);
    assert!(is_likely_encrypted_with_ecb(&[0; 64]));

    let mut result = vec![];
    while let Some(next) = deduce_next_byte(aes_128_ecb_with_secret, blocksize, &result) {
        result.push(next);
    }

    result.pop();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_example() {
        assign_key();

        let secret = crack_secret();
        assert_eq!(secret, get_secret());
    }
}
