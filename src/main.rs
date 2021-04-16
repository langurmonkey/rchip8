mod chip8;
mod constants;
mod debug;
mod display;
mod time;

extern crate clap;
extern crate sdl2;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

use chip8::Chip8;
use display::Display;

// Simple CHIP8 emulator
// MIT license

fn main() {
    let matches = App::new("R-CHIP-8")
        .version("0.1.0")
        .author("Toni Sagrsità Sellés <me@tonisagrista.com>")
        .about("CHIP-8 emulator")
        .arg(
            Arg::with_name("input")
                .required(true)
                .index(1)
                .help("ROM file to load and run"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("Run in debug mode - pauses after each instruction, prints info to stdout"),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    // Read ROM
    println!("Reading ROM file: {}", filename);
    let mut f = File::open(filename).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Start time
    let start: u128 = time::time_nanos();

    // Create the display
    let mut display = Display::initialize("R-CHIP-8");
    display.clear();

    // Create the machine
    let debug_mode = matches.occurrences_of("debug") > 0;
    println!("Debug: {}", debug_mode);
    let mut chip8 = Chip8::initialize(rom, start, debug_mode);

    // Main loop
    'mainloop: loop {
        let t: u128 = time::time_nanos();
        // Run the machine
        chip8.cycle(t);

        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        // Update display if needed
        if chip8.display_flag {
            display.render(chip8.display);
        }
    }
}
