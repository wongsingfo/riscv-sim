use std::cmp::max;
use std::fs::File;
use std::io::{Seek, SeekFrom};

use objdump::Elf;

use crate::action;
use crate::action::{ExecuteInfo, Instruction};
use crate::cache;
use crate::cache::{Storage};
use crate::memory::{Memory, MemorySegment};
use crate::register::{RegisterFile, from_name};
use crate::statistic::Statistic;

const STACK_ADDRESS: u64 = 0x3f3f3f_fffff;
const STACK_SIZE: usize = 4096;

pub struct Simulator {
    pub memory: Memory,
    pub regs: RegisterFile,
    pub elf: Elf,
    pub pc: u64,
    pub stat: Statistic,
    pub cache: Box<dyn Storage>,
    pub instr: [ExecuteInfo; 5],
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            memory: Memory::new(),
            regs: RegisterFile::new(),
            pc: 0,
            elf: Elf::default(),
            stat: Statistic::default(),
            cache: cache::new(),
            instr: [ExecuteInfo::default(); 5],
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
            println!("load segment {:x} ~ {:x}",
                     segment.vaddr,
                     segment.vaddr + segment.memsz);
            seg.load_from(&mut f, segment.filesz as usize);
            self.memory.push(seg);
        });

        self.memory.push(MemorySegment::new(
            STACK_ADDRESS - STACK_SIZE as u64, STACK_SIZE));
        self.regs.set(from_name("sp"), STACK_ADDRESS);

        let main = elf.symbol_entries.iter()
            .filter(|x| {
                x.0.contains("main")
            }).next().unwrap();
        self.pc = main.1;
        self.elf = elf;
    }

    fn decode(&mut self) -> Instruction {
        let inst: u32 = self.memory.load_u32(self.pc);
        if (inst & 0b11) != 0b11 {
            panic!("oa, it's a 16bit instruction");
        }
        if (inst & 0b11100) == 0b11100 {
            panic!("it's an instruction that is longer that 32bit");
        }

        action::matching(inst)
    }

    pub fn run(&mut self) -> bool {
        if self.pc == 0 {
            return false
        }
        print!("{:<7x}", self.pc);
        let inst = self.decode();
        println!("{:?}", inst);
        self.single_step(inst);
        true
    }

    fn single_step(&mut self, inst: Instruction) {
        self.stat.num_inst += 1;
        self.instr[4] = self.instr[3];    // WB
        self.instr[3] = self.instr[2];    // MEM
        self.instr[2] = self.instr[1];    // EX
        self.instr[1] = self.instr[0];    // ID
        self.instr[0] = action::execute(self, inst);
        let mut cycles = max(
            self.instr[3].mem_access,
            self.instr[2].exe_cycles);
        let load_reg = self.instr[3].load_reg;
        if load_reg.not_zero() {
            if self.instr[2].reg_read[0] == load_reg
                || self.instr[2].reg_read[1] == load_reg {
                cycles = max(cycles, 2);
                self.stat.num_data_hazard += 1;
            }
        }
        if self.instr[2].is_branch {
            self.stat.num_branch += 1;
            if !self.instr[2].taken_branch {
                self.stat.num_mis_pred += 1;
            }
        }
        self.stat.cycle += cycles;
    }
}
