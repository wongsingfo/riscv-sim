use std::fmt::{Debug, Formatter, Error};

const REG_NUM: usize = 32;

const REG_NAME: &'static [&'static str; REG_NUM] = &[
    "zero", "ra", "sp",  "gp",  "tp", "t0", "t1", "t2",
    "s0",   "s1", "a0",  "a1",  "a2", "a3", "a4", "a5",
    "a6",   "a7", "s2",  "s3",  "s4", "s5", "s6", "s7",
    "s8",   "s9", "s10", "s11", "t3", "t4", "t5", "t6"
];

#[derive(Copy, Clone, Default)]
pub struct Reg {
    index: u8,
}

impl Debug for Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", REG_NAME[self.index as usize])
    }
}

impl Reg {
    pub fn not_zero(&self) -> bool {
        self.index != 0
    }
}

impl std::cmp::PartialEq for Reg {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl From<u32> for Reg {
    fn from(index: u32) -> Self {
        Self {
            index: index as u8
        }
    }
}

pub struct RegisterFile {
    regs: [u64; REG_NUM],
}

pub fn from_name(name: &str) -> Reg {
    let i = REG_NAME.iter()
        .position(|&s| s == name)
        .unwrap();
    Reg {
        index: i as u8
    }
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            regs: [0; REG_NUM]
        }
    }

    pub fn get(&self, reg: Reg) -> u64 {
        self.regs[reg.index as usize]
    }

    pub fn set(&mut self, reg: Reg, value: u64) {
        if reg.index == 0 { return }
        self.regs[reg.index as usize] = value
    }

    pub fn get_by_name(&self, reg: &str) -> u64 {
        self.get(from_name(reg))
    }

    pub fn set_by_name(&mut self, reg: &str, value: u64) {
        self.set(from_name(reg), value)
    }

    pub fn println(&self) {
        for i in 1..REG_NUM {
            let reg = Reg { index: i as u8 };
            let name = REG_NAME[i];
            let value = self.get(reg);
            print!("{:<3}={:0>16x} ", name, value);
            if i % 4 == 0 {
                println!()
            }
        }
        println!()
    }
}