pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len());

    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let a = byte_vec::from("1c0111001f010100061a024b53535009181c").unwrap();
        let b = byte_vec::from("686974207468652062756c6c277320657965").unwrap();

        let result = byte_vec::from("746865206b696420646f6e277420706c6179").unwrap();

        assert_eq!(fixed_xor(&a, &b), result);
    }
}
