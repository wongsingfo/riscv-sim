use crate::memory::Memory;
use crate::register::RegisterFile;

pub struct Simulator {
    memory: Memory,
    regs: RegisterFile,
    pc: u64,
}

impl Simulator {
    fn new() -> Self {
        Simulator {
            memory: Memory::new(),
            regs: RegisterFile::new(),
            pc: 0,
        }
    }
}
