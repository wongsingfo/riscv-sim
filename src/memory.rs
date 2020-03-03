use std::os::macos::raw::stat;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian};

pub struct Memory {
    segments: Vec<MemorySegment>,
}

pub struct MemorySegment {
    start: u64,
    array: Vec<u8>,
    size: usize,
}

impl MemorySegment {
    pub fn new(start: u64, size: usize) -> Self {
        Self {
            start,
            size,
            array: vec![0; size],
        }
    }

    fn end(&self) -> u64 {
        self.start + (self.size as u64)
    }

    fn contains(&self, address: u64) -> bool {
        self.start <= address && address < self.end()
    }

    fn store_u8(&mut self, address: u64, value: u8) {
        debug_assert!(self.contains(address));
        let offset = address - self.start;
        self.array[offset as usize] = value
    }

    fn store_u16(&mut self, address: u64, value: u16) {
        debug_assert!(self.contains(address));
        debug_assert!(self.contains(address + 1));
        let offset = (address - self.start) as usize;
        LittleEndian::write_u16(&mut self.array[offset..], value)
    }

    fn store_u32(&mut self, address: u64, value: u32) {
        debug_assert!(self.contains(address));
        debug_assert!(self.contains(address + 2));
        let offset = (address - self.start) as usize;
        LittleEndian::write_u32(&mut self.array[offset..], value)
    }

    fn load_u8(&self, address: u64) -> u8 {
        debug_assert!(self.contains(address));
        let offset = address - self.start;
        self.array[offset as usize]
    }

    fn load_u16(&self, address: u64) -> u16 {
        debug_assert!(self.contains(address));
        debug_assert!(self.contains(address + 1));
        let offset = (address - self.start) as usize;
        LittleEndian::read_u16(&self.array[offset..])
    }

    fn load_u32(&self, address: u64) -> u32 {
        debug_assert!(self.contains(address));
        debug_assert!(self.contains(address + 3));
        let offset = (address - self.start) as usize;
        LittleEndian::read_u32(&self.array[offset..])
    }

    pub fn load_from<T>(&mut self, reader: &mut T, size: usize)
        where T: Read {
        reader.read_exact(&mut self.array[..size]).unwrap();
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn push(&mut self, segment: MemorySegment) {
        self.segments.push(segment);
    }

    pub fn alloc(&mut self, start: u64, size: usize) {
        self.segments.push(
            MemorySegment::new(start, size)
        )
    }

    pub fn store_u8(&mut self, address: u64, value: u8) {
        self.segments.iter_mut()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .store_u8(address, value)
    }

    pub fn load_u8(&mut self, address: u64) -> u8 {
        self.segments.iter()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .load_u8(address)
    }

    pub fn store_u16(&mut self, address: u64, value: u16) {
        self.segments.iter_mut()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .store_u16(address, value)
    }

    pub fn load_u16(&mut self, address: u64) -> u16 {
        self.segments.iter()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .load_u16(address)
    }

    pub fn store_u32(&mut self, address: u64, value: u32) {
        self.segments.iter_mut()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .store_u32(address, value)
    }

    pub fn load_u32(&mut self, address: u64) -> u32 {
        self.segments.iter()
            .find(|x| x.contains(address))
            .expect("invalid memory address")
            .load_u32(address)
    }
}
