use bip39::{Language, Mnemonic};
use hex_literal::hex;
use sha3::{Digest, Sha3_256};
use std::error::Error;

mod format;

pub struct Config {
    pub key: Vec<u8>,
    pub iterations: u32,
    pub salt: [u8; 32],
}

impl Config {
    pub fn new(secret: String, iterations: String) -> Result<Config, Box<dyn Error>> {
        let key = secret.as_bytes().to_vec();

        let iterations = iterations
            .clone()
            .trim()
            .parse()
            .expect("iterations must be a number");

        // chosen by dice roll
        const SALT: [u8; 32] =
            hex!("afa2064c4afe76d976ddea35e79c04deac4c60d1dc372563bf43a3e93e47efcf");

        Ok(Config {
            key,
            iterations,
            salt: SALT,
        })
    }
}

fn hash(config: Config) -> [u8; 32] {
    let mut hasher = Sha3_256::new();

    let message_hex = &[&config.salt[..], &config.key[..]].concat()[..];
    hasher.update(message_hex);
    let mut result = hasher.finalize_reset();

    for _ in 1..config.iterations {
        hasher.update(result);
        result = hasher.finalize_reset();
    }

    result.into()
}

fn hash_to_mnemonic(entropy: &[u8]) -> String {
    Mnemonic::from_entropy_in(Language::English, &entropy)
        .unwrap()
        .to_string()
}

pub fn derive(config: Config) -> Result<String, Box<dyn Error>> {
    let hash = hash(config);
    let mnemonic = hash_to_mnemonic(&hash);

    Ok(mnemonic)
}

#[cfg(test)]
mod tests {
    use crate::format::HexSlice;
    use crate::*;
    // test vectors from using `message_hex` as "Input type: hex" https://emn178.github.io/online-tools/sha3_256.html

    #[test]
    fn it_works_with_1_iteration() {
        let config = Config::new("secret123".into(), "1".into()).unwrap();

        // let message_hex = [&config.salt[..], &config.key[..]].concat();
        // println!("message_hex: {:x}", HexSlice::new(&message_hex));

        let expected_hash =
            hex!("69668080f55b44c865d8d645926e844cf22b5ecc43a5f64c29ed69a5a89ad744");
        let h = hash(config);

        assert_eq!(h, expected_hash);
    }

    #[test]
    fn it_works_with_2_iterations() {
        let config = Config::new("secret123".into(), "2".into()).unwrap();

        // let message_hex = hex!("69668080f55b44c865d8d645926e844cf22b5ecc43a5f64c29ed69a5a89ad744");
        // println!("message_hex: {:x}", HexSlice::new(&message_hex));

        let expected_hash =
            hex!("3f7246827d8bb72a8366537692ee4c78a8db66f930d1bef4f496a4314d99e25e");
        let h = hash(config);

        assert_eq!(h, expected_hash);
    }

    #[test]
    fn it_matches_entropy_to_correct_bip39() {
        let entropy = hex!("3f7246827d8bb72a8366537692ee4c78a8db66f930d1bef4f496a4314d99e25e");

        // test vector from enabling "Show entropy details" and pasting the entropy hex string https://iancoleman.io/bip39/
        let expected_bip = "disorder mutual party wild rocket next assault skill isolate number narrow vanish misery recall tooth boss same police certain embody below smooth maximum sheriff";
        let bip = hash_to_mnemonic(&entropy);

        assert_eq!(bip, expected_bip);
    }

    #[test]
    fn it_matches_config_to_correct_bip39() {
        let config = Config::new("secret123".into(), "2".into()).unwrap();

        // test vector from enabling "Show entropy details" and pasting the entropy hex string https://iancoleman.io/bip39/
        let expected_bip = "disorder mutual party wild rocket next assault skill isolate number narrow vanish misery recall tooth boss same police certain embody below smooth maximum sheriff";
        let bip = derive(config).unwrap();

        assert_eq!(bip, expected_bip);
    }
}
