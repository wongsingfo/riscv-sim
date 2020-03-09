#![forbid(unsafe_code)]

use std::fs::File;
use std::io::{Seek, SeekFrom};

pub use error::Error;
use header::{
    ElfHeader,
    ProgramEntry,
    SectionEntry,
    StringTable,
};

use crate::header::SymbolEntry;

mod error;
mod header;
mod utils;

#[derive(Default)]
pub struct Elf {
    pub header: ElfHeader,
    pub programs: Vec<ProgramEntry>,
    sections: Vec<SectionEntry>,
    symbols: Vec<SymbolEntry>,
    shstrtab: StringTable,
    strtab: StringTable,
    // FIXME: magic tuple
    pub symbol_entries: Vec<(String, u64, u64)>,
}

impl Elf {
    fn find_section(&self, name: &str) -> Result<&SectionEntry, Error> {
        match self.sections.iter().filter(|x| {
            self.shstrtab.section_is(x, name)
        }).next() {
            Some(rv) => Ok(rv),
            None => Err(Error::CanNotFindSection(String::from(name)))
        }
    }

    pub fn open(filename: &str) -> Result<Elf, Error> {
        let mut elf: Elf = Default::default();
        let mut f = File::open(filename)?;
        elf.header = ElfHeader::from_reader(&mut f)?;

        let _ = f.seek(SeekFrom::Start(elf.header.phoff))?;
        for _ in 0..elf.header.phnum {
            let p = ProgramEntry::from_reader(&mut f)?;
            elf.programs.push(p);
        }

        let _ = f.seek(SeekFrom::Start(elf.header.shoff))?;
        for _ in 0..elf.header.shnum {
            let p = SectionEntry::from_reader(&mut f)?;
            elf.sections.push(p);
        }

        let shstr = &elf.sections[elf.header.shstrndx as usize];
        elf.shstrtab = StringTable::from_file(&mut f, shstr);

        let strtab = elf.find_section(".strtab")?;
        elf.strtab = StringTable::from_file(&mut f, strtab);

        let symtab = elf.find_section(".symtab")?;
        if symtab.entsize != header::ELF_SYMBOL_SIZE {
            return Err(Error::InvalidStructureSize);
        }
        let _ = f.seek(SeekFrom::Start(symtab.offset))?;
        for _ in 0..symtab.size / symtab.entsize {
            let p = SymbolEntry::from_reader(&mut f)?;
            elf.symbol_entries.push(
                (elf.strtab.lookup_symbol_name(&p), p.value, p.size));
            elf.symbols.push(p);
        }

        Ok(elf)
    }
}

#[test]
fn test_elf_header() {
    let filename = "test_obj/a.out";
    let elf = Elf::open(filename).unwrap();
    assert_eq!(elf.programs.len(), 2);
    assert_eq!(elf.programs[0].filesz, 0x4b6);
    assert_eq!(elf.sections.len(), 12);
    assert_eq!(elf.sections[9].entsize, 0x18); // [9] .symtab
    assert_eq!(elf.sections[11].size, 0x5e); // [11] .shstrtab
    elf.shstrtab.equal_to_string(1, ".symtab");
    elf.strtab.equal_to_string(1, "crtstuff");
}