use crate::error::{Error, Result};

const ABI_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

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

fn numeric_to_abi(op: &str) -> String {
    if let Some(stripped) = op.strip_prefix('x') {
        if let Ok(idx) = stripped.parse::<usize>() {
            if idx < ABI_NAMES.len() {
                return ABI_NAMES[idx].to_string();
            }
        }
    }
    op.to_string()
}

pub fn operands_to_abi(ops: &[String]) -> Vec<String> {
    ops.iter().map(|o| numeric_to_abi(o)).collect()
}

fn abi_to_numeric(abi: &str) -> String {
    if let Some(idx) = ABI_NAMES.iter().position(|&name| name == abi) {
        format!("x{}", idx)
    } else {
        abi.to_string()
    }
}

pub fn abis_to_operands(abis: &[String]) -> Vec<String> {
    abis.iter().map(|a| abi_to_numeric(a)).collect()
}
