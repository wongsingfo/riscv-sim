
type Duration = u64;

pub enum CacheOp {
    Read,
    Write,
}

pub trait Storage {
    fn access(&mut self, address: u64, op: CacheOp) -> Duration;
}

#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    write_through: bool,
    write_allocate: bool,
    capacity: u64,
    associativity: u64,
    line_size: u64,
    latency: Duration,
}

#[derive(Default, Debug, Clone, Copy)]
struct CacheLine {
    is_valid: bool,
    last_visit: u64,
    tag: u64,
}

#[derive(Default, Debug, Clone)]
struct CacheLines {
    last_visit: u64,
    lines: Vec<CacheLine>,
}

pub struct Cache {
    config: CacheConfig,
    lower: Box<dyn Storage>,
    line_mask: u64,
    tag_mask: u64,
    lines: Vec<CacheLines>,
}

#[derive(Debug, Clone, Copy)]
pub struct DRAM {
    latency: Duration,
}

pub fn new() -> Box<dyn Storage> {
    let dram = DRAM {
        latency: 10,
    };
    Box::new(dram)
}

impl CacheLines {
    fn find(&mut self, tag: u64) -> Option<&mut CacheLine> {
        self.last_visit += 1;
        let mut result = self.lines.iter_mut().find(|x| x.tag == tag);
        if let Some(mut line) = result.as_mut() {
            line.last_visit = self.last_visit;
        }
        result 
    }

    fn insert(&mut self, tag: u64) {
        debug_assert!(self.find(tag).is_none());

        self.last_visit += 1;

        *(match self.lines.iter_mut().find(|x| !x.is_valid) {
            Some(x) => x,
            None => self.lines.iter_mut().min_by_key(|x| x.last_visit).unwrap(),
        }) = CacheLine {
            is_valid: true,
            last_visit: self.last_visit,
            tag,
        }
    }
}

impl Cache {
    pub fn new(config: CacheConfig, lower: Box<dyn Storage>) -> Self {
        assert_eq!(config.capacity % config.line_size, 0);
        let num_lines = config.capacity / config.line_size;

        Self {
            config,
            lower,
            line_mask: ((num_lines / config.associativity) - 1) * config.line_size,
            tag_mask: !(config.capacity - 1),
            lines: vec![CacheLines::default(); num_lines as usize],
        }
    }

    fn read(&mut self, address: u64) -> Duration {
        let lines =
            &mut self.lines[((address & self.line_mask) / self.config.line_size) as usize];
        let tag = address & self.tag_mask;

        match lines.find(tag) {
            Some(line) => self.config.latency,
            None => {
                lines.insert(tag);
                self.config.latency + self.lower.access(address, CacheOp::Read)
            },
        }
    }

    fn write(&mut self, address: u64) -> Duration {
        let lines =
            &mut self.lines[((address & self.line_mask) / self.config.line_size) as usize];
        let tag = address & self.tag_mask;

        match lines.find(tag) {
            Some(line) => if self.config.write_through {
                self.config.latency + self.lower.access(address, CacheOp::Write)
            } else {
                self.config.latency
            },
            None => if self.config.write_allocate {
                lines.insert(tag);
                self.config.latency
            } else {
                self.config.latency + self.lower.access(address, CacheOp::Write)
            }
        }
    }
}

impl Storage for Cache {
    fn access(&mut self, address: u64, op: CacheOp) -> Duration {
        match op {
            Read => self.read(address),
            Write => self.write(address),
        }
    }
}

impl Storage for DRAM {
    fn access(&mut self, _address: u64, _op: CacheOp) -> Duration {
        self.latency
    }
}


