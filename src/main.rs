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

use clap::{App, Arg};
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
                .help(&format!("Integer display scale factor, defaults to {} (for 640x320 upscaled resolution)", constants::DEF_SCALE)),
        )
        .arg(
            Arg::with_name("ips")
                .short("i")
                .long("ips")
                .takes_value(true)
                .help(&format!("Emulation speed in instructions per second, defaults to {}", constants::DEF_IPS)),
        )
        .arg(
            Arg::with_name("fgcol")
                .short("c")
                .long("fgcol")
                .takes_value(true)
                .help(&format!("Foreground (on) color as a hex code, defaults to {}", constants::DEF_FG_COL)),
        )
        .arg(
            Arg::with_name("bgcol")
                .short("b")
                .long("bgcol")
                .takes_value(true)
                .help(&format!("Background (off) color as a hex code, defaults to {}", constants::DEF_BG_COL)),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    // Read ROM
    println!("Reading ROM file: {}", filename);
    let mut f = File::open(filename).unwrap();
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();

    // Scaling
    let def_scale: &str = &constants::DEF_SCALE.to_string();
    let scl_str = matches.value_of("scale").unwrap_or(def_scale);
    let scl_res = scl_str.parse::<u32>();
    let mut scale: u32 = constants::DEF_SCALE;
    match scl_res {
        Ok(n) => scale = n,
        Err(e) => println!(
            "The scale ({}) is not a valid unsigned integer, using default: {}",
            scl_str, e
        ),
    }

    // Emulation speed
    let default_ips: &str = &constants::DEF_IPS.to_string();
    let ips_str = matches.value_of("ips").unwrap_or(default_ips);
    let ips_res = ips_str.parse::<u32>();
    let mut ips: u32 = constants::DEF_IPS;
    match ips_res {
        Ok(n) => ips = n,
        Err(e) => println!(
            "The ips ({}) is not a valid unsigned integer, using default: {}",
            ips_str, e
        ),
    }
    let instruction_time_ns: u128 = (1e9 as u128 / ips as u128) as u128;

    // Foreground color
    let fg_str = matches.value_of("fgcol").unwrap_or(constants::DEF_FG_COL);
    let fg = hex_to_col(fg_str);
    let fgcol = match fg {
        Ok(fgcol) => fgcol,
        Err(error) => {
            println!("{}", error);
            constants::DEF_FG
        }
    };

    // Background color
    let bg_str = matches.value_of("bgcol").unwrap_or(constants::DEF_BG_COL);
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
