mod chip8;
mod constants;
mod debug;
mod display;
mod keyboard;
mod time;

extern crate clap;
extern crate sdl2;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::prelude::*;

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
                .help("Run in debug mode. Pauses after each instruction, prints info to stdout"),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true)
                .help("Integer display scale factor, defaults to 15"),
        )
        .arg(
            Arg::with_name("ips")
                .short("i")
                .long("ips")
                .takes_value(true)
                .help("Emulation speed in instructions per second, defaults to 1000"),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    // Read ROM
    println!("Reading ROM file: {}", filename);
    let mut f = File::open(filename).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Scaling
    let scl_str = matches.value_of("scale").unwrap_or("15");
    let scl_res = scl_str.parse::<u32>();
    let mut scale: u32 = 15;
    match scl_res {
        Ok(n) => scale = n,
        Err(e) => println!(
            "The scale ({}) is not a valid unsigned integer, using default: {}",
            scl_str, e
        ),
    }

    // Emulation speed
    let ips_str = matches.value_of("ips").unwrap_or("1000");
    let ips_res = ips_str.parse::<u128>();
    let mut ips: u128 = 1000;
    match ips_res {
        Ok(n) => ips = n,
        Err(e) => println!(
            "The ips ({}) is not a valid unsigned integer, using default: {}",
            ips_str, e
        ),
    }
    let instruction_time_ns: u128 = (1e9 as u128 / ips) as u128;

    // Start time
    let start: u128 = time::time_nanos();

    println!("R-CHIP-8 starting");

    // Create the display
    let mut display = Display::new("R-CHIP-8", scale);

    // Create the machine
    let debug_mode = matches.occurrences_of("debug") > 0;
    println!("Debug: {}", debug_mode);
    let mut chip8 = Chip8::new(rom, start, instruction_time_ns, debug_mode);

    // Main loop
    'mainloop: loop {
        let t: u128 = time::time_nanos();

        // Event loop
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::CapsLock),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        // Run the machine
        chip8.cycle(t, &mut display.event_pump);

        // Clear/update display if needed
        if chip8.display_clear_flag {
            display.clear();
        }
        if chip8.display_update_flag {
            display.render(chip8.display);
        }
    }
    println!("Bye!");
}
