
const REG_NUM: usize = 32;

const REG_NAME: &'static [&'static str; REG_NUM] = &[
    "zero", "ra", "sp",  "gp",  "tp", "t0", "t1", "t2",
    "s0",   "s1", "a0",  "a1",  "a2", "a3", "a4", "a5",
    "a6",   "a7", "s2",  "s3",  "s4", "s5", "s6", "s7",
    "s8",   "s9", "s10", "s11", "t3", "t4", "t5", "t6"
];


pub type Reg = u8;

pub struct RegisterFile {
    regs: [i64; REG_NUM],
}

pub fn from_name(name: &str) -> Reg {
    let i = REG_NAME.iter()
        .position(|&s| s == name)
        .unwrap();
    i as Reg
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            regs: [0; REG_NUM]
        }
    }

    pub fn get(&self, reg: Reg) -> i64 {
        self.regs[reg as usize]
    }

    pub fn set(&mut self, reg: Reg, value: i64) {
        self.regs[reg as usize] = value
    }

    pub fn get_by_name(&self, reg: &str) -> i64 {
        self.get(from_name(reg))
    }

    pub fn set_by_name(&mut self, reg: &str, value: i64) {
        self.set(from_name(reg), value)
    }
}