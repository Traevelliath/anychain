use crate::AddressError;
use crate::AmountError;
use crate::FormatError;
use crate::PublicKeyError;
use crate::TransactionError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Runtime Error:{0}")]
    RuntimeError(String),

    #[error("Invalid Address: {0}")]
    InvalidAddress(#[from] AddressError),

    #[error("Invalid Transaction: {0:}")]
    InvalidTransaction(#[from] TransactionError),

    #[error("Invalid Amount: {0:}")]
    InvalidAmount(#[from] AmountError),

    #[error("Invalid PublickKey: {0:}")]
    InvalidPublickKey(#[from] PublicKeyError),

    #[error("Invalid Format: {0:}")]
    InvalidFormat(#[from] FormatError),

    #[error("io error: {0:}")]
    Io(#[from] std::io::Error),

    #[error("fmt error: {0:}")]
    Fmt(#[from] std::fmt::Error),

    #[error("fromHex error: {0:}")]
    FromHex(#[from] hex::FromHexError),

    #[error("parsing error: {0:}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("secp265k1 error: {0:}")]
    Secp256k1Error(#[from] libsecp256k1::Error),
}
