use crate::error::{Error, Result};

pub fn decode(instr: u32) -> Result<String> {
    let opcode = instr & 0x7f;
    let rd = (instr >> 7) & 0x1f;

    let funct3 = (instr >> 12) & 0x7;
    let funct7 = (instr >> 25) & 0x7f;
    let rs1 = (instr >> 15) & 0x1f;
    let rs2 = (instr >> 20) & 0x1f;
    let imm_i = ((instr as i32) >> 20) as i32;

    let decoded = match opcode {
        0b0110011 => {
            // R-type
            match (funct3, funct7) {
                (0x0, 0x00) => format!("add x{}, x{}, x{}", rd, rs1, rs2),
                (0x0, 0x20) => format!("sub x{}, x{}, x{}", rd, rs1, rs2),
                (0x4, 0x00) => format!("xor x{}, x{}, x{}", rd, rs1, rs2),
                (0x6, 0x00) => format!("or x{}, x{}, x{}", rd, rs1, rs2),
                (0x7, 0x00) => format!("and x{}, x{}, x{}", rd, rs1, rs2),
                (0x1, 0x00) => format!("sll x{}, x{}, x{}", rd, rs1, rs2),
                (0x5, 0x00) => format!("srl x{}, x{}, x{}", rd, rs1, rs2),
                (0x5, 0x20) => format!("sra x{}, x{}, x{}", rd, rs1, rs2),
                (0x2, 0x00) => format!("slt x{}, x{}, x{}", rd, rs1, rs2),
                (0x3, 0x00) => format!("sltu x{}, x{}, x{}", rd, rs1, rs2),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        0b0010011 => {
            // I-type
            match funct3 {
                0x0 => format!("addi x{}, x{}, {}", rd, rs1, imm_i),
                0x4 => format!("xori x{}, x{}, {}", rd, rs1, imm_i),
                0x6 => format!("ori x{}, x{}, {}", rd, rs1, imm_i),
                0x7 => format!("andi x{}, x{}, {}", rd, rs1, imm_i),
                0x1 => {
                    let shamt = (instr >> 20) & 0x1f;
                    format!("slli x{}, x{}, {}", rd, rs1, shamt)
                }
                0x5 => {
                    let shamt = (instr >> 20) & 0x1f;
                    let funct7 = (instr >> 25) & 0x7f;
                    match funct7 {
                        0x00 => format!("srli x{}, x{}, {}", rd, rs1, shamt),
                        0x20 => format!("srai x{}, x{}, {}", rd, rs1, shamt),
                        _ => return Err(Error::UnknownInstruction),
                    }
                }
                0x2 => format!("slti x{}, x{}, {}", rd, rs1, imm_i),
                0x3 => format!("sltiu x{}, x{}, {}", rd, rs1, imm_i),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        0b0000011 => {
            // I-type LOAD
            match funct3 {
                0x0 => format!("lb x{}, {}(x{})", rd, imm_i, rs1),
                0x1 => format!("lh x{}, {}(x{})", rd, imm_i, rs1),
                0x2 => format!("lw x{}, {}(x{})", rd, imm_i, rs1),
                0x4 => format!("lbu x{}, {}(x{})", rd, imm_i, rs1),
                0x5 => format!("lhu x{}, {}(x{})", rd, imm_i, rs1),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        0b0100011 => {
            // S-type
            let imm_4_0 = (instr >> 7) & 0x1f;
            let imm_11_5 = (instr >> 25) & 0x7f;
            let imm_s = (((imm_11_5 << 5) | imm_4_0) as i32)
                .wrapping_shl(20)
                .wrapping_shr(20);

            match funct3 {
                0x0 => format!("sb x{}, {}(x{})", rs2, imm_s, rs1),
                0x1 => format!("sh x{}, {}(x{})", rs2, imm_s, rs1),
                0x2 => format!("sw x{}, {}(x{})", rs2, imm_s, rs1),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        0b1100011 => {
            // BRANCH
            let imm_11 = (instr >> 7) & 0x1;
            let imm_4_1 = (instr >> 8) & 0xf;
            let imm_10_5 = (instr >> 25) & 0x3f;
            let imm_12 = (instr >> 31) & 0x1;

            let imm_b = (((imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1))
                as i32)
                .wrapping_shl(19)
                .wrapping_shr(19);

            match funct3 {
                0x0 => format!("beq x{}, x{}, {}", rs1, rs2, imm_b),
                0x1 => format!("bne x{}, x{}, {}", rs1, rs2, imm_b),
                0x4 => format!("blt x{}, x{}, {}", rs1, rs2, imm_b),
                0x5 => format!("bge x{}, x{}, {}", rs1, rs2, imm_b),
                0x6 => format!("bltu x{}, x{}, {}", rs1, rs2, imm_b),
                0x7 => format!("bgeu x{}, x{}, {}", rs1, rs2, imm_b),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        0b1101111 => {
            // JAL
            let j_imm_20 = (instr >> 31) & 0x1;
            let j_imm_10_1 = (instr >> 21) & 0x3ff;
            let j_imm_11 = (instr >> 20) & 0x1;
            let j_imm_19_12 = (instr >> 12) & 0xff;

            let j_imm =
                (((j_imm_20 << 20) | (j_imm_19_12 << 12) | (j_imm_11 << 11) | (j_imm_10_1 << 1))
                    as i32)
                    .wrapping_shl(11)
                    .wrapping_shr(11);

            format!("jal x{}, {}", rd, j_imm)
        }
        0b1100111 => {
            // JALR
            if funct3 == 0x0 {
                format!("jalr x{}, {}(x{})", rd, imm_i, rs1)
            } else {
                return Err(Error::UnknownInstruction);
            }
        }
        0b0110111 => {
            // LUI
            let imm = (instr & 0xfffff000) as i32;
            format!("lui x{}, {}", rd, imm)
        }
        0b0010111 => {
            // AUIPC
            let imm = (instr & 0xfffff000) as i32;
            format!("auipc x{}, {}", rd, imm)
        }
        0b1110011 => {
            // SYSTEM
            let csr = (instr >> 20) & 0xfff;
            let zimm = (instr >> 15) & 0x1f;
            match (funct3, csr) {
                (0x0, 0x000) if rd == 0 && rs1 == 0 => "ecall".to_string(),
                (0x0, 0x001) if rd == 0 && rs1 == 0 => "ebreak".to_string(),
                (0x1, _) => format!("csrrw x{}, {}, x{}", rd, csr, rs1),
                (0x2, _) => format!("csrrs x{}, {}, x{}", rd, csr, rs1),
                (0x3, _) => format!("csrrc x{}, {}, x{}", rd, csr, rs1),
                (0x5, _) => format!("csrrwi x{}, {}, {}", rd, csr, zimm),
                (0x6, _) => format!("csrrsi x{}, {}, {}", rd, csr, zimm),
                (0x7, _) => format!("csrrci x{}, {}, {}", rd, csr, zimm),
                _ => return Err(Error::UnknownInstruction),
            }
        }
        _ => return Err(Error::InvalidOpcode),
    };

    Ok(decoded)
}
