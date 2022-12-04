use super::challenge2::fixed_xor;

pub fn vigenere(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let key = key
        .iter()
        .cycle()
        .take(ciphertext.len())
        .cloned()
        .collect::<Vec<u8>>();

    fixed_xor(ciphertext, &key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let plaintext =
            b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";

        let expected = byte_vec::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();

        assert_eq!(vigenere(plaintext, key), expected);
    }
}
