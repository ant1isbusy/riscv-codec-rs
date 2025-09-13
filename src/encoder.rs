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
    pub imm20, set_imm20: 31;
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

pub fn encode(instr_string: &str) -> Result<Instruction> {
    let tokens: Vec<&str> = instr_string
        .split(|c| c == ' ' || c == ',' || c == '(' || c == ')')
        .filter(|s| !s.is_empty())
        .collect();

    if tokens.is_empty() {
        return Err(Error::InvalidFormat);
    }

    println!("Tokens: {:?}", tokens);

    let mnemonic = tokens[0].to_lowercase();
    let operands = &tokens[1..];

    match mnemonic.as_str() {
        "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or" | "and" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(operands[0])?;
            let rs1 = util::parse_reg(operands[1])?;
            let rs2 = util::parse_reg(operands[2])?;
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

            Ok(Instruction::RType(r))
        }

        "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" | "lb"
        | "lh" | "lw" | "lbu" | "lhu" => {
            if operands.len() != 3 {
                return Err(Error::InvalidFormat);
            }
            let rd = util::parse_reg(operands[0])?;
            let rs1: u32;
            let mut imm: i32;
            if ["lb", "lh", "lw", "lbu", "lhu"].contains(&mnemonic.as_str()) {
                imm = util::parse_immediate(operands[1])?;
                rs1 = util::parse_reg(operands[2])?;
            } else {
                imm = util::parse_immediate(operands[2])?;
                rs1 = util::parse_reg(operands[1])?;
            }
            if imm < -2048 || imm > 2047 {
                return Err(Error::ImmediateOutOfRange);
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
                _ => unreachable!(),
            };
            if mnemonic == "slli" || mnemonic == "srli" || mnemonic == "srai" {
                imm = imm & 0x1f; // shamt
            }
            let opcode = if ["lb", "lh", "lw", "lbu", "lhu"].contains(&mnemonic.as_str()) {
                0b0000011
            } else {
                0b0010011
            };
            let mut i = IType(0);
            i.set_imm(imm as u32);
            i.set_rs1(rs1);
            i.set_funct3(funct3);
            i.set_rd(rd);
            i.set_opcode(opcode);

            Ok(Instruction::IType(i))
        }

        _ => Err(Error::UnknownInstruction),
    }
}
