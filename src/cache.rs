pub type Duration = u64;

pub enum CacheOp {
    Read,
    Write,
}

pub trait Storage {
    fn access(&mut self, address: u64, op: CacheOp) -> Duration;

    fn output_stats(&self) {
        // default: do nothing
    }

    fn stats(&self) -> StorageStats;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct StorageStats {
    pub num_access: u64,
    pub num_miss: u64,
    pub time: Duration,
}

#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    name: &'static str,
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
    is_dirty: bool,
    last_visit: u64,
    tag: u64,
    address: u64,
}

#[derive(Default, Debug, Clone)]
struct CacheLines {
    last_visit: u64,
    lines: Vec<CacheLine>,
}

pub struct Cache {
    stats: StorageStats,
    config: CacheConfig,
    lower: Box<dyn Storage>,
    line_mask: u64,
    tag_mask: u64,
    lines: Vec<CacheLines>,
}

#[derive(Debug, Clone, Copy)]
pub struct Dram {
    latency: Duration,
    stats: StorageStats,
}

pub fn new() -> Box<dyn Storage> {
    let dram = Box::new(Dram::new(13));
    let llc = Box::new(Cache::new(
        CacheConfig {
            name: "LLC",
            write_through: false,
            write_allocate: true,
            capacity: 8 * 1024 * 1024,
            associativity: 8,
            line_size: 64,
            latency: 4,
        },
        dram
    ));

    let l2 = Box::new(Cache::new(
        CacheConfig {
            name: "L2",
            write_through: false,
            write_allocate: true,
            capacity: 256 * 1024,
            associativity: 8,
            line_size: 64,
            latency: 2,
        },
        llc
    ));

    let l1 = Box::new(Cache::new(
        CacheConfig {
            name: "L1",
            write_through: false,
            write_allocate: true,
            capacity: 32 * 1024,
            associativity: 8,
            line_size: 64,
            latency: 1,
        },
        l2
    ));

    l1
}

impl CacheLines {
    fn new(size: usize) -> CacheLines {
        CacheLines {
            last_visit: 0,
            lines: vec![CacheLine::default(); size],
        }
    }

    fn find(&mut self, tag: u64) -> Option<&mut CacheLine> {
        self.last_visit += 1;
        let mut result = self.lines.iter_mut()
                                   .filter(|x| x.is_valid)
                                   .find(|x| x.tag == tag);
        if let Some(mut line) = result.as_mut() {
            line.last_visit = self.last_visit;
        }
        result 
    }

    // return the address of the victim 
    fn insert(&mut self, tag: u64, address: u64) -> Option<u64> {
        assert!(self.find(tag).is_none());

        self.last_visit += 1;

        let mut line = (match self.lines.iter_mut().find(|x| !x.is_valid) {
            Some(x) => x,
            None => match self.lines.iter_mut().min_by_key(|x| x.last_visit) {
                Some(x) => x,
                None => panic!("eviction failed"),
            },
        });

        let result = if line.is_dirty { Some(line.address) } else { None };

        *line = CacheLine {
            is_valid: true,
            is_dirty: false,
            last_visit: self.last_visit,
            tag,
            address,
        };

        result
    }
}

impl Cache {
    pub fn new(config: CacheConfig, lower: Box<dyn Storage>) -> Self {
        assert_eq!(config.capacity % config.line_size, 0);
        let num_lines = config.capacity / config.line_size;

        Self {
            stats: Default::default(),
            config,
            lower,
            line_mask: ((num_lines / config.associativity) - 1) * config.line_size,
            tag_mask: !(config.capacity / config.associativity - 1),
            lines: vec![CacheLines::new(config.associativity as usize); num_lines as usize],
        }
    }

    fn read(&mut self, address: u64) -> Duration {
        let lines =
            &mut self.lines[((address & self.line_mask) / self.config.line_size) as usize];
        let tag = address & self.tag_mask;

        match lines.find(tag) {
            Some(line) => self.config.latency,
            None => {
                self.stats.num_miss += 1;
                self.config.latency + self.lower.access(address, CacheOp::Read) +
                    match lines.insert(tag, address) {
                        Some(lower_address) => self.lower.access(lower_address, CacheOp::Write),
                        None => 0,
                    }
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
                line.is_dirty = true;
                self.config.latency
            },
            None => {
                self.stats.num_miss += 1;
                if self.config.write_allocate {
                    self.config.latency + self.lower.access(address, CacheOp::Read) +
                        match lines.insert(tag, address) {
                            Some(lower_address) => self.lower.access(lower_address, CacheOp::Write),
                            None => 0,
                        }
                } else {
                    self.config.latency + self.lower.access(address, CacheOp::Write)
                }
            }
        }
    }
}

#[test]
fn test001() {
    let llc = Box::new(Cache::new(
        CacheConfig {
            name: "test",
            write_through: false,
            write_allocate: true,
            capacity: 8 * 1024 * 1024,
            associativity: 8,
            line_size: 64,
            latency: 4,
        },
        Box::new(Dram::new(13))
    ));
    assert_eq!(llc.line_mask, 0xfffc0);
    assert_eq!(llc.tag_mask, !0xfffff);
}

impl Storage for Cache {
    fn access(&mut self, address: u64, op: CacheOp) -> Duration {
        let result = match op {
            CacheOp::Read => self.read(address),
            CacheOp::Write => self.write(address),
        };
        self.stats.num_access += 1;
        self.stats.time += result;
        result
    }

    fn output_stats(&self) {
        println!("{}:", self.config.name);
        println!("  {:?}", self.stats);
        println!("  miss rate: {}", self.stats.num_miss as f32 / self.stats.num_access as f32);
        self.lower.output_stats();
    }

    fn stats(&self) -> StorageStats {
        let StorageStats {
            num_access,
            num_miss,
            time,
        } = self.lower.stats();
        StorageStats {
            num_access: num_access + self.stats.num_access,
            num_miss: num_miss + self.stats.num_miss,
            time: time + self.stats.time,
        }
    }
}

impl Dram {
    fn new(latency: Duration) -> Self {
        Self {
            latency,
            stats: Default::default(),
        }
    }
}

impl Storage for Dram {

    fn access(&mut self, _address: u64, _op: CacheOp) -> Duration {
        self.latency
    }

    fn stats(&self) -> StorageStats {
        self.stats
    }
}


