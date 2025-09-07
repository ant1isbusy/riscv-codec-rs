use crate::error::{Error, Result};

pub fn decode(instr: u32) -> Result<String> {
    let opcode = instr & 0x7f;
    let rd = (instr >> 7) & 0x1f;

    let str = match opcode {
        0b0110011 => {
            // R-type
            let funct3 = (instr >> 12) & 0x7;
            let funct7 = (instr >> 25) & 0x7f;
            let rs1 = (instr >> 15) & 0x1f;
            let rs2 = (instr >> 20) & 0x1f;

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
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        0b0010011 => {
            // I-type
            let funct3 = (instr >> 12) & 0x7;
            let rs1 = (instr >> 15) & 0x1f;
            let imm = ((instr as i32) >> 20) as i32;

            match funct3 {
                0x0 => format!("addi x{}, x{}, {}", rd, rs1, imm),
                0x4 => format!("xori x{}, x{}, {}", rd, rs1, imm),
                0x6 => format!("ori x{}, x{}, {}", rd, rs1, imm),
                0x7 => format!("andi x{}, x{}, {}", rd, rs1, imm),
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
                        _ => return Err(Error::UnknownInstruction(instr)),
                    }
                }
                0x2 => format!("slti x{}, x{}, {}", rd, rs1, imm),
                0x3 => format!("sltiu x{}, x{}, {}", rd, rs1, imm),
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        0b0000011 => {
            // I-type LOAD
            let funct3 = (instr >> 12) & 0x7;
            let rs1 = (instr >> 15) & 0x1f;
            let imm = ((instr as i32) >> 20) as i32;

            match funct3 {
                0x0 => format!("lb x{}, {}(x{})", rd, imm, rs1),
                0x1 => format!("lh x{}, {}(x{})", rd, imm, rs1),
                0x2 => format!("lw x{}, {}(x{})", rd, imm, rs1),
                0x4 => format!("lbu x{}, {}(x{})", rd, imm, rs1),
                0x5 => format!("lhu x{}, {}(x{})", rd, imm, rs1),
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        0b0100011 => {
            // S-type
            let funct3 = (instr >> 12) & 0x7;
            let rs1 = (instr >> 15) & 0x1f;
            let rs2 = (instr >> 20) & 0x1f;
            let imm_4_0 = (instr >> 7) & 0x1f;
            let imm_11_5 = (instr >> 25) & 0x7f;
            let imm = (((imm_11_5 << 5) | imm_4_0) as i32)
                .wrapping_shl(20)
                .wrapping_shr(20);

            match funct3 {
                0x0 => format!("sb x{}, {}(x{})", rs2, imm, rs1),
                0x1 => format!("sh x{}, {}(x{})", rs2, imm, rs1),
                0x2 => format!("sw x{}, {}(x{})", rs2, imm, rs1),
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        0b1100011 => {
            // BRANCH
            let funct3 = (instr >> 12) & 0x7;
            let rs1 = (instr >> 15) & 0x1f;
            let rs2 = (instr >> 20) & 0x1f;
            let imm_11 = (instr >> 7) & 0x1;
            let imm_4_1 = (instr >> 8) & 0xf;
            let imm_10_5 = (instr >> 25) & 0x3f;
            let imm_12 = (instr >> 31) & 0x1;

            let imm = (((imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1)) as i32)
                .wrapping_shl(19)
                .wrapping_shr(19);

            match funct3 {
                0x0 => format!("beq x{}, x{}, {}", rs1, rs2, imm),
                0x1 => format!("bne x{}, x{}, {}", rs1, rs2, imm),
                0x4 => format!("blt x{}, x{}, {}", rs1, rs2, imm),
                0x5 => format!("bge x{}, x{}, {}", rs1, rs2, imm),
                0x6 => format!("bltu x{}, x{}, {}", rs1, rs2, imm),
                0x7 => format!("bgeu x{}, x{}, {}", rs1, rs2, imm),
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        0b1101111 => {
            // JAL
            let imm_20 = (instr >> 31) & 0x1;
            let imm_10_1 = (instr >> 21) & 0x3ff;
            let imm_11 = (instr >> 20) & 0x1;
            let imm_19_12 = (instr >> 12) & 0xff;

            let imm = (((imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1))
                as i32)
                .wrapping_shl(11)
                .wrapping_shr(11);

            format!("jal x{}, {}", rd, imm)
        }
        0b1100111 => {
            // JALR
            let funct3 = (instr >> 12) & 0x7;
            let rs1 = (instr >> 15) & 0x1f;
            let imm = ((instr as i32) >> 20) as i32;

            if funct3 == 0x0 {
                format!("jalr x{}, {}(x{})", rd, imm, rs1)
            } else {
                return Err(Error::UnknownInstruction(instr));
            }
        }
        0110111 => {
            // LUI
            let imm = (instr & 0xfffff000) as i32;
            format!("lui x{}, {}", rd, imm)
        }
        0010111 => {
            // AUIPC
            let imm = (instr & 0xfffff000) as i32;
            format!("auipc x{}, {}", rd, imm)
        }
        0b1110011 => {
            // SYSTEM
            let funct3 = (instr >> 12) & 0x7;
            let csr = (instr >> 20) & 0xfff;
            let zimm = (instr >> 15) & 0x1f;
            let rs1 = (instr >> 15) & 0x1f;
            match (funct3, csr) {
                (0x0, 0x000) if rd == 0 && rs1 == 0 => "ecall".to_string(),
                (0x0, 0x001) if rd == 0 && rs1 == 0 => "ebreak".to_string(),
                (0x1, _) => format!("csrrw x{}, {}, x{}", rd, csr, rs1),
                (0x2, _) => format!("csrrs x{}, {}, x{}", rd, csr, rs1),
                (0x3, _) => format!("csrrc x{}, {}, x{}", rd, csr, rs1),
                (0x5, _) => format!("csrrwi x{}, {}, {}", rd, csr, zimm),
                (0x6, _) => format!("csrrsi x{}, {}, {}", rd, csr, zimm),
                (0x7, _) => format!("csrrci x{}, {}, {}", rd, csr, zimm),
                _ => return Err(Error::UnknownInstruction(instr)),
            }
        }
        _ => return Err(Error::InvalidOpcode(opcode)),
    };

    Ok(str)
}
