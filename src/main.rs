mod chip8;
mod constants;
mod debug;
mod time;

use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

use chip8::Chip8;

// Simple CHIP8 emulator
// MIT license

fn print_help() {
    println!("Usage: rchip8 [ROM_FILE]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        process::exit(1);
    }
    // Read ROM
    println!("Reading ROM file: {}", args[1]);
    let mut f = File::open(&args[1]).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Create the machine
    let mut chip8 = Chip8::initialize(rom);
    // Run the machine
    chip8.run();
}
