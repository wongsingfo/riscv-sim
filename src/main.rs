use std::env;
use objdump::Elf;
use std::process::exit;
use crate::simulator::Simulator;

mod memory;
mod simulator;
mod register;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} binary", args[0]);
        exit(1)
    }

    let mut simulator = Simulator::new();
    simulator.load_from_elf(args[1].as_str());
}
