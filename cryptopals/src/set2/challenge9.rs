pub fn pkcs_7_pad(block: &mut Vec<u8>, length: u8) {
    let difference = length - block.len() as u8;

    block.append(&mut ([difference].repeat(difference.into())).to_vec());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_example() {
        let mut block = b"YELLOW SUBMARINE".to_vec();
        pkcs_7_pad(&mut block, 20);

        assert_eq!(block, b"YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
