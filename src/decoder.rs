use crate::{
    error::{Error, Result},
    format::{BType, CSRType, EncodedInstruction, IType, Instruction, JType, RType, SType, UType},
};

pub fn decode(instr: u32) -> Result<EncodedInstruction> {
    let opcode = instr & 0x7f;
    let (mnemonic, operands, instr_enum) = match opcode {
        0b0110011 => {
            // R-type
            let r = RType(instr);
            let rd = r.rd();
            let rs1 = r.rs1();
            let rs2 = r.rs2();
            let funct3 = r.funct3();
            let funct7 = r.funct7();
            let mnemonic = match (funct3, funct7) {
                (0x0, 0x00) => "add",
                (0x0, 0x20) => "sub",
                (0x4, 0x00) => "xor",
                (0x6, 0x00) => "or",
                (0x7, 0x00) => "and",
                (0x1, 0x00) => "sll",
                (0x5, 0x00) => "srl",
                (0x5, 0x20) => "sra",
                (0x2, 0x00) => "slt",
                (0x3, 0x00) => "sltu",
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = vec![format!("x{}", rd), format!("x{}", rs1), format!("x{}", rs2)];
            (mnemonic.to_string(), operands, Instruction::RType(r))
        }
        0b0010011 => {
            // I-type
            let i = IType(instr);
            let rd = i.rd();
            let rs1 = i.rs1();
            let funct3 = i.funct3();
            let imm = ((instr as i32) >> 20) as i32;
            let (mnemonic, arg3) = match funct3 {
                0x0 => ("addi", format!("{}", imm)),
                0x4 => ("xori", format!("{}", imm)),
                0x6 => ("ori", format!("{}", imm)),
                0x7 => ("andi", format!("{}", imm)),
                0x1 => {
                    let shamt = (instr >> 20) & 0x1f;
                    ("slli", format!("{}", shamt))
                }
                0x5 => {
                    let shamt = (instr >> 20) & 0x1f;
                    let funct7 = (instr >> 25) & 0x7f;
                    match funct7 {
                        0x00 => ("srli", format!("{}", shamt)),
                        0x20 => ("srai", format!("{}", shamt)),
                        _ => return Err(Error::UnknownInstruction),
                    }
                }
                0x2 => ("slti", format!("{}", imm)),
                0x3 => ("sltiu", format!("{}", imm)),
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = vec![format!("x{}", rd), format!("x{}", rs1), arg3];
            (mnemonic.to_string(), operands, Instruction::IType(i))
        }
        0b0000011 => {
            // I-type LOAD
            let i = IType(instr);
            let rd = i.rd();
            let rs1 = i.rs1();
            let funct3 = i.funct3();
            let imm = ((instr as i32) >> 20) as i32;
            let mnemonic = match funct3 {
                0x0 => "lb",
                0x1 => "lh",
                0x2 => "lw",
                0x4 => "lbu",
                0x5 => "lhu",
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = vec![format!("x{}", rd), format!("{}", imm), format!("x{}", rs1)];
            (mnemonic.to_string(), operands, Instruction::IType(i))
        }
        0b0100011 => {
            // S-type
            let s = SType(instr);
            let rs2 = s.rs2();
            let rs1 = s.rs1();
            let funct3 = s.funct3();
            let imm = (((s.imm11_5() << 5) | s.imm4_0()) as i32)
                .wrapping_shl(20)
                .wrapping_shr(20);
            let mnemonic = match funct3 {
                0x0 => "sb",
                0x1 => "sh",
                0x2 => "sw",
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = vec![format!("x{}", rs2), format!("{}", imm), format!("x{}", rs1)];
            (mnemonic.to_string(), operands, Instruction::SType(s))
        }
        0b1100011 => {
            // BRANCH
            let b = BType(instr);
            let rs1 = b.rs1();
            let rs2 = b.rs2();
            let funct3 = b.funct3();
            let imm = (((b.imm12() as u32) << 12
                | ((b.imm11() as u32) << 11)
                | (b.imm10_5() << 5)
                | (b.imm4_1() << 1)) as i32)
                .wrapping_shl(19)
                .wrapping_shr(19);
            let mnemonic = match funct3 {
                0x0 => "beq",
                0x1 => "bne",
                0x4 => "blt",
                0x5 => "bge",
                0x6 => "bltu",
                0x7 => "bgeu",
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = vec![format!("x{}", rs1), format!("x{}", rs2), format!("{}", imm)];
            (mnemonic.to_string(), operands, Instruction::BType(b))
        }
        0b1101111 => {
            // JAL
            let j = JType(instr);
            let rd = j.rd();
            let j_imm = ((((j.imm20() as u32) << 20)
                | (j.imm19_12() << 12)
                | ((j.imm11() as u32) << 11)
                | (j.imm10_1() << 1)) as i32)
                .wrapping_shl(11)
                .wrapping_shr(11);
            let mnemonic = "jal";
            let operands = vec![format!("x{}", rd), format!("{}", j_imm)];
            (mnemonic.to_string(), operands, Instruction::JType(j))
        }
        0b1100111 => {
            // JALR
            let i = IType(instr);
            let rd = i.rd();
            let rs1 = i.rs1();
            let funct3 = i.funct3();
            let imm = ((instr as i32) >> 20) as i32;
            let mnemonic = "jalr";
            if funct3 == 0x0 {
                let operands = vec![format!("x{}", rd), format!("{}", imm), format!("x{}", rs1)];
                (mnemonic.to_string(), operands, Instruction::IType(i))
            } else {
                return Err(Error::UnknownInstruction);
            }
        }
        0b0110111 => {
            // LUI
            let u = UType(instr);
            let rd = u.rd();
            let imm = (instr & 0xfffff000) as i32;
            let mnemonic = "lui";
            let operands = vec![format!("x{}", rd), format!("{}", imm)];
            (mnemonic.to_string(), operands, Instruction::UType(u))
        }
        0b0010111 => {
            // AUIPC
            let u = UType(instr);
            let rd = u.rd();
            let imm = (instr & 0xfffff000) as i32;
            let mnemonic = "auipc";
            let operands = vec![format!("x{}", rd), format!("{}", imm)];
            (mnemonic.to_string(), operands, Instruction::UType(u))
        }
        0b1110011 => {
            // SYSTEM
            let c = CSRType(instr);
            let rd = c.rd();
            let csr = c.csr();
            let rs1 = c.rs1();
            let funct3 = c.funct3();
            let zimm = (instr >> 15) & 0x1f;
            let mnemonic = match (funct3, csr) {
                (0x0, 0x000) if rd == 0 && rs1 == 0 => "ecall",
                (0x0, 0x001) if rd == 0 && rs1 == 0 => "ebreak",
                (0x1, _) => "csrrw",
                (0x2, _) => "csrrs",
                (0x3, _) => "csrrc",
                (0x5, _) => "csrrwi",
                (0x6, _) => "csrrsi",
                (0x7, _) => "csrrci",
                _ => return Err(Error::UnknownInstruction),
            };
            let operands = match mnemonic {
                "ecall" => vec![],
                "ebreak" => vec![],
                "csrrw" | "csrrs" | "csrrc" => {
                    vec![format!("x{}", rd), format!("{}", csr), format!("x{}", rs1)]
                }
                "csrrwi" | "csrrsi" | "csrrci" => {
                    vec![format!("x{}", rd), format!("{}", csr), format!("{}", zimm)]
                }
                _ => unreachable!(),
            };
            (mnemonic.to_string(), operands, Instruction::CSRType(c))
        }
        _ => return Err(Error::InvalidOpcode),
    };

    Ok(EncodedInstruction {
        instr: instr_enum,
        mnemonic,
        operands,
    })
}
