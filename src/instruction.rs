#![allow(non_snake_case)]

use crate::register::Reg;

const R_MASK: u32 =     0b_1111111_00000_00000_111_00000_1111111_u32;
const I_MASK: u32 =     0b_0000000_00000_00000_111_00000_1111111_u32;

pub enum InstFormat {
    RFormat(u32, u32, u32),
    IFormat(u32, u32),
}

pub struct ROperands {
    pub rs2: Reg,
    pub rs1: Reg,
    pub rd: Reg,
}

pub struct IOperands {
    pub imm: u64,
    pub rs1: Reg,
    pub rd: Reg,
}

pub enum Instruction {
    ADDI(IOperands),
    LUI(),
}

pub trait InstrMatch {
    fn is_match(&self, format: InstFormat) -> bool;
    fn decode_I(&self) -> IOperands;
}

fn u32_bits(value: u32, from: u32, to: u32) -> u32 {
    let len = to - from;
    let mask = (1 << len) - 1;
    (value & mask) >> len
}

fn u32_opcode(value: u32) -> u32 { u32_bits(value, 0, 7) }
fn u32_rd    (value: u32) -> Reg { u32_bits(value, 7, 12) as Reg }
fn u32_rs1   (value: u32) -> Reg { u32_bits(value, 15, 20) as Reg }
fn u32_rs2   (value: u32) -> Reg { u32_bits(value, 20, 25) as Reg }
fn u32_i_imm (value: u32) -> u64 { (((value as i32) >> 25) as i64) as u64 }


impl InstrMatch for u32 {
    fn is_match(&self, format: InstFormat) -> bool {
        match format {
            InstFormat::RFormat(funct7, funct3, op) => {
                let v = (funct7 << 25) | (funct3 << 12) | op;
                (self & R_MASK) == v
            }
            InstFormat::IFormat(funct3, op) => {
                let v = (funct3 << 12) | op;
                (self & I_MASK) == v
            }
        }
    }

    fn decode_I(&self) -> IOperands {
        IOperands {
            imm: u32_i_imm(*self),
            rs1: u32_rs1(*self),
            rd: u32_rd(*self),
        }
    }
}

