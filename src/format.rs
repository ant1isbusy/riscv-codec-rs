use crate::util::*;
use bitfield::bitfield;
use colored::*;

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

fn format_r_type(d: &EncodedInstruction, r: &RType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].yellow(),
        operands[2].blue(),
    );
    let abi_instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].yellow(),
        abi_operands[2].blue(),
    );
    let fields = [
        format!("{:07b}", r.funct7()).red().to_string(),
        format!("{:05b}", r.rs2()).blue().to_string(),
        format!("{:05b}", r.rs1()).yellow().to_string(),
        format!("{:03b}", r.funct3()).red().to_string(),
        format!("{:05b}", r.rd()).green().to_string(),
        format!("{:07b}", r.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", r.0).bold().to_string();

    return (instr, abi_instr, bits, hex);
}

fn format_i_type(d: &EncodedInstruction, i: &IType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let hex = format!("0x{:08x}", i.0).bold().to_string();
    let instr;
    let abi_instr;
    if ["lb", "lh", "lw", "lbu", "lhu", "jalr"].contains(&d.mnemonic.as_str()) {
        instr = format!(
            "{} {}, {}({})",
            d.mnemonic.red().bold(),
            operands[0].green(),
            operands[1].blue(),
            operands[2].yellow(),
        );
        abi_instr = format!(
            "{} {}, {}({})",
            d.mnemonic.red().bold(),
            abi_operands[0].green(),
            abi_operands[1].blue(),
            abi_operands[2].yellow(),
        );
    } else {
        instr = format!(
            "{} {}, {}, {}",
            d.mnemonic.red().bold(),
            operands[0].green(),
            operands[1].yellow(),
            operands[2].blue(),
        );
        abi_instr = format!(
            "{} {}, {}, {}",
            d.mnemonic.red().bold(),
            abi_operands[0].green(),
            abi_operands[1].yellow(),
            abi_operands[2].blue(),
        );
    }
    let fields = [
        format!("{:012b}", i.imm()).blue().to_string(),
        format!("{:05b}", i.rs1()).yellow().to_string(),
        format!("{:03b}", i.funct3()).red().to_string(),
        format!("{:05b}", i.rd()).green().to_string(),
        format!("{:07b}", i.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    return (instr, abi_instr, bits, hex);
}

fn format_s_type(d: &EncodedInstruction, s: &SType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let instr = format!(
        "{} {}, {}({})",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
        operands[2].yellow(),
    );
    let abi_instr = format!(
        "{} {}, {}({})",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].blue(),
        abi_operands[2].yellow(),
    );
    let fields = [
        format!("{:07b}", s.imm11_5()).blue().to_string(),
        format!("{:05b}", s.rs2()).green().to_string(),
        format!("{:05b}", s.rs1()).yellow().to_string(),
        format!("{:03b}", s.funct3()).red().to_string(),
        format!("{:05b}", s.imm4_0()).blue().to_string(),
        format!("{:07b}", s.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", s.0).bold().to_string();
    return (instr, abi_instr, bits, hex);
}

fn format_b_type(d: &EncodedInstruction, b: &BType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].yellow(),
        operands[2].blue(),
    );
    let abi_instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].yellow(),
        abi_operands[2].blue(),
    );
    let fields = [
        format!("{:01b}", b.imm12() as u32).blue().to_string(),
        format!("{:06b}", b.imm10_5() as u32).blue().to_string(),
        format!("{:05b}", b.rs2()).yellow().to_string(),
        format!("{:05b}", b.rs1()).green().to_string(),
        format!("{:03b}", b.funct3()).red().to_string(),
        format!("{:04b}", b.imm4_1() as u32).blue().to_string(),
        format!("{:01b}", b.imm11() as u32).blue().to_string(),
        format!("{:07b}", b.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", b.0).bold().to_string();
    return (instr, abi_instr, bits, hex);
}

fn format_u_type(d: &EncodedInstruction, u: &UType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
    );
    let abi_instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].blue(),
    );
    let fields = [
        format!("{:020b}", u.imm()).blue().to_string(),
        format!("{:05b}", u.rd()).green().to_string(),
        format!("{:07b}", u.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", u.0).bold().to_string();
    return (instr, abi_instr, bits, hex);
}

fn format_j_type(d: &EncodedInstruction, j: &JType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
    );
    let abi_instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].blue(),
    );
    let fields = [
        format!("{:01b}", j.imm20() as u32).blue().to_string(),
        format!("{:010b}", j.imm10_1() as u32).blue().to_string(),
        format!("{:01b}", j.imm11() as u32).blue().to_string(),
        format!("{:008b}", j.imm19_12() as u32).blue().to_string(),
        format!("{:05b}", j.rd()).green().to_string(),
        format!("{:07b}", j.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", j.0).bold().to_string();
    return (instr, abi_instr, bits, hex);
}

fn format_csr_type(d: &EncodedInstruction, c: &CSRType) -> (String, String, String, String) {
    let operands = &d.operands;
    let abi_operands = operands_to_abi(operands);
    let hex = format!("0x{:08x}", c.0).bold().to_string();
    let instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
        operands[2].yellow(),
    );
    let abi_instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        abi_operands[0].green(),
        abi_operands[1].blue(),
        abi_operands[2].yellow(),
    );
    let fields = [
        format!("{:012b}", c.csr()).blue().to_string(),
        format!("{:05b}", c.rs1()).yellow().to_string(),
        format!("{:03b}", c.funct3()).red().to_string(),
        format!("{:05b}", c.rd()).green().to_string(),
        format!("{:07b}", c.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    return (instr, abi_instr, bits, hex);
}

pub fn print_encoded_instruction(d: &EncodedInstruction) {
    let out = match &d.instr {
        Instruction::RType(r) => format_r_type(d, r),
        Instruction::IType(i) => format_i_type(d, i),
        Instruction::SType(s) => format_s_type(d, s),
        Instruction::BType(b) => format_b_type(d, b),
        Instruction::UType(u) => format_u_type(d, u),
        Instruction::JType(j) => format_j_type(d, j),
        Instruction::CSRType(c) => format_csr_type(d, c),
    };

    println!("ASM: {}", out.0);
    println!("ABI: {}", out.1);
    println!("BIN: {}", out.2);
    println!("HEX: {}\n", out.3);
}
