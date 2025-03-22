use serde::{de, ser};
use std::fmt::Display;
use thiserror::Error;

pub type CordResult<T, E = CordError> = std::result::Result<T, E>;

#[derive(Debug, Error, PartialEq)]
pub enum CordError {
    #[error("Cord encountered I/O error: {0}")]
    IOError(String),
    #[error("Cord does not support: {0}")]
    NotSupported(&'static str),
    #[error("Cord could not validate: {0}")]
    ValidationError(&'static str),
    #[error("Cord serialization error: {0}")]
    SerializationError(String),
    #[error("Cord deserialization error: {0}")]
    DeserializationError(String),
}

impl From<std::io::Error> for CordError {
    fn from(err: std::io::Error) -> Self {
        CordError::IOError(err.to_string())
    }
}

impl ser::Error for CordError {
    fn custom<T: Display>(msg: T) -> Self {
        CordError::SerializationError(msg.to_string())
    }
}

impl de::Error for CordError {
    fn custom<T: Display>(msg: T) -> Self {
        CordError::DeserializationError(msg.to_string())
    }
}
