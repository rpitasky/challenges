pub const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn btoa(hex: Vec<u8>) -> String {
    let alphabet = ALPHABET.to_vec();

    let mut result = Vec::with_capacity(4 * hex.len() / 3);
    hex.chunks(3).for_each(|c| {
        result.push(alphabet[(c[0] >> 2) as usize]);

        if c.len() == 3 {
            result.push(alphabet[(((c[0] & 0x03) << 4) | (c[1] >> 4)) as usize]);
            result.push(alphabet[(((c[1] & 0x0f) << 2) | (c[2] >> 6)) as usize]);
            result.push(alphabet[(c[2] & 0x3f) as usize]);
        } else {
            if c.len() == 2 {
                result.push(alphabet[(((c[0] & 0x03) << 4) | (c[1] >> 4)) as usize]);
                result.push(alphabet[((c[1] & 0x0f) << 2) as usize]);
            } else {
                result.push(alphabet[((c[0] & 0x03) << 4) as usize]);
                result.push(b'=');
            }

            result.push(b'=');
        }
    });

    // SAFETY: constructed from ASCII characters
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let input = byte_vec::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let expected =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();

        assert_eq!(btoa(input), expected);
    }

    #[test]
    fn other() {
        let input = "Cryptopals".bytes().collect::<Vec<_>>();
        let expected = "Q3J5cHRvcGFscw==".to_string();

        assert!(btoa(input) == expected);
    }
}
