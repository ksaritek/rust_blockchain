use std::iter::repeat;

use crypto::digest::Digest;
use ring::digest::{Context, SHA256};
use ring::rand::SystemRandom;
use ring::signature::EcdsaKeyPair;

pub fn new_key_pair() -> Result<Vec<u8>, ring::error::Unspecified> {
    let rng = SystemRandom::new();
    let pkcs = EcdsaKeyPair::generate_pkcs8(
        &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        &rng,
    )?;
    Ok(pkcs.as_ref().to_vec())
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8>{
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);
    return buf
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Result<Vec<u8>, bs58::decode::Error> {
    bs58::decode(data).into_vec()
}

