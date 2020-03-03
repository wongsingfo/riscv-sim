use std::env;
use objdump::Elf;
use std::process::exit;

mod memory;
mod simulator;
mod register;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} binary", args[0]);
        exit(1)
    }

    let elf: Elf = Elf::open(args[1].as_str())
        .expect("can not open the binary file");


}
