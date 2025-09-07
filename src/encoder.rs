use crate::error::{Error, Result};

pub fn try_parse_u32(s: &str) -> Option<u32> {
    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(stripped, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

pub fn parse_reg(reg_str: &str) -> Result<u32> {
    if reg_str.starts_with('x') {
        if let Ok(num) = reg_str[1..].parse::<u32>() {
            if num <= 31 {
                return Ok(num);
            }
        }
    }
    Err(Error::InvalidRegister)
}

pub fn encode(instr_str: &str) -> Result<u32> {
    let parts: Vec<&str> = instr_str
        .split_whitespace()
        .map(|s| s.trim_end_matches(','))
        .collect();
    if parts.is_empty() {
        return Err(Error::Other);
    } else {
        let mnemonic = parts[0].to_lowercase();

        match mnemonic.as_str() {
            "add" | "sub" | "and" | "or" | "xor" | "sll" | "srl" | "sra" | "slt" | "sltu" => {
                if parts.len() != 4 {
                    return Err(Error::InvalidFormat);
                }
                let rd = parse_reg(parts[1])?;
                let rs1 = parse_reg(parts[2])?;
                let rs2 = parse_reg(parts[3])?;
                let funct3 = match mnemonic.as_str() {
                    "add" | "sub" => 0x0,
                    "xor" => 0x4,
                    "or" => 0x6,
                    "and" => 0x7,
                    "sll" => 0x1,
                    "srl" | "sra" => 0x5,
                    "slt" => 0x2,
                    "sltu" => 0x3,
                    _ => unreachable!(),
                };
                let funct7 = if mnemonic == "sub" || mnemonic == "sra" {
                    0x20
                } else {
                    0x00
                };
                let opcode = 0b0110011;
                let instruction = (funct7 << 25)
                    | (rs2 << 20)
                    | (rs1 << 15)
                    | (funct3 << 12)
                    | (rd << 7)
                    | opcode;
                return Ok(instruction);
            }
            _ => return Err(Error::UnknownInstruction),
        }
    }
}
