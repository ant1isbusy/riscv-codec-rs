use crate::encoder::*;
use colored::*;

fn format_r_type(d: &EncodedInstruction, r: &RType) -> (String, String, String) {
    let operands = &d.operands;
    let instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].yellow(),
        operands[2].blue(),
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

    return (instr, bits, hex);
}

fn format_i_type(d: &EncodedInstruction, i: &IType) -> (String, String, String) {
    let operands = &d.operands;
    let hex = format!("0x{:08x}", i.0).bold().to_string();
    let instr;
    if ["lb", "lh", "lw", "lbu", "lhu", "jalr"].contains(&d.mnemonic.as_str()) {
        instr = format!(
            "{} {}, {}({})",
            d.mnemonic.red().bold(),
            operands[0].green(),
            operands[1].blue(),
            operands[2].yellow(),
        );
    } else {
        instr = format!(
            "{} {}, {}, {}",
            d.mnemonic.red().bold(),
            operands[0].green(),
            operands[1].yellow(),
            operands[2].blue(),
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
    return (instr, bits, hex);
}

fn format_s_type(d: &EncodedInstruction, s: &SType) -> (String, String, String) {
    let operands = &d.operands;
    let instr = format!(
        "{} {}, {}({})",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
        operands[2].yellow(),
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
    return (instr, bits, hex);
}

fn format_b_type(d: &EncodedInstruction, b: &BType) -> (String, String, String) {
    let operands = &d.operands;
    let instr = format!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].yellow(),
        operands[2].blue(),
    );
    let fields = [
        format!("{:01b}", b.imm12() as u32).blue().to_string(),
        format!("{:06b}", b.imm10_5() as u32).blue().to_string(),
        format!("{:05b}", b.rs2()).green().to_string(),
        format!("{:05b}", b.rs1()).yellow().to_string(),
        format!("{:03b}", b.funct3()).red().to_string(),
        format!("{:04b}", b.imm4_1() as u32).blue().to_string(),
        format!("{:01b}", b.imm11() as u32).blue().to_string(),
        format!("{:07b}", b.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", b.0).bold().to_string();
    return (instr, bits, hex);
}

fn format_u_type(d: &EncodedInstruction, u: &UType) -> (String, String, String) {
    let operands = &d.operands;
    let instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
    );
    let fields = [
        format!("{:020b}", u.imm()).blue().to_string(),
        format!("{:05b}", u.rd()).green().to_string(),
        format!("{:07b}", u.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    let hex = format!("0x{:08x}", u.0).bold().to_string();
    return (instr, bits, hex);
}

fn format_j_type(d: &EncodedInstruction, j: &JType) -> (String, String, String) {
    let operands = &d.operands;
    let instr = format!(
        "{} {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].blue(),
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
    return (instr, bits, hex);
}

pub fn print_encoded_instruction(d: &EncodedInstruction) {
    let out = match &d.instr {
        Instruction::RType(r) => format_r_type(d, r),
        Instruction::IType(i) => format_i_type(d, i),
        Instruction::SType(s) => format_s_type(d, s),
        Instruction::BType(b) => format_b_type(d, b),
        Instruction::UType(u) => format_u_type(d, u),
        Instruction::JType(j) => format_j_type(d, j),
        /*
        Instruction::CSRType(_) => {
            print_csr_type(d);
        } */
        _ => (
            "Unsupported instruction type".to_string(),
            "".to_string(),
            "".to_string(),
        ),
    };

    println!("ASM: {}", out.0);
    println!("BIN: {}", out.1);
    println!("HEX: {}\n", out.2);
}
