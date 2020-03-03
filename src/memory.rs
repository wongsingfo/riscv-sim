use std::os::macos::raw::stat;
use std::io::Read;

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

    fn load_u8(&self, address: u64) -> u8 {
        debug_assert!(self.contains(address));
        let offset = address - self.start;
        self.array[offset as usize]
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
            .unwrap()
            .store_u8(address, value)
    }

    pub fn load_u8(&mut self, address: u64) -> u8 {
        self.segments.iter()
            .find(|x| x.contains(address))
            .unwrap()
            .load_u8(address)
    }

    pub fn store_u16(&mut self, address: u64, value: u16) {
        self.store_u8(address, value as u8);
        self.store_u8(address + 1, (value & 0xff) as u8)
    }
}
