use crate::core::columns_iterator::Columns;
use std::ops::Range;

use super::challenge3::crack_single_byte_xor;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum()
}

pub fn guess_vigenere_keysize(ciphertext: &[u8], guesses: Range<usize>) -> usize {
    guesses
        .map(|key_size| {
            // sample a bunch of chunks for more accuracy
            let mut chunks = ciphertext.chunks(key_size).collect::<Vec<_>>();
            chunks.truncate(16);

            let score: u32 = chunks
                .as_slice()
                .windows(2)
                .map(|window| hamming_distance(window[0], window[1]))
                .sum();

            (
                (score as f32 / chunks.len() as f32) / key_size as f32,
                key_size,
            )
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
        .1
}

pub fn crack_vigenere(ciphertext: &[u8], keysize_guesses: Range<usize>) -> Vec<u8> {
    let keysize = guess_vigenere_keysize(ciphertext, keysize_guesses);

    let cracked_columns = Columns::from(
        ciphertext
            .chunks(keysize)
            .map(|chunk| chunk.iter().cloned())
            .collect(),
    )
    .map(|column| crack_single_byte_xor(&column).1)
    .collect::<Vec<_>>();

    Columns::from(
        cracked_columns
            .iter()
            .map(|chunk| chunk.iter().cloned())
            .collect(),
    )
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::byte_vec;

    use crate::set1::challenge1::btoa;

    #[test]
    fn hamming_distance() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";

        assert_eq!(super::hamming_distance(a, b), 37);
    }

    #[test]
    fn guess_vigenere_keysize() {
        let ciphertext = byte_vec::from(include_str!("data/challenge6.txt"))
            .ok()
            .unwrap();

        assert_eq!(super::guess_vigenere_keysize(&ciphertext, 2..40), 29);
    }

    #[test]
    fn provided_example() {
        let ciphertext = byte_vec::from(include_str!("data/challenge6.txt")).unwrap();
        let plaintext = include_str!("data/challenge6_solution.txt");

        assert_eq!(
            plaintext.to_string(),
            btoa(crack_vigenere(&ciphertext, 2..40))
        );
    }
}
