use std::collections::HashMap;

use clap::{Error, Parser, Subcommand, Args};
use chrono::{Duration, DateTime, Utc, Local, TimeZone};
use serde::{Deserialize, Serialize};


use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use ring::aead::AES_256_GCM;
use ring::aead::UnboundKey;
use ring::aead::BoundKey;
use ring::aead::SealingKey;
use ring::aead::OpeningKey;
use ring::aead::Aad;
use ring::aead::NonceSequence;
use ring::aead::NONCE_LEN;
use ring::aead::Nonce;


use hex;
use std::str;

// Always returns the same nonce value for simplicity, don't use for more than 1 sealing operation!
struct SingleNonceSequence([u8; NONCE_LEN]);

impl NonceSequence for SingleNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        Nonce::try_assume_unique_for_key(&self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Encryptor {
    algorithm: String,
    secret_key: Vec<u8>,
    aad: Vec<u8>
}

impl Encryptor {

    pub fn new(algorithm: String, secret_key: String) -> Encryptor {
        let key = secret_key.as_bytes();
        let aad = "ora".as_bytes().to_vec();

        let mut key_bytes = vec![0; AES_256_GCM.key_len()];

        if key.len() > key_bytes.len() {
            panic!("Key too long");
        }

        key_bytes[..key.len()].copy_from_slice(key);

        Encryptor {
            aad,
            algorithm,
            secret_key: key_bytes,
        }
    }

    pub fn encrypt(self, data: String) -> String {
        // "encrypted".to_string()
        let content = data.as_bytes().to_vec();
        let aad = Aad::from(self.aad.clone());
        let mut in_out = content.clone();
        let sealing_key = &mut self.sealing_key();
        let tag = sealing_key.seal_in_place_separate_tag(aad, &mut in_out).unwrap();
        hex::encode([&in_out, tag.as_ref()].concat())
    }

    pub fn decrypt(&self, data: String) -> String {
        let plaintext = hex::decode(data).unwrap();
        let aad = Aad::from(self.aad.clone());
        let mut in_out = plaintext.clone();
        let opening_key = &mut self.opening_key();
        let decrypted = opening_key.open_in_place(aad, &mut in_out).unwrap();
        String::from_utf8(decrypted.to_vec()).unwrap()
    }

    fn sealing_key(&self) -> SealingKey<SingleNonceSequence> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.secret_key).unwrap();
        let nonce_sequence = SingleNonceSequence([0; NONCE_LEN]);
        SealingKey::new(unbound_key, nonce_sequence)
    }

    fn opening_key(&self) -> OpeningKey<SingleNonceSequence> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.secret_key).unwrap();
        let nonce_sequence = SingleNonceSequence([0; NONCE_LEN]);
        OpeningKey::new(unbound_key, nonce_sequence)
    }

    // fn hash(&self, data: String) -> String {}

}