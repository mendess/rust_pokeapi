use super::serde_json;
use super::reqwest;

use std::fmt;

#[derive(Debug)]
pub enum DexError {
    JsonError(serde_json::Error),
    ReqwestError(reqwest::Error),
    Other(String),
}

impl From<serde_json::Error> for DexError {
    fn from(error: serde_json::Error) -> Self {
        DexError::JsonError(error)
    }
}

impl From<reqwest::Error> for DexError {
    fn from(error: reqwest::Error) -> Self {
        DexError::ReqwestError(error)
    }
}

impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::dex_error::DexError::*;
        match self {
            JsonError(e) => write!(f, "JsonError({})", e),
            ReqwestError(e) => write!(f, "ReqwestError({})", e),
            Other(s) => write!(f, "Other({})", s),
        }
    }
}
