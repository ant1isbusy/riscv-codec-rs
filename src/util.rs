use crate::error::{Error, Result};

pub fn is_hex(s: &str) -> bool {
    s.starts_with("0x") || s.starts_with("0X")
}

pub fn parse_immediate(s: &str) -> Result<i32> {
    let s = s.trim();
    if is_hex(s) {
        i32::from_str_radix(&s[2..], 16).map_err(|_| Error::InvalidImmediate)
    } else {
        s.parse::<i32>().map_err(|_| Error::InvalidImmediate)
    }
}

pub fn parse_reg(reg: &str) -> Result<u32> {
    if let Some(stripped) = reg.strip_prefix('x') {
        let num = stripped
            .parse::<u32>()
            .map_err(|_| Error::InvalidRegister)?;
        if num <= 31 {
            Ok(num)
        } else {
            Err(Error::InvalidRegister)
        }
    } else {
        Err(Error::InvalidRegister)
    }
}
