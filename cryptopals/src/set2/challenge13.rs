use super::challenge9::pkcs_7_pad;

// Some implementation details:

// Rust's guarantees on strings mean that the attack I'm suppposed to perform would be caught almost
// immediately. Because of this, I have to operate on Vec<u8> everywhere, which is very unidiomatic.
// It's also worth noting that the attack is somewhat more difficult because HashMaps are unordered,
// meaning we don't get the exact same stringification results as the example. I define the ordering
// to be alphabetical by val, while the example orders key-value pairs by their order in the cookie.

pub fn aes_128_ecb_decrypt(key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Vec<u8> {
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    openssl::symm::decrypt(cipher, key, iv, data).unwrap()
}

macro_rules! to_bytes {
    ($str:literal) => {
        $str.to_string().into_bytes()
    };
}

pub mod cookies {
    use super::aes_128_ecb_decrypt;
    use crate::set2::{challenge10::aes_128_ecb_encrypt, challenge11::rand_aes_128_key};

    use std::collections::HashMap;

    #[derive(Clone, Copy)]
    enum State {
        Key,
        Value,
    }

    pub fn parse(cookie: &[u8]) -> HashMap<Vec<u8>, Vec<u8>> {
        let iterator = cookie.iter().peekable();

        let mut state = State::Key;

        let mut key = Vec::new();
        let mut value = Vec::new();

        let mut result = HashMap::new();

        for c in iterator {
            match (state, c) {
                (State::Key, b'=') => {
                    value = Vec::new();

                    state = State::Value;
                }

                (State::Value, b'&') => {
                    result.insert(key.clone(), value.clone());
                    key = Vec::new();
                    value = Vec::new();

                    state = State::Key;
                }

                (State::Key, b'&') | (State::Value, b'=') => panic!("Invalid character"),

                (State::Key, &c) => {
                    key.push(c);
                }

                (State::Value, &c) => {
                    value.push(c);
                }
            }
        }

        assert!(matches!(state, State::Value));

        result.insert(key.clone(), value.clone());

        result
    }

    pub fn stringify(map: &HashMap<Vec<u8>, Vec<u8>>) -> String {
        let mut result = Vec::new();

        let mut pairs: Vec<_> = map.iter().collect();
        pairs.sort_by_key(|x| x.1);

        pairs.iter().for_each(|(key, value)| {
            // "whatever you want to do" includes panicking ¯\_(ツ)_/¯
            assert!(![b'&', b'='].iter().any(|c| key.contains(c)));
            assert!(![b'&', b'='].iter().any(|c| value.contains(c)));

            if !result.is_empty() {
                result.push(b'&');
            }

            result.extend_from_slice(key);
            result.push(b'=');
            result.extend_from_slice(value);
        });

        String::from_utf8(result).unwrap()
    }

    pub struct Server {
        users: HashMap<Vec<u8>, HashMap<Vec<u8>, Vec<u8>>>,
        key: [u8; 16],
    }

    impl Server {
        pub fn new() -> Self {
            Self {
                users: HashMap::new(),
                key: rand_aes_128_key(),
            }
        }

        pub fn add_profile(&mut self, email: &[u8]) {
            let mut profile = HashMap::new();

            profile.insert(to_bytes!("email"), email.to_vec());
            profile.insert(
                to_bytes!("uid"),
                (self.users.len() + 1).to_string().into_bytes(),
            );
            profile.insert(to_bytes!("role"), to_bytes!("user"));
            self.users.insert(email.to_vec(), profile);
        }

        pub fn query_profile(&self, email: &[u8]) -> Vec<u8> {
            aes_128_ecb_encrypt(
                &self.key,
                None,
                stringify(self.users.get(&email.to_vec()).unwrap()).as_bytes(),
            )
        }

        pub fn modify_profile(&mut self, encrypted: &[u8]) {
            let decrypted = aes_128_ecb_decrypt(&self.key, None, encrypted);
            let user = parse(&decrypted);

            let email = user.get(&to_bytes!("email")).unwrap();

            self.users.insert(email.to_vec(), user);
        }

        pub fn is_admin(&self, email: &[u8]) -> bool {
            self.users
                .get(&email.to_vec())
                .unwrap()
                .get(&to_bytes!("role"))
                .unwrap()
                == b"admin"
        }
    }

    impl Default for Server {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub fn make_admin_user(server: &mut cookies::Server) -> Vec<u8> {
    let block_size: usize = 16; // todo: figure this out programmatically somehow?

    // fill up uids to length 3
    for i in 0..100 {
        server.add_profile(&[i]);
    }

    let length_to_next_block = block_size - ("uid=".len() + 3 + "&email=".len());

    let email_1 = b"a".repeat(length_to_next_block + block_size - "&role=".len());

    let mut admin_plaintext = to_bytes!("admin");
    pkcs_7_pad(&mut admin_plaintext, block_size as u8);

    let email_2 = [b"a".repeat(length_to_next_block), admin_plaintext].concat();

    server.add_profile(&email_1);
    server.add_profile(&email_2);

    let admin_encrypted = &server.query_profile(&email_2)[block_size..2 * block_size];

    let mut malicious_profile = server.query_profile(&email_1);
    let start = malicious_profile.len() - block_size;

    malicious_profile.splice(start..malicious_profile.len(), admin_encrypted.to_owned());

    server.modify_profile(&malicious_profile);

    email_1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cookie() {
        let parsed = cookies::parse(b"foo=bar&baz=qux&zap=zazzle");

        assert_eq!(*parsed.get(&to_bytes!("foo")).unwrap(), to_bytes!("bar"));
        assert_eq!(*parsed.get(&to_bytes!("baz")).unwrap(), to_bytes!("qux"));
        assert_eq!(*parsed.get(&to_bytes!("zap")).unwrap(), to_bytes!("zazzle"));
    }

    #[test]
    fn stringify_cookie() {
        let input = to_bytes!("uid=10&email=foo@bar.com&role=user");

        let parsed = cookies::parse(&input);

        let stringified = cookies::stringify(&parsed).into_bytes();
        let reparsed = cookies::parse(&stringified);

        assert_eq!(parsed, reparsed);
        assert_eq!(stringified, input);
    }

    #[test]
    fn add_profile() {
        let mut server = cookies::Server::new();
        server.add_profile(b"email@example.com");
    }

    #[test]
    fn query_profile() {
        let mut server = cookies::Server::new();
        server.add_profile(b"myprofile@example.com");

        server.query_profile(b"myprofile@example.com");
    }

    #[test]
    fn modify_profile() {
        let mut server = cookies::Server::new();
        server.add_profile(b"myprofile@example.com");

        let encrypted = server.query_profile(b"myprofile@example.com");
        server.modify_profile(&encrypted);

        assert_eq!(server.query_profile(b"myprofile@example.com"), encrypted);
    }

    #[test]
    fn make_admin_user() {
        let mut server = cookies::Server::new();

        let email = super::make_admin_user(&mut server);
        assert!(server.is_admin(&email));
    }
}
