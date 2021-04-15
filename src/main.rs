mod chip8;
mod constants;
mod debug;
mod time;

extern crate sdl2;

use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    //canvas.present();

    // Run the machine
    chip8.run();
}
