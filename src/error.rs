use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum BlockchainError {
    DatabaseError(String),
    DeserializationError(String),
    NetworkError(String),
    WalletError(String),
    InvalidTransaction(String),
    NotFoundError(String),
    ValidationError(String),
    // Add more error types as needed
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockchainError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            BlockchainError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            BlockchainError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BlockchainError::WalletError(msg) => write!(f, "Wallet error: {}", msg),
            BlockchainError::InvalidTransaction(msg) => write!(f, "Invalid transaction error: {}", msg),
            BlockchainError::NotFoundError(msg) => write!(f, "Not found error: {}", msg),
            BlockchainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for BlockchainError {}

impl From<sled::Error> for BlockchainError {
    fn from(err: sled::Error) -> Self {
        BlockchainError::DatabaseError(err.to_string())
    }
} 
