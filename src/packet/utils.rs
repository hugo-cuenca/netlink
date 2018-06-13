use byteorder::{ByteOrder, NativeEndian};
use {Error, Result};

pub fn parse_ipv6(payload: &[u8]) -> Result<[u8; 16]> {
    if payload.len() != 16 {
        return Err(Error::MalformedNlaValue);
    }
    let mut address: [u8; 16] = [0; 16];
    for (i, byte) in payload.into_iter().enumerate() {
        address[i] = *byte;
    }
    Ok(address)
}

pub fn parse_string(payload: &[u8]) -> Result<String> {
    if payload.is_empty() {
        return Ok(String::new());
    }
    let s = String::from_utf8(payload[..payload.len() - 1].to_vec())
        .map_err(|_| Error::MalformedNlaValue)?;
    Ok(s)
}

pub fn parse_u8(payload: &[u8]) -> Result<u8> {
    if payload.len() != 1 {
        return Err(Error::MalformedNlaValue);
    }
    Ok(payload[0])
}

pub fn parse_u32(payload: &[u8]) -> Result<u32> {
    if payload.len() != 4 {
        return Err(Error::MalformedNlaValue);
    }
    Ok(NativeEndian::read_u32(payload))
}

pub fn parse_i32(payload: &[u8]) -> Result<i32> {
    if payload.len() != 4 {
        return Err(Error::MalformedNlaValue);
    }
    Ok(NativeEndian::read_i32(payload))
}