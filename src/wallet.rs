use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use serde::{Serialize, Deserialize};
use std::fmt;
use hex;

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}

#[derive(Debug)]
pub enum WalletError {
    KeyPairCreation(String),
    FileOperation(std::io::Error),
    Serialization(bincode::Error),
}

impl std::error::Error for WalletError {}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::KeyPairCreation(e) => write!(f, "Failed to create key pair: {}", e),
            WalletError::FileOperation(e) => write!(f, "File operation failed: {}", e),
            WalletError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<std::io::Error> for WalletError {
    fn from(error: std::io::Error) -> Self {
        WalletError::FileOperation(error)
    }
}

impl From<bincode::Error> for WalletError {
    fn from(error: bincode::Error) -> Self {
        WalletError::Serialization(error)
    }
}

impl Wallet {
    pub fn new() -> Result<Wallet, WalletError> {
        let pkcs8 = crate::new_key_pair()
            .map_err(|e| WalletError::KeyPairCreation(e.to_string()))?;
        let key_pair = EcdsaKeyPair::from_pkcs8(
            &ECDSA_P256_SHA256_FIXED_SIGNING,
            pkcs8.as_ref(),
        ).map_err(|e| WalletError::KeyPairCreation(format!("from_pkcs8 error: {:?}", e)))?;

        let public_key = key_pair.public_key().as_ref().to_vec();
        Ok(Wallet { pkcs8, public_key })
    }

    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload: Vec<u8> = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        // version + pub_key_hash + checksum
        crate::base58_encode(payload.as_slice())
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }
}

impl fmt::Debug for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wallet")
            .field("pkcs8", &hex::encode(&self.pkcs8))
            .field("public_key", &hex::encode(&self.public_key))
            .finish()
    }
}

pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = crate::sha256_digest(pub_key);
    crate::ripemd160_digest(pub_key_sha256.as_slice())
}

fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = crate::sha256_digest(payload);
    let second_sha = crate::sha256_digest(first_sha.as_slice());
    second_sha[0..ADDRESS_CHECK_SUM_LEN].to_vec()
}

pub fn validate_address(address: &str) -> bool {
    crate::base58_decode(address)
        .map(|payload| {
            let actual_checksum = &payload[payload.len() - ADDRESS_CHECK_SUM_LEN..];
            let version = payload[0];
            let pub_key_hash = &payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN];

            let mut target_vec = vec![version];
            target_vec.extend_from_slice(pub_key_hash);
            let target_checksum = checksum(&target_vec);
            actual_checksum == target_checksum.as_slice()
        })
        .unwrap_or(false)
}

pub fn convert_address(pub_hash_key: &[u8]) -> String {
    let mut payload: Vec<u8> = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);
    let checksum = checksum(payload.as_slice());
    payload.extend(checksum.as_slice());
    crate::base58_encode(payload.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new().unwrap();
        println!("{:?}", wallet);
        
        assert!(!wallet.get_public_key().is_empty());
        assert!(!wallet.get_pkcs8().is_empty());
    }

    #[test]
    fn test_get_address() {
        let wallet = Wallet::new().unwrap();
        let address = wallet.get_address();
        assert!(validate_address(&address));
    }

    #[test]
    fn test_validate_address() {
        let wallet = Wallet::new().unwrap();
        let address = wallet.get_address();
        assert!(validate_address(&address));
        assert!(!validate_address("invalid_address"));
    }

    #[test]
    fn test_convert_address() {
        let wallet = Wallet::new().unwrap();
        let pub_key_hash = hash_pub_key(wallet.get_public_key());
        let address = convert_address(&pub_key_hash);
        assert!(validate_address(&address));
    }
}
