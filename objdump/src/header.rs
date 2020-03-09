use std::fs;
use std::fs::File;
use std::io::{Read, SeekFrom, Seek};

use crate::error::Error;
use crate::utils::*;

const ELF_MAGIC: [u8; 4] = [0x7F, 0x45, 0x4c, 0x46];
const ELF_HEADER_SIZE: u16 = 0x40;
const ELF_SHENT_SIZE: u16 = 0x40;
const ELF_PHENT_SIZE: u16 = 0x38;
pub(crate) const ELF_SYMBOL_SIZE: u64 = 0x18;

#[derive(Default)]
pub struct ElfHeader {
    pub entry: u64,
    pub shoff: u64,
    pub phoff: u64,
    pub phnum: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[derive(Default)]
pub struct SectionEntry {
    pub name: u32,
    pub offset: u64,
    pub size: u64,
    pub entsize: u64,
}

#[derive(Default)]
pub struct ProgramEntry {
    pub off: u64,
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
}

#[derive(Default)]
pub struct StringTable {
    table: Vec<u8>,
}

#[derive(Default)]
pub struct SymbolEntry {
    pub name: u32,
    pub value: u64,
    pub size: u64,
}

impl ElfHeader {
    pub fn from_reader(reader: &mut impl Read) -> Result<ElfHeader, Error> {
        let mut rv: ElfHeader = Default::default();
        let mut b = [0; ELF_HEADER_SIZE as usize];
        reader.read_exact(&mut b)?;
        if &b[0..4] != ELF_MAGIC {
            return Err(Error::InvalidMagic);
        }
        if b[4] != 2 {
            return Err(Error::Not64Bit);
        }
        if b[5] != 1 {
            return Err(Error::NotLittleEndianness);
        }
        if array_as_u16(&b[0x34..0x36]) != ELF_HEADER_SIZE ||
            array_as_u16(&b[0x36..0x38]) != ELF_PHENT_SIZE ||
            array_as_u16(&b[0x3a..0x3c]) != ELF_SHENT_SIZE {
            return Err(Error::InvalidStructureSize);
        }

        rv.entry = array_as_u64(&b[0x18..]);
        rv.phoff = array_as_u64(&b[0x20..]);
        rv.shoff = array_as_u64(&b[0x28..]);
        rv.phnum = array_as_u16(&b[0x38..]);
        rv.shnum = array_as_u16(&b[0x3c..]);
        rv.shstrndx = array_as_u16(&b[0x3e..]);

        Ok(rv)
    }
}

impl SectionEntry {
    pub fn from_reader(reader: &mut impl Read)
                       -> Result<SectionEntry, Error> {
        let mut rv: SectionEntry = Default::default();
        let mut b = [0; ELF_SHENT_SIZE as usize];
        reader.read_exact(&mut b)?;

        rv.name = array_as_u32(&b[0x0..]);
        rv.offset = array_as_u64(&b[0x18..]);
        rv.size = array_as_u64(&b[0x20..]);
        rv.entsize = array_as_u64(&b[0x38..]);

        Ok(rv)
    }
}

impl ProgramEntry {
    pub fn from_reader(reader: &mut impl Read)
                       -> Result<ProgramEntry, Error> {
        let mut rv: ProgramEntry = Default::default();
        let mut b = [0; ELF_PHENT_SIZE as usize];
        reader.read_exact(&mut b)?;

        rv.vaddr = array_as_u64(&b[0x10..]);
        rv.off = array_as_u64(&b[0x08..]);
        rv.filesz = array_as_u64(&b[0x20..]);
        rv.memsz = array_as_u64(&b[0x28..]);

        Ok(rv)
    }
}

impl StringTable {
    fn lookup(&self, index: usize) -> &[u8] {
        let mut rear = index;
        while self.table[rear] != 0 {
            rear += 1;
        }
        &self.table[index..rear]
    }

    pub fn lookup_symbol_name(&self, sym: &SymbolEntry) -> String {
        let b = self.lookup(sym.name as usize);
        match String::from_utf8(Vec::from(b)) {
            Ok(rv) =>
                if rv.is_empty() {String::from("<null>")} else {rv},
            Err(e) => String::from("<unknown>"),
        }
    }

    pub fn equal_to_string(&self, index: usize, s: &str) -> bool {
        s.as_bytes() == self.lookup(index)
    }

    pub fn from_reader(mut reader: impl Read, size: usize) -> Self {
        let mut rv = Self {
            table: Vec::new(),
        };
        rv.table.resize(size, 0);
        reader.read_exact(rv.table.as_mut_slice());
        rv
    }

    pub fn from_file(f: &mut File, sec: &SectionEntry) -> Self {
        f.seek(SeekFrom::Start(sec.offset));
        StringTable::from_reader(f, sec.size as usize)
    }

    pub fn section_is(&self, sec: &SectionEntry, s: &str) -> bool {
        self.equal_to_string(sec.name as usize, s)
    }
}

impl SymbolEntry {
    pub fn from_reader(reader: &mut impl Read)
                       -> Result<SymbolEntry, Error> {
        let mut rv: SymbolEntry = Default::default();
        let mut b = [0; ELF_SYMBOL_SIZE as usize];
        reader.read_exact(&mut b)?;

        rv.name = array_as_u32(&b[0x0..]);
        rv.value = array_as_u64(&b[0x08..]);
        rv.size = array_as_u64(&b[0x10..]);

        Ok(rv)
    }
}

#[test]
fn test_elf_header() {
    let filename = "test_obj/a.out";
    let mut f = fs::File::open(filename).unwrap();
    let header: ElfHeader = ElfHeader::from_reader(&mut f).unwrap();
}