use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TxInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TxInput>,
    vout: Vec<TxOutput>,
}

impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }    
}