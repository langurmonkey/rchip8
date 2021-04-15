use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, process};

mod debug;

// Simple CHIP8 emulator
// MIT license

// Emulation speed in instructions per second
const SPEED_INSTRUCT_SEC: u128 = 700;
const INSTRUCTION_TIME_NS: u128 = (1e9 as u128 / SPEED_INSTRUCT_SEC) as u128;

const PROGRAM_LOC: usize = 0x200;

// RAM size in B
const RAM_SIZE: usize = 4096;
// Stack size in number of 16 b units
const STACK_SIZE: usize = 64;
// Number of registers
const N_REGISTERS: usize = 16;
// Display width in pixels
const DISPLAY_WIDTH: usize = 64;
// Display height in pixels
const DISPLAY_HEIGHT: usize = 32;
// Total number of pixels in display
const DISPLAY_LEN: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

fn print_help() {
    println!("Usage: rchip8 [ROM_FILE]");
}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        process::exit(1);
    }

    // Initialize the machine

    // RAM memory: 4 kB
    let mut ram: [u8; RAM_SIZE] = [0; RAM_SIZE];
    // Registers: 16 * 1 B
    let mut registers: [u8; N_REGISTERS] = [0; N_REGISTERS];
    // Index register I: 16 b
    let mut index: u16 = 0;
    // Stack of 16 b addresses
    let mut stack: [u16; STACK_SIZE] = [0; STACK_SIZE];
    // Current index of the top of the stack
    let mut istack: usize = 0;
    // Program counter
    let mut pc: usize = PROGRAM_LOC;
    // Delay timer: 8 b
    let mut delay_timer: u8 = 0;
    // Sound timer: 8 b
    let mut sound_timer: u8 = 0;
    // Display memory
    let mut display: [u8; DISPLAY_LEN] = [0; DISPLAY_LEN];

    // Initialize the fonts
    let fonts: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    ram[..80].copy_from_slice(&fonts);
    println!("Fonts loaded into memory: {} b [{}..{}]", 80, 0, 80);

    // Read ROM
    println!("Reading ROM file: {}", args[1]);
    let mut f = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();

    // Read the whole file
    let bytes = f.read_to_end(&mut buffer).unwrap();
    // Copy to memory
    let ppos = PROGRAM_LOC + bytes;
    ram[PROGRAM_LOC..ppos].copy_from_slice(&buffer[0..bytes]);
    println!(
        "Program loaded into memory: {} b [{}..{}]",
        bytes, PROGRAM_LOC, ppos
    );

    // Internal variables
    let start: u128 = time_nanos();
    let mut last_t = start;
    let mut last_timer_t = start;
    let mut last_instruction_t = start;

    // Main loop
    loop {
        let t: u128 = time_nanos();
        let dt: u128 = t - last_t;

        // TIMERS
        // Decrement delay_timer and sound_timer 60 times per second
        // if their value is > 0
        if t - last_timer_t > 16_666 {
            if delay_timer > 0 {
                delay_timer -= 1;
            }
            if sound_timer > 0 {
                sound_timer -= 1;
            }
            last_timer_t = t;
        }

        // LOG SPEED
        // let spf: f64 = (dt as f64) * 1.0e-9;
        // let fps: f64 = 1.0 / spf;
        // println!("Frame time: {} s, Frame rate: {} Hz", spf, fps);

        // INTERPRET

        if t - last_instruction_t > INSTRUCTION_TIME_NS {
            if pc >= RAM_SIZE {
                panic!("Reached the end!");
            }
            // RUN INSTRUCTION
            let inst: u16 = ((ram[pc] as u16) << 8) | ram[pc + 1] as u16;
            pc += 2;

            let code = inst & 0xF000;
            let x = ((inst & 0x0F00) >> 8) as usize;
            let y = ((inst & 0x00F0) >> 4) as usize;
            let n = inst & 0x000F;
            let nn = inst & 0x00FF;
            let nnn = inst & 0x0FFF;

            println!("pc:          {} (0x{:04x})", pc - 2, pc - 2);
            println!("{}", debug::debug_instr(code, x, y, n, nn, nnn));
            println!("instr:       0x{:04x}", inst);
            println!("code:        0x{:04x}", code);
            println!("x:           0x{:04x} ({})", x, x);
            println!("y:           0x{:04x} ({})", y, y);
            println!("n:           0x{:04x} ({})", n, y);
            println!("nn:          0x{:04x} ({})", nn, nn);
            println!("nnn:         0x{:04x} ({})", nnn, nnn);
            pause();

            match code {
                // 00E0 - clear screen
                0x0000 => display.iter_mut().for_each(|m| *m = 0),
                // 1NNN - jump
                0x1000 => pc = nnn as usize,
                // 6XNN - set register VX to NN
                0x6000 => registers[x] = nn as u8,
                // 7XNN - add NN to register VX
                0x7000 => registers[x] += nn as u8,
                0x800 => {
                    match n {
                        // 8XY0 - VX := VY
                        0 => registers[x] = registers[y],
                        // 8XY1 - VX := VX OR VY
                        1 => registers[x] = registers[x] | registers[y],
                        // 8XY2 - VX := VX AND VY
                        2 => registers[x] = registers[x] & registers[y],
                        // 8XY3 - VX := VX XOR VY
                        3 => registers[x] = registers[x] ^ registers[y],
                        // 8XY4 - VX := VX + VY
                        4 => registers[x] = registers[x] + registers[y],
                        // Default
                        _ => (),
                    }
                }
                // ANNN - set index register to NNN
                0xA000 => index = nnn,
                // DXYN - display/draw
                0xD000 => {
                    let mut xpos: usize = registers[x] as usize % DISPLAY_WIDTH;
                    let mut ypos: usize = registers[y] as usize % DISPLAY_HEIGHT;
                    let pixel = display[y * DISPLAY_WIDTH + x];
                    registers[0x0F] = 0;
                    for _ in 0..n {
                        let byte: u8 = ram[index as usize];
                        for bit in 0..8 {
                            let mask: u8 = 1 << bit;
                            if byte & mask != 0 {
                                // Bit is on
                                if pixel != 0 {
                                    // Display pixel is on
                                    display[y * DISPLAY_WIDTH + x] = 0;
                                    registers[0x0F] = 1;
                                } else {
                                    // Display pixel is off
                                    display[y * DISPLAY_WIDTH + x] = 1;
                                }
                            } else {
                                // Bit is off
                            }
                            if xpos == DISPLAY_WIDTH - 1 {
                                // Reached the right edge
                                break;
                            }
                            xpos += 1;
                        }
                        if ypos == DISPLAY_HEIGHT - 1 {
                            // Reached the bottom edge
                            break;
                        }
                        ypos += 1;
                    }
                }
                // Default
                _ => (),
            };

            last_instruction_t = t;
        }

        last_t = t;
    }
}

fn time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
