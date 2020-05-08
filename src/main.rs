use std::{env, io};
use objdump::Elf;
use std::process::exit;
use crate::simulator::Simulator;
use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use crate::cache::{CacheOp, Storage, CacheConfig};

mod memory;
mod simulator;
mod register;
mod instruction;
mod action;
mod statistic;
mod cache;

fn lab2_pipeline(args: &[String]) {
    if args.len() < 1 {
        eprintln!("unknown filename");
        exit(1);
    }

    let mut simulator = Simulator::new();
    simulator.load_from_elf(args[0].as_str());
    if args.len() == 1 {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match &input[..] {
                        "regs\n" => simulator.regs.println(),
                        "s\n" => { simulator.run(); }
                        "mem\n" => {
                            let mut input_text = String::new();
                            io::stdin()
                                .read_line(&mut input_text)
                                .expect("failed to read from stdin");
                            let trimmed = input_text.trim();
                            match u64::from_str_radix(trimmed, 16) {
                                Ok(i) => simulator.memory.println(i, 4),
                                Err(..) => println!("this was not a valid address: {}", trimmed),
                            }
                        }
                        _ => println!("unknown command")
                    }
                },
                Err(_) => {
                    break
                },
            }
        }
    }
    while simulator.run() {}

    args.iter().for_each(|s| {
        let res = simulator.elf.symbol_entries.iter()
            .filter(|x| {
                x.0.contains(s)
            }).next();
        match res {
            None => {
                println!("cannot find {}", s);
            },
            Some((s, start, size)) => {
                println!("{} 0x{:x} {}", s, start, size);
                simulator.memory.println(*start, *size as usize);
            },
        }
    });

    simulator.stat.println();
}

fn lab3_run(cache: &mut Box<dyn Storage>, filename: &String) -> cache::StorageStats {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        let mut iter = l.split_whitespace();
        let op = match iter.next().unwrap() {
            "r" => CacheOp::Read,
            "w" => CacheOp::Write,
            _ => {
                panic!("unknown op")
            }
        };
        let addr_s = iter.next().unwrap();
        let addr: u64 = if addr_s.starts_with("0x") {
            u64::from_str_radix(addr_s.trim_start_matches("0x"), 16).unwrap()
        } else {
            addr_s.parse::<u64>().unwrap()
        };
        cache.access(addr, op);
    }

    cache.stats()
}

fn lab3_cache(args: &[String]) {
    if args.len() < 1 {
        eprintln!("unknown filename");
        exit(1);
    }

    let mut cache = cache::new_3_levels();

    let cache::StorageStats {
        num_access,
        time,
        ..
    } = lab3_run(&mut cache, &args[0]);
    cache.output_stats();

    println!("AMAT: {}", time as f32 / num_access as f32);
}

fn lab3_cache1(args: &[String]) {
    if args.len() < 1 {
        eprintln!("unknown filename");
        exit(1);
    }

    let line_size: &'static [u64] = &[32, 64, 128, 256, 512, 1024, 2048, 4096];
    let cache_size: &'static [u64] = &[32, 128, 512, 2048, 8192, 32768];
    let associativity: &'static [u64] = &[1, 2, 4, 8, 16, 32];

    for c in cache_size {
        for l in line_size {
            let mut cache = cache::new_1_levels(CacheConfig {
                    name: "L1",
                    write_through: false,
                    write_allocate: true,
                    capacity: c * 1024,
                    associativity: 8,
                    line_size: *l,
                    latency: 3,
                });
            let result = lab3_run(&mut cache, &args[0]);
            print!("{}\t", result.num_miss as f32 / result.num_access as f32)
        }
        println!()
    }

    println!();

    for c in cache_size {
        for a in associativity {
            let mut cache = cache::new_1_levels(CacheConfig {
                    name: "L1",
                    write_through: false,
                    write_allocate: true,
                    capacity: c * 1024,
                    associativity: *a,
                    line_size: 512,
                    latency: 3,
                });
            let result = lab3_run(&mut cache, &args[0]);
            print!("{}\t", result.num_miss as f32 / result.num_access as f32)
        }
        println!()
    }

    println!();

    for b1 in &[true, false] {
        for b2 in &[true, false] {
            let mut cache = cache::new_1_levels(CacheConfig {
                    name: "L1",
                    write_through: *b2,
                    write_allocate: *b1,
                    capacity: 2 * 1024 * 1024,
                    associativity: 8,
                    line_size: 512,
                    latency: 3,
                });
            let result = lab3_run(&mut cache, &args[0]);
            print!("{}\t", result.time)
        }
        println!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [pipeline|cache|cache1]", args[0]);
        exit(1)
    }


    match args[1].as_str() {
        "cache" => lab3_cache(&args[2..]),
        "cache1" => lab3_cache1(&args[2..]),
        "pipeline" => lab2_pipeline(&args[2..]),
        _ => {
            eprintln!("Usage: {} [pipeline|cache]", args[0]);
            exit(1);
        },
    }
}
