use crate::address::{Address, AddressError};
use crate::format::Format;
use crate::no_std::*;
use core::{
    fmt::{Debug, Display},
    str::FromStr,
};

/// Generic public key.
pub trait PublicKey: Clone + Debug + Display + FromStr + Send + Sync + 'static + Sized {
    type SecretKey;
    type Address: Address;
    type Format: Format;

    /// Returns a public key given a secp256k1 secret key.
    fn from_secret_key(secret_key: &Self::SecretKey) -> Self;

    /// Returns an address corresponding to this public key.
    fn to_address(&self, format: &Self::Format) -> Result<Self::Address, AddressError>;
}

#[derive(Debug, Error)]
pub enum PublicKeyError {
    #[error("{0}: {1}")]
    Crate(&'static str, String),

    #[error("invalid byte length: {0}")]
    InvalidByteLength(usize),

    #[error("invalid character length: {0}")]
    InvalidCharacterLength(usize),

    #[error("invalid public key prefix: {0}")]
    InvalidPrefix(String),

    #[error("no public spending key found")]
    NoSpendingKey,

    #[error("no public viewing key found")]
    NoViewingKey,
}

impl From<io::Error> for PublicKeyError {
    fn from(error: io::Error) -> Self {
        PublicKeyError::Crate("crate::no_std::io", format!("{:?}", error))
    }
}

impl From<base58::FromBase58Error> for PublicKeyError {
    fn from(error: base58::FromBase58Error) -> Self {
        PublicKeyError::Crate("base58", format!("{:?}", error))
    }
}

impl From<bech32::primitives::hrp::Error> for PublicKeyError {
    fn from(error: bech32::primitives::hrp::Error) -> Self {
        PublicKeyError::Crate("bech32", format!("{:?}", error))
    }
}

impl From<hex::FromHexError> for PublicKeyError {
    fn from(error: hex::FromHexError) -> Self {
        PublicKeyError::Crate("hex", format!("{:?}", error))
    }
}

impl From<libsecp256k1::Error> for PublicKeyError {
    fn from(error: libsecp256k1::Error) -> Self {
        PublicKeyError::Crate("libsecp256k1", format!("{:?}", error))
    }
}
