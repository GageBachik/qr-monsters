use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

use crate::Monster;

const PREFIX: &str = "QRM1:";

pub fn encode_share(mon: &Monster) -> Result<String, String> {
    mon.validate()?;
    let bytes = bincode::serialize(mon).map_err(|e| e.to_string())?;
    Ok(format!("{PREFIX}{}", URL_SAFE_NO_PAD.encode(bytes)))
}

pub fn decode_share(s: &str) -> Result<Monster, String> {
    let body = s.strip_prefix(PREFIX).ok_or("missing QRM1 prefix")?;
    let bytes = URL_SAFE_NO_PAD.decode(body).map_err(|e| e.to_string())?;
    let mon: Monster = bincode::deserialize(&bytes).map_err(|e| e.to_string())?;
    mon.validate()?;
    Ok(mon)
}
