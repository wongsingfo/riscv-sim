use crate::simulator::Simulator;
use crate::instruction::{Instruction, InstrMatch, IOperands};
use crate::instruction::InstFormat::*;
use crate::instruction::Instruction::*;
use crate::main;

fn decode(sim: &mut Simulator) -> Instruction {
    let inst: u32 = sim.memory.load_u32(sim.pc);
    if (inst & 0b11) != 0b11 {
        panic!("oa, it's a 16bit instruction");
    }
    if (inst & 0b11100) == 0b11100 {
        panic!("it's an instruction that is longer that 32bit");
    }

    matching(inst)
}

fn execute(sim: &mut Simulator, inst: Instruction) {
    let r = &mut sim.regs;
    let m = &mut sim.memory;
    match inst {
        ADDI(IOperands{imm, rs1, rd}) => {
            r.set(rd, r.get(rs1) + imm)
        },
        LUI() => {},
    }
}

fn matching<T>(code: T) -> Instruction
    where T: InstrMatch {
    if code.is_match(IFormat(0b000, 0b0010011)) {
        ADDI(code.decode_I())
    } else {
        LUI()
    }
}
