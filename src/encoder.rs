use crate::error::{Error, Result};

/*
pub fn try_parse_u32(s: &str) -> Option<u32> {
    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(stripped, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
} */

pub fn encode(instr_str: &str) -> Result<u32> {
    let parts: Vec<&str> = instr_str
        .split_whitespace()
        .map(|s| s.trim_end_matches(','))
        .collect();
    if parts.is_empty() {
        return Err(Error::Other);
    } else {
        for part in &parts {
            println!("Part: {}", part);
        }
        return Ok(0);
    }
}
