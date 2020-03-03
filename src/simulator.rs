use crate::memory::{Memory, MemorySegment};
use crate::register::RegisterFile;
use objdump::Elf;
use std::fs::File;
use std::io::{Seek, SeekFrom};

pub struct Simulator {
    pub memory: Memory,
    pub regs: RegisterFile,
    pub pc: u64,
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            memory: Memory::new(),
            regs: RegisterFile::new(),
            pc: 0,
        }
    }

    pub fn load_from_elf(&mut self, filename: &str) {
        let elf: Elf = Elf::open(filename)
            .expect("can not open the binary file");

        let mut f = File::open(filename).unwrap();
        elf.programs.iter().for_each(|segment| {
            let _ = f.seek(SeekFrom::Start(segment.off)).unwrap();
            debug_assert!(segment.memsz >= segment.filesz);
            let mut seg = MemorySegment::new(
                segment.vaddr, segment.memsz as usize);
            seg.load_from(&mut f, segment.filesz as usize);
        })
    }
}
