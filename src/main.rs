mod audio;
mod chip8;
mod constants;
mod debug;
mod display;
mod keyboard;
mod time;

extern crate clap;
extern crate hex;
extern crate sdl2;

use clap::{Arg, Command};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::prelude::*;

use audio::Beep;
use chip8::Chip8;
use display::Display;

// Simple CHIP8 emulator
// MIT license

fn main() {
    let matches = Command::new("R-CHIP-8")
        .version("0.1.0")
        .author("Toni Sagrsità Sellés <me@tonisagrista.com>")
        .about("CHIP-8 emulator")
        .arg(
            Arg::new("input")
                .required(true)
                .index(1)
                .help("ROM file to load and run")
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::SetTrue)
                .help("Run in debug mode. Pauses after each instruction, prints info to stdout")
        )
        .arg(
            Arg::new("scale")
                .short('s')
                .long("scale")
                .value_parser(clap::value_parser!(u32))
                .default_value(constants::DEF_SCALE_STR)
                .help(&format!("Unsigned integer display scale factor, defaults to {} (for 640x320 upscaled resolution)", constants::DEF_SCALE_STR))
        )
        .arg(
            Arg::new("ips")
                .short('i')
                .long("ips")
                .value_parser(clap::value_parser!(u32))
                .default_value(constants::DEF_IPS_STR)
                .help(&format!("Emulation speed in instructions per second, defaults to {}", constants::DEF_IPS_STR))
        )
        .arg(
            Arg::new("fgcol")
                .short('c')
                .long("fgcol")
                .default_value(constants::DEF_FG_COL)
                .help(&format!("Foreground (on) color as a hex code, defaults to {}", constants::DEF_FG_COL))
        )
        .arg(
            Arg::new("bgcol")
                .short('b')
                .long("bgcol")
                .default_value(constants::DEF_BG_COL)
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help(&format!("Background (off) color as a hex code, defaults to {}", constants::DEF_BG_COL))
        )
        .get_matches();

    let filename = matches.get_one::<String>("input").unwrap();
    // Read ROM
    println!("Reading ROM file: {}", filename);
    let mut f = File::open(filename).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Scaling
    let scale: u32 = *matches.get_one("scale").expect("required");

    // Emulation speed
    let ips: u32 = *matches.get_one("ips").expect("required");
    let instruction_time_ns: u128 = (1e9 as u128 / ips as u128) as u128;

    // Foreground color
    let fg_str = matches.get_one::<String>("fgcol").unwrap();
    let fg = hex_to_col(fg_str);
    let fgcol = match fg {
        Ok(fgcol) => fgcol,
        Err(error) => {
            println!("{}", error);
            constants::DEF_FG
        }
    };

    // Background color
    let bg_str = matches.get_one::<String>("bgcol").unwrap();
    let bg = hex_to_col(bg_str);
    let bgcol = match bg {
        Ok(bgcol) => bgcol,
        Err(error) => {
            println!("{}", error);
            constants::DEF_BG
        }
    };

    // Start time
    let start: u128 = time::time_nanos();

    println!("R-CHIP-8 starting");

    // Init SDL2
    let sdl_context = sdl2::init().unwrap();

    // Create the display
    let mut display = Display::new(&sdl_context, "R-CHIP-8", scale, fgcol, bgcol);

    // Create audio beep
    let beep = Beep::new(&sdl_context);

    // Create the machine
    let debug_mode: &bool = matches.get_one("debug").unwrap();
    println!("Debug: {}", debug_mode);
    let mut chip8 = Chip8::new(rom, start, instruction_time_ns, *debug_mode);

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

        // Play/pause the beep
        if chip8.beep_flag {
            beep.play();
        } else {
            beep.pause();
        }
    }
    println!("Bye!");
}

fn hex_to_col(hexcol: &str) -> Result<(u8, u8, u8), String> {
    let r = hex_to_u8(&hexcol[..2]);
    let g = hex_to_u8(&hexcol[2..4]);
    let b = hex_to_u8(&hexcol[4..]);

    if r.is_err() || g.is_err() || b.is_err() {
        return Err(format!(
            "Error converting hex '{}' to decimal, using default",
            hexcol
        ));
    }

    Ok((r.unwrap(), g.unwrap(), b.unwrap()))
}

fn hex_to_u8(hexbyte: &str) -> Result<u8, String> {
    match hex::decode(hexbyte) {
        Ok(val) => Ok(*val.get(0).unwrap()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
