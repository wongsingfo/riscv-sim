use std::{env, io};
use objdump::Elf;
use std::process::exit;
use crate::simulator::Simulator;
use std::io::Error;

mod memory;
mod simulator;
mod register;
mod instruction;
mod action;
mod statistic;
mod cache;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} binary", args[0]);
        exit(1)
    }

    let mut simulator = Simulator::new();
    simulator.load_from_elf(args[1].as_str());
    if args.len() == 2 {
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

    args[2..].iter().for_each(|s| {
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
