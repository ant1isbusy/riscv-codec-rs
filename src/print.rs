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

pub fn print_encoded_instruction(d: &EncodedInstruction) {
    let out = match &d.instr {
        Instruction::RType(r) => format_r_type(d, r),
        Instruction::IType(i) => format_i_type(d, i),
        /* Instruction::IType(_) => {
            print_i_type(d, i);
        }
        Instruction::SType(_) => {
            print_s_type(d);
        }
        Instruction::BType(_) => {
            print_b_type(d);
        }
        Instruction::UType(_) => {
            print_u_type(d);
        }
        Instruction::JType(_) => {
            print_j_type(d);
        }
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
