use crate::wallet::{Wallet, WalletError};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};

pub const WALLET_FILE: &str = "wallet.dat";

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Result<Wallets, WalletError> {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file()?;
        Ok(wallets)
    }

    pub fn create_wallet(&mut self) -> Result<String, WalletError> {
        let wallet = Wallet::new()?;
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file()?;
        Ok(address)
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = vec![];
        for (address, _) in &self.wallets {
            addresses.push(address.clone())
        }
        return addresses;
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        if let Some(wallet) = self.wallets.get(address) {
            return Some(wallet);
        }
        None
    }

    fn load_from_file(&mut self) -> Result<(), WalletError> {
        let path = current_dir()?.join(WALLET_FILE);
        if !path.exists() {
            return Ok(());
        }
        let mut file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut buf = vec![0; metadata.len() as usize];
        file.read(&mut buf)?;
        let wallets: HashMap<String, Wallet> = bincode::deserialize(&buf[..]).map_err(WalletError::Serialization)?;
        self.wallets = wallets;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), WalletError> {
        let path = current_dir()?.join(WALLET_FILE);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)?;
        let mut writer = BufWriter::new(file);
        let wallets_bytes = bincode::serialize(&self.wallets)?;
        writer.write_all(&wallets_bytes)?;
        writer.flush()?;
        Ok(())
    }
}