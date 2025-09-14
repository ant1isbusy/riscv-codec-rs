use crate::error::{Error, Result};
use bitfield::bitfield;

use crate::util;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
    CSRType(CSRType),
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct RType(u32);
    impl Debug;
    u32;
    pub funct7, set_funct7: 31, 25;
    pub rs2, set_rs2: 24, 20;
    pub rs1, set_rs1: 19, 15;
    pub funct3, set_funct3: 14, 12;
    pub rd, set_rd: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct IType(u32);
    impl Debug;
    pub imm, set_imm: 31, 20;
    pub rs1, set_rs1: 19, 15;
    pub funct3, set_funct3: 14, 12;
    pub rd, set_rd: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct SType(u32);
    impl Debug;
    pub imm11_5, set_imm11_5: 31, 25;
    pub rs2, set_rs2: 24, 20;
    pub rs1, set_rs1: 19, 15;
    pub funct3, set_funct3: 14, 12;
    pub imm4_0, set_imm4_0: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct BType(u32);
    impl Debug;
    pub imm12, set_imm12: 31;
    pub imm10_5, set_imm10_5: 30, 25;
    pub rs2, set_rs2: 24, 20;
    pub rs1, set_rs1: 19, 15;
    pub funct3, set_funct3: 14, 12;
    pub imm4_1, set_imm4_1: 11, 8;
    pub imm11, set_imm11: 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct UType(u32);
    impl Debug;
    pub imm, set_imm: 31, 12;
    pub rd, set_rd: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct JType(u32);
    impl Debug;
    pub imm20, set_imm20: 31;
    pub imm10_1, set_imm10_1: 30, 21;
    pub imm11, set_imm11: 20;
    pub imm19_12, set_imm19_12: 19, 12;
    pub rd, set_rd: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct CSRType(u32);
    impl Debug;
    pub csr, set_csr: 31, 20;
    pub rs1, set_rs1: 19, 15;
    pub funct3, set_funct3: 14, 12;
    pub rd, set_rd: 11, 7;
    pub opcode, set_opcode: 6, 0;
}

pub struct EncodedInstruction {
    pub instr: Instruction,
    pub mnemonic: String,
    pub operands: Vec<String>,
}

pub fn encode(instr_string: &str) -> Result<EncodedInstruction> {
    let tokens: Vec<&str> = instr_string
        .split(|c| c == ' ' || c == ',' || c == '(' || c == ')')
        .filter(|s| !s.is_empty())
        .collect();

    if tokens.is_empty() {
        return Err(Error::InvalidFormat);
    }

    let mnemonic = tokens[0].to_lowercase();
    let operands = tokens[1..]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let instr = match mnemonic.as_str() {
        "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or" | "and" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(&operands[0])?;
            let rs1 = util::parse_reg(&operands[1])?;
            let rs2 = util::parse_reg(&operands[2])?;
            let (funct3, funct7) = match mnemonic.as_str() {
                "add" => (0x0, 0x00),
                "sub" => (0x0, 0x20),
                "xor" => (0x4, 0x00),
                "or" => (0x6, 0x00),
                "and" => (0x7, 0x00),
                "sll" => (0x1, 0x00),
                "srl" => (0x5, 0x00),
                "sra" => (0x5, 0x20),
                "slt" => (0x2, 0x00),
                "sltu" => (0x3, 0x00),
                _ => unreachable!(),
            };
            let opcode = 0b0110011;

            let mut r = RType(0);
            r.set_funct7(funct7);
            r.set_rs2(rs2);
            r.set_rs1(rs1);
            r.set_funct3(funct3);
            r.set_rd(rd);
            r.set_opcode(opcode);

            Instruction::RType(r)
        }

        "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" | "lb"
        | "lh" | "lw" | "lbu" | "lhu" | "jalr" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(&operands[0])?;
            let rs1: u32;
            let mut imm: i32;
            if ["lb", "lh", "lw", "lbu", "lhu", "jalr"].contains(&mnemonic.as_str()) {
                imm = util::parse_immediate(&operands[1])?;
                rs1 = util::parse_reg(&operands[2])?;
            } else {
                imm = util::parse_immediate(&operands[2])?;
                rs1 = util::parse_reg(&operands[1])?;
            }

            // uimm variant dont check range
            if ["sltiu", "lbu", "lhu"].contains(&mnemonic.as_str()) {
                if imm < 0 || imm > 4095 {
                    return Err(Error::ImmediateOutOfRange);
                }
            } else {
                if imm < -2048 || imm > 2047 {
                    return Err(Error::ImmediateOutOfRange);
                }
            }
            let funct3 = match mnemonic.as_str() {
                "addi" => 0x0,
                "xori" => 0x4,
                "ori" => 0x6,
                "andi" => 0x7,
                "slli" => 0x1,
                "srli" => 0x5,
                "srai" => 0x5,
                "slti" => 0x2,
                "sltiu" => 0x3,
                "lb" => 0x0,
                "lh" => 0x1,
                "lw" => 0x2,
                "lbu" => 0x4,
                "lhu" => 0x5,
                "jalr" => 0x0,
                _ => unreachable!(),
            };
            if mnemonic == "slli" || mnemonic == "srli" || mnemonic == "srai" {
                imm = imm & 0x1f; // shamt
            }
            let opcode = if ["lb", "lh", "lw", "lbu", "lhu"].contains(&mnemonic.as_str()) {
                0b0000011
            } else if mnemonic == "jalr" {
                0b1100111
            } else {
                0b0010011
            };
            let mut i = IType(0);
            i.set_imm(imm as u32);
            i.set_rs1(rs1);
            i.set_funct3(funct3);
            i.set_rd(rd);
            i.set_opcode(opcode);

            Instruction::IType(i)
        }

        "sb" | "sh" | "sw" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rs2 = util::parse_reg(&operands[0])?;
            let imm = util::parse_immediate(&operands[1])?;
            let rs1 = util::parse_reg(&operands[2])?;

            if imm < -2048 || imm > 2047 {
                return Err(Error::ImmediateOutOfRange);
            }

            let funct3 = match mnemonic.as_str() {
                "sb" => 0x0,
                "sh" => 0x1,
                "sw" => 0x2,
                _ => unreachable!(),
            };
            let opcode = 0b0100011;

            let mut s = SType(0);
            s.set_imm4_0((imm as u32) & 0x1f);
            s.set_imm11_5(((imm as u32) >> 5) & 0x7f);
            s.set_rs2(rs2);
            s.set_rs1(rs1);
            s.set_funct3(funct3);
            s.set_opcode(opcode);

            Instruction::SType(s)
        }
        "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rs1 = util::parse_reg(&operands[0])?;
            let rs2 = util::parse_reg(&operands[1])?;
            let imm = util::parse_immediate(&operands[2])?;

            if imm < -4096 || imm > 4094 {
                return Err(Error::ImmediateOutOfRange);
            }
            if imm % 2 != 0 {
                return Err(Error::ImmediateMisaligned);
            }

            let funct3 = match mnemonic.as_str() {
                "beq" => 0x0,
                "bne" => 0x1,
                "blt" => 0x4,
                "bge" => 0x5,
                "bltu" => 0x6,
                "bgeu" => 0x7,
                _ => unreachable!(),
            };
            let opcode = 0b1100011;

            let mut b = BType(0);
            b.set_imm11(((imm as u32) >> 11) & 0x1 != 0);
            b.set_imm4_1(((imm as u32) >> 1) & 0xf);
            b.set_imm10_5(((imm as u32) >> 5) & 0x3f);
            b.set_imm12(((imm as u32) >> 12) & 0x1 != 0);
            b.set_rs2(rs2);
            b.set_rs1(rs1);
            b.set_funct3(funct3);
            b.set_opcode(opcode);

            Instruction::BType(b)
        }
        "lui" | "auipc" => {
            if operands.len() != 2 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(&operands[0])?;
            let imm = util::parse_immediate(&operands[1])?;
            if imm < -524288 || imm > 524287 {
                return Err(Error::ImmediateOutOfRange);
            }
            let opcode = if mnemonic == "lui" {
                0b0110111
            } else {
                0b0010111
            };

            let mut u = UType(0);
            u.set_imm(imm as u32);
            u.set_rd(rd);
            u.set_opcode(opcode);
            Instruction::UType(u)
        }
        "jal" => {
            if operands.len() != 2 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(&operands[0])?;
            let imm = util::parse_immediate(&operands[1])?;
            if imm < -1048576 || imm > 1048574 {
                return Err(Error::ImmediateOutOfRange);
            }
            if imm % 2 != 0 {
                return Err(Error::ImmediateMisaligned);
            }
            let opcode = 0b1101111;
            let mut j = JType(0);
            j.set_rd(rd);
            j.set_imm20(((imm as u32) >> 20) & 0x1 != 0);
            j.set_imm19_12(((imm as u32) >> 12) & 0xff);
            j.set_imm11(((imm as u32) >> 11) & 0x1 != 0);
            j.set_imm10_1(((imm as u32) >> 1) & 0x3ff);
            j.set_opcode(opcode);
            Instruction::JType(j)
        }
        "csrrw" | "csrrs" | "csrrc" | "csrrwi" | "csrrsi" | "csrrci" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(&operands[0])?;
            let csr = util::parse_immediate(&operands[1])? as u32;
            if csr > 0xfff {
                return Err(Error::ImmediateOutOfRange);
            }
            let rs1: u32;
            let is_imm: bool;
            if ["csrrwi", "csrrsi", "csrrci"].contains(&mnemonic.as_str()) {
                rs1 = util::parse_immediate(&operands[2])? as u32;
                is_imm = true;
            } else {
                rs1 = util::parse_reg(&operands[2])?;
                is_imm = false;
            }
            if is_imm && (rs1 > 31) {
                return Err(Error::ImmediateOutOfRange);
            }
            let funct3 = match mnemonic.as_str() {
                "csrrw" => 0x1,
                "csrrs" => 0x2,
                "csrrc" => 0x3,
                "csrrwi" => 0x5,
                "csrrsi" => 0x6,
                "csrrci" => 0x7,
                _ => unreachable!(),
            };
            let opcode = 0b1110011;

            let mut c = CSRType(0);
            c.set_csr(csr);
            c.set_rs1(rs1);
            c.set_funct3(funct3);
            c.set_rd(rd);
            c.set_opcode(opcode);

            Instruction::CSRType(c)
        }
        _ => return Err(Error::UnknownInstruction),
    };

    Ok(EncodedInstruction {
        instr,
        mnemonic,
        operands,
    })
}
