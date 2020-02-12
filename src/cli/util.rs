use crate::crypto::Secp256k1PrivKey;
use std::str;
use std::str::FromStr;
use types::{H256, H512};

pub fn is_hex(hex: &str) -> Result<(), String> {
    let tmp = hex.as_bytes();
    if tmp.len() < 2 {
        Err("Must be a hexadecimal string".to_string())
    } else if tmp[..2] == b"0x"[..] || tmp[..2] == b"0X"[..] {
        Ok(())
    } else {
        Err("Must hex string".to_string())
    }
}

pub fn key_validator(hash: &str) -> Result<(), String> {
    is_hex(hash)?;
    if hash.len() > 66 {
        h512_validator(hash)
    } else {
        h256_validator(hash)
    }
}

pub fn h256_validator(value: &str) -> Result<(), String> {
    is_hex(value)?;
    H256::from_str(remove_0x(value))
        .map(|_| ())
        .map_err(|err| format!("{}", err))
}

pub fn h512_validator(value: &str) -> Result<(), String> {
    is_hex(value)?;
    H512::from_str(remove_0x(value))
        .map(|_| ())
        .map_err(|err| format!("{}", err))
}

/// Attempt to resolve the private key
pub fn parse_privkey(hash: &str) -> Result<Secp256k1PrivKey, String> {
    is_hex(hash)?;
    Ok(Secp256k1PrivKey::from_str(remove_0x(hash)).map_err(|err| format!("{}", err))?)
}

#[inline]
pub fn remove_0x(hex: &str) -> &str {
    if hex.len() >= 2 {
        let tmp = hex.as_bytes();
        if tmp[..2] == b"0x"[..] || tmp[..2] == b"0X"[..] {
            return str::from_utf8(&tmp[2..]).unwrap();
        }
    }
    hex
}
