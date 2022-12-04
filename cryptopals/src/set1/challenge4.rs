use super::challenge3::crack_single_byte_xor;

pub fn find_encoded(lines: Vec<Vec<u8>>) -> (f32, Vec<u8>) {
    lines
        .iter()
        .map(|line| crack_single_byte_xor(line))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::byte_vec;

    #[test]
    fn provided_example() {
        let data = include_str!("data/challenge4.txt")
            .lines()
            .map(|string| byte_vec::from(string).ok().unwrap())
            .collect::<Vec<Vec<u8>>>();
        let result = find_encoded(data);

        assert_eq!(
            Ok("Now that the party is jumping\n".to_string()),
            String::from_utf8(result.1)
        )
    }
}
