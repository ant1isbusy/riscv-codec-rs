use crate::error::{Error, Result};
use bitfield::bitfield;

#[derive(Debug)]
pub enum Instruction {
    RType(RType),
    IType(IType),
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
