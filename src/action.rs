use crate::simulator::Simulator;
use crate::instruction::{InstrMatch, IOperands};
use crate::instruction::InstFormat::*;
use crate::{main, register};
use crate::register::{Reg};
use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    ADDI(IOperands),
    LUI(),
}

#[derive(Default, Copy, Clone)]
pub struct ExecuteInfo {
    pub exe_cycles: u64,
    pub mem_access: u64,
    pub load_reg: Reg,
    pub reg_read: [Reg; 2],
    pub is_branch: bool,
    pub taken_branch: bool,
}

pub(crate) fn execute(sim: &mut Simulator, inst: Instruction) -> ExecuteInfo {
    let r = &mut sim.regs;
    let m = &mut sim.memory;
    let pc = &mut sim.pc;
    let mut exe_cycles = 1;
    let mut access = 0;
    let mut load_reg = Default::default();
    let mut reg_read: [Reg; 2] = Default::default();
    let mut is_branch = false;
    let mut taken_branch = false;
    match inst {
        ADDI(IOperands{imm, rs1, rd}) => {
            r.set(rd, r.get(rs1) + imm);
        },
        LUI() => {
        },
    };
    let mem_access = sim.cache.access(access);
    ExecuteInfo {
        exe_cycles,
        mem_access,
        load_reg,
        reg_read,
        is_branch,
        taken_branch,
    }
}

pub(crate) fn matching<T>(code: T) -> Instruction
    where T: InstrMatch {
    if code.is_match(IFormat(0b000, 0b0010011)) {
        ADDI(code.decode_I())
    } else {
        LUI()
    }
}
