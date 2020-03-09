#![allow(non_snake_case)]

use crate::register::Reg;
use std::fmt::{Debug, Formatter, Error};

const R_MASK: u32 =     0b_1111111_00000_00000_111_00000_1111111_u32;
const I_MASK: u32 =     0b_0000000_00000_00000_111_00000_1111111_u32;
const OP_MASK: u32 =    0b_0000000_00000_00000_000_00000_1111111_u32;

pub enum InstFormat {
    RFormat(u32, u32, u32),
    IFormat(u32, u32),
    SFormat(u32, u32),
    BFormat(u32, u32),
    UFormat(u32),
    JFormat(u32),
}

#[derive(Debug)]
pub struct ROperands {
    pub rs2: Reg,
    pub rs1: Reg,
    pub rd: Reg,
}

#[derive(Debug)]
pub struct IOperands {
    pub imm: u64,
    pub rs1: Reg,
    pub rd: Reg,
}

#[derive(Debug)]
pub struct SOperands {
    pub imm: u64,
    pub rs2: Reg,
    pub rs1: Reg,
}

#[derive(Debug)]
pub struct BOperands {
    pub imm: u64,
    pub rs2: Reg,
    pub rs1: Reg,
}

#[derive(Debug)]
pub struct UOperands {
    pub imm: u64,
    pub rd: Reg,
}

#[derive(Debug)]
pub struct JOperands {
    pub imm: u64,
    pub rd: Reg,
}

pub trait InstrMatch {
    fn is_match(&self, format: InstFormat) -> bool;
    fn decode_I(&self) -> IOperands;
    fn decode_S(&self) -> SOperands;
    fn decode_R(&self) -> ROperands;
    fn decode_B(&self) -> BOperands;
    fn decode_U(&self) -> UOperands;
    fn decode_J(&self) -> JOperands;
}

fn u32_bits(value: u32, from: u32, to: u32) -> u32 {
    let len = to - from;
    let mask = (1u32 << len) - 1;
    (value >> from) & mask
}

fn u32_inst(value: u32, from: u32, to: u32, start: u32) -> u64 {
    (u32_bits(value, from, to) as u64) << start as u64
}

fn u32_inst_sign(value: u32, start: u32) -> u64 {
    ((((value as i32 >> 31) as i64) >> start as i64) as u64) << start as u64
}

fn u32_opcode(value: u32) -> u32 { u32_bits(value, 0, 7) }

fn u32_rd(value: u32) -> Reg { Reg::from(u32_bits(value, 7, 12)) }

fn u32_rs1(value: u32) -> Reg { Reg::from(u32_bits(value, 15, 20)) }

fn u32_rs2(value: u32) -> Reg { Reg::from(u32_bits(value, 20, 25)) }

fn u32_i_imm(value: u32) -> u64 {
    u32_inst_sign(value, 11)
        | u32_inst(value, 20, 31, 0)
}

fn u32_s_imm(value: u32) -> u64 {
    u32_inst_sign(value, 11)
        | u32_inst(value, 25, 31, 5)
        | u32_inst(value, 7, 12, 0)
}

fn u32_b_imm(value: u32) -> u64 {
    u32_inst_sign(value, 31)
        | u32_inst(value, 7, 8, 11)
        | u32_inst(value, 25, 31, 5)
        | u32_inst(value, 8, 12, 1)
}

fn u32_u_imm(value: u32) -> u64 {
    u32_inst_sign(value, 31)
        | u32_inst(value, 12, 31, 12)
}

fn u32_j_imm(value: u32) -> u64 {
    u32_inst_sign(value, 20)
        | u32_inst(value, 12, 20, 12)
        | u32_inst(value, 20, 21, 11)
        | u32_inst(value, 21, 31, 1)
}

#[test]
fn test001() {
    assert_eq!(u32_inst_sign(0x7fffffff, 5), 0);
    assert_eq!(u32_inst_sign(0x81234567, 16), 0xffff_ffff_ffff_0000);
    assert_eq!(u32_bits(0xfa5ff0ef, 0, 6), 0b10_1111);
    assert_eq!(u32_bits(0xfa5ff0ef, 4, 6), 0b10);
    assert_eq!(u32_inst(0xfa5ff0ef, 0, 6, 4), 0b10_1111_0000);
    assert_eq!(u32_j_imm(0xfa5ff0ef), (-0x5ci32) as u64);
}

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
            InstFormat::SFormat(funct3, op) => {
                let v = (funct3 << 12) | op;
                (self & I_MASK) == v
            }
            InstFormat::BFormat(funct3, op) => {
                let v = (funct3 << 12) | op;
                (self & I_MASK) == v
            }
            InstFormat::UFormat(op) => {
                let v = op;
                (self & OP_MASK) == v
            }
            InstFormat::JFormat(op) => {
                let v = op;
                (self & OP_MASK) == v
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

    fn decode_S(&self) -> SOperands {
        SOperands {
            imm: u32_s_imm(*self),
            rs2: u32_rs2(*self),
            rs1: u32_rs1(*self),
        }
    }

    fn decode_R(&self) -> ROperands {
        ROperands {
            rs2: u32_rs2(*self),
            rs1: u32_rs1(*self),
            rd: u32_rd(*self),
        }
    }

    fn decode_B(&self) -> BOperands {
        BOperands {
            imm: u32_b_imm(*self),
            rs2: u32_rs2(*self),
            rs1: u32_rs1(*self),
        }
    }

    fn decode_U(&self) -> UOperands {
        UOperands {
            imm: u32_u_imm(*self),
            rd: u32_rd(*self),
        }
    }

    fn decode_J(&self) -> JOperands {
        JOperands {
            imm: u32_j_imm(*self),
            rd: u32_rd(*self),
        }
    }
}

