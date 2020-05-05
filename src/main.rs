use std::{env, io};
use objdump::Elf;
use std::process::exit;
use crate::simulator::Simulator;
use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use crate::cache::CacheOp;

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

fn lab3_cache(args: &[String]) {
    if args.len() < 1 {
        eprintln!("unknown filename");
        exit(1);
    }

    let file = File::open(args[0].as_str()).unwrap();
    let reader = BufReader::new(file);

    let mut cache = cache::new();

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

    cache.output_stats();
    let cache::StorageStats {
        num_access,
        time,
        ..
    } = cache.stats();

    println!("AMAT: {}", time as f32 / num_access as f32);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [pipeline|cache]", args[0]);
        exit(1)
    }


    match args[1].as_str() {
        "cache" => lab3_cache(&args[2..]),
        "pipeline" => lab2_pipeline(&args[2..]),
        _ => {
            eprintln!("Usage: {} [pipeline|cache]", args[0]);
            exit(1);
        },
    }
}
