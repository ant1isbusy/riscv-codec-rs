use crate::encoder::*;
use colored::*;

fn print_r_type(d: &EncodedInstruction, r: &RType) {
    let operands = &d.operands;

    // print instruction (operands):
    println!(
        "{} {}, {}, {}",
        d.mnemonic.red().bold(),
        operands[0].green(),
        operands[1].yellow(),
        operands[2].blue(),
    );
    // print binary representation:
    let fields = [
        format!("{:07b}", r.funct7()).red().to_string(),
        format!("{:05b}", r.rs2()).blue().to_string(),
        format!("{:05b}", r.rs1()).yellow().to_string(),
        format!("{:03b}", r.funct3()).red().to_string(),
        format!("{:05b}", r.rd()).green().to_string(),
        format!("{:07b}", r.opcode()).red().to_string(),
    ];
    let bits = fields.join(" ");
    println!("{}", bits);
}

fn print_i_type(_d: &EncodedInstruction, _i: &IType) {
    // TODO
}

pub fn print_encoded_instruction(d: &EncodedInstruction) {
    match &d.instr {
        Instruction::RType(r) => {
            print_r_type(d, r);
        }
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
        _ => {
            println!("Encoded Instruction: {:?}", d.instr);
        }
    }
}
