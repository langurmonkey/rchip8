use crate::constants;
use crate::debug;
use crate::keyboard;

use rand::random;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::process;

// Emulates the CHIP-8 machine
pub struct Chip8 {
    // RAM memory: 4 kB
    pub ram: [u8; constants::RAM_SIZE],
    // Registers: 16 * 1 B
    pub registers: [u8; constants::N_REGISTERS],
    // Index register I: 16 b
    pub index: u16,
    // Stack of 16 b addresses
    pub stack: [u16; constants::STACK_SIZE],
    // Current index of the top of the stack
    pub istack: usize,
    // Program counter (PC)
    pub pc: usize,
    // Delay timer (DT): 8 b
    pub dt: u8,
    // Sound timer (ST): 8 b
    pub st: u8,
    // Display memory
    pub display: [u8; constants::DISPLAY_LEN],
    // Flag: update display (draw has been called)
    pub display_update_flag: bool,
    // Flag: clear display
    pub display_clear_flag: bool,
    // Flag: beep
    pub beep_flag: bool,

    // Emulation speed in instruction time [ns]
    instruction_time_ns: u128,
    // Flag: run in debug mode
    debug_mode: bool,
    // Last timer time
    last_timer_t: u128,
    // Last instruction time
    last_instruction_t: u128,
}

impl Chip8 {
    // Initializes the machine with the given ROM data [Vec<u8>] and start time [ns]
    pub fn new(rom: Vec<u8>, start_t: u128, instruction_time_ns: u128, debug_mode: bool) -> Self {
        // Initialize the machine

        // RAM memory: 4 kB
        let mut ram: [u8; constants::RAM_SIZE] = [0; constants::RAM_SIZE];
        // Registers: 16 * 1 B
        let registers: [u8; constants::N_REGISTERS] = [0; constants::N_REGISTERS];
        // Index register I: 16 b
        let index: u16 = 0;
        // Stack of 16 b addresses
        let stack: [u16; constants::STACK_SIZE] = [0; constants::STACK_SIZE];
        // Current index of the top of the stack
        let istack: usize = 0;
        // Program counter
        let pc: usize = constants::PROGRAM_LOC;
        // Delay timer: 8 b
        let dt: u8 = 0;
        // Sound timer: 8 b
        let st: u8 = 0;
        // Display memory
        let display: [u8; constants::DISPLAY_LEN] = [0; constants::DISPLAY_LEN];

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
        println!(
            "Fonts loaded into memory: {} b [0x{:04x}..0x{:04x}]",
            80, 0, 80
        );
        // Copy ROM to memory
        let bytes = rom.len();
        let ppos = constants::PROGRAM_LOC + bytes;
        ram[constants::PROGRAM_LOC..ppos].copy_from_slice(&rom[0..bytes]);
        println!(
            "ROM loaded into memory: {} b [0x{:04x}..0x{:04x}]",
            bytes,
            constants::PROGRAM_LOC,
            ppos
        );

        Chip8 {
            ram,
            registers,
            index,
            stack,
            istack,
            pc,
            dt,
            st,
            display,
            display_update_flag: false,
            display_clear_flag: false,
            beep_flag: false,
            instruction_time_ns,
            debug_mode,
            last_timer_t: start_t,
            last_instruction_t: start_t,
        }
    }

    // Runs a CPU cycle with the given current time t [ns]
    pub fn cycle(&mut self, t: u128, event_pump: &mut EventPump) {
        self.display_update_flag = false;
        self.display_clear_flag = false;
        // TIMERS
        // Decrement delay_timer and sound_timer 60 times per second
        // if their value is > 0
        if t - self.last_timer_t > 16_666_666 {
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                self.st -= 1;
                self.beep_flag = self.st > 0;
            }
            self.last_timer_t = t;
        }

        // INTERPRET
        if t - self.last_instruction_t > self.instruction_time_ns {
            if self.pc >= constants::RAM_SIZE {
                panic!("Reached the end!");
            }
            // RUN INSTRUCTION
            let instr: u16 = ((self.ram[self.pc] as u16) << 8) | self.ram[self.pc + 1] as u16;
            self.pc += 2;

            // INSTRUCTION: 0xIXYN with 0x000N, 0x00NN, 0x0NNN
            let code = instr & 0xF000;
            let x = ((instr & 0x0F00) >> 8) as usize;
            let y = ((instr & 0x00F0) >> 4) as usize;
            let n = instr & 0x000F;
            let nn = instr & 0x00FF;
            let nnn = instr & 0x0FFF;

            if self.debug_mode {
                debug::debug(
                    self.pc,
                    instr,
                    code,
                    x,
                    y,
                    n,
                    nn,
                    nnn,
                    self.registers,
                    self.index,
                );
            }

            match code {
                0x0000 => {
                    match n {
                        // 00E0 - CLS
                        0 => {
                            self.display.iter_mut().for_each(|m| *m = 0);
                            self.display_clear_flag = true;
                        }
                        // 00EE - RET
                        0x0E => {
                            self.pc = self.stack[self.istack] as usize;
                            if self.istack > 0 {
                                self.istack -= 1;
                            }
                        }
                        // Default
                        _ => (),
                    }
                }
                // 1NNN - JMP
                0x1000 => self.pc = nnn as usize,
                // 2NNN - CALL NNN
                0x2000 => {
                    self.istack += 1;
                    self.stack[self.istack] = self.pc as u16;
                    self.pc = nnn as usize;
                }
                // 3XNN - SE VX, NN
                0x3000 => {
                    if self.registers[x] as u16 == nn {
                        self.pc += 2;
                    }
                }
                // 4XNN - SNE VX, NN
                0x4000 => {
                    if self.registers[x] as u16 != nn {
                        self.pc += 2;
                    }
                }
                // 5XY0 - SE VX, VY
                0x5000 => {
                    if self.registers[x] == self.registers[y] {
                        self.pc += 2;
                    }
                }
                // 6XNN - LD  VX, NN
                0x6000 => self.registers[x] = nn as u8,
                // 7XNN - ADD  VX, NN
                0x7000 => self.registers[x] = (self.registers[x] as u16 + nn) as u8,
                0x8000 => {
                    match n {
                        // 8XY0 - LD VX, VY
                        0x00 => self.registers[x] = self.registers[y],
                        // 8XY1 - OR VX, VY
                        0x01 => self.registers[x] = self.registers[x] | self.registers[y],
                        // 8XY2 - AND VX, VY
                        0x02 => self.registers[x] = self.registers[x] & self.registers[y],
                        // 8XY3 - XOR VX, VY
                        0x03 => self.registers[x] = self.registers[x] ^ self.registers[y],
                        // 8XY4 - ADD VX, VY
                        0x04 => {
                            let res = self.registers[x] as usize + self.registers[y] as usize;
                            if res > 255 {
                                // Carry to VF
                                self.registers[0x0F] = 1;
                            } else {
                                self.registers[0x0F] = 0;
                            }
                            self.registers[x] = res as u8;
                        }
                        // 8XY5 - SUB VX, VY
                        0x05 => {
                            self.registers[0x0F] = if self.registers[x] > self.registers[y] {
                                // Carry to VF
                                1
                            } else {
                                0
                            };
                            self.registers[x] =
                                (self.registers[x] as i32 - self.registers[y] as i32) as u8;
                        }
                        // 8XY6 - SHR VX {, VY}
                        0x06 => {
                            self.registers[0x0F] = self.registers[x] & 0x01;
                            self.registers[x] /= 2;
                        }
                        // 8XY7 - SUBN VX, VY
                        0x07 => {
                            self.registers[0x0F] = if self.registers[y] > self.registers[x] {
                                1
                            } else {
                                0
                            };
                            self.registers[x] = self.registers[y] - self.registers[x];
                        }
                        // 8XYE - SHL VX {, VY}
                        0x0E => {
                            self.registers[0x0F] = self.registers[x] & 0x80;
                            self.registers[x] = (self.registers[x] as u16 * 2) as u8;
                        }
                        // Default
                        _ => (),
                    }
                }
                // 0x9XY0 - SNE VX, VY  (skip next instruction)
                0x9000 => {
                    if self.registers[x] != self.registers[y] {
                        self.pc += 2;
                    }
                }
                // ANNN - LD  I, NNN
                0xA000 => self.index = nnn,
                // BNNN - JMP  V0, NNN  (jump to nnn + V0)
                0xB000 => self.pc = nnn as usize + self.registers[0] as usize,
                // CXNN - RND VX, NN  (set VX = RANDOM_BYTE AND NN)
                0xC000 => self.registers[x] = nn as u8 & random::<u8>(),
                // DXYN - DRW  VX, VY, N
                0xD000 => {
                    self.registers[0x0F] = 0;
                    let xpos: usize = self.registers[x] as usize % constants::DISPLAY_WIDTH;
                    let ypos: usize = self.registers[y] as usize % constants::DISPLAY_HEIGHT;
                    for row in 0..n {
                        // Fetch bits
                        let bits: u8 = self.ram[(self.index + row) as usize];
                        // Current Y
                        let cy = (ypos + row as usize) % constants::DISPLAY_HEIGHT;
                        // Loop over bits
                        for col in 0..8_usize {
                            // Current X
                            let cx = (xpos + col) % constants::DISPLAY_WIDTH;
                            let current_color = self.display[cy * constants::DISPLAY_WIDTH + cx];
                            let mask: u8 = 0x01 << 7 - col;
                            let color = bits & mask;
                            // XOR
                            // 0 0 -> 0
                            // 0 1 -> 1
                            // 1 0 -> 1
                            // 1 1 -> 0
                            if color > 0 {
                                // color is on
                                if current_color > 0 {
                                    // current color is on
                                    self.display[cy * constants::DISPLAY_WIDTH + cx] = 0;
                                    self.registers[0x0F] = 1;
                                } else {
                                    // current color is off
                                    self.display[cy * constants::DISPLAY_WIDTH + cx] = 1;
                                }
                            } else {
                                // Bit is off
                                // Do nothing
                            }
                            if cx == constants::DISPLAY_WIDTH - 1 {
                                // Reached the right edge
                                break;
                            }
                        }
                        if cy == constants::DISPLAY_HEIGHT - 1 {
                            // Reached the bottom edge
                            break;
                        }
                    }
                    self.display_update_flag = true;
                }
                0xE000 => {
                    match nn {
                        // EX9E - SKP VX  (skip next instr if key with val VX is pressed)
                        0x9E => {
                            if event_pump
                                .keyboard_state()
                                .is_scancode_pressed(keyboard::map(self.registers[x]))
                            {
                                self.pc += 2;
                            }
                        }
                        // EXA1 - SKNP VX  (skip next instr if key with val VX is not pressed)
                        0xA1 => {
                            if !event_pump
                                .keyboard_state()
                                .is_scancode_pressed(keyboard::map(self.registers[x]))
                            {
                                self.pc += 2;
                            }
                        }
                        _ => (),
                    }
                }
                0xF000 => {
                    match nn {
                        // FX07 - LD VX, DT  (set VX = delay timer)
                        0x07 => self.registers[x] = self.dt,
                        // FX0A - LD VX, N  (wait for key press, store key value in VX)
                        0x0A => {
                            let keycode: u8 = loop {
                                let event = event_pump.wait_event();
                                let code = match event {
                                    Event::KeyDown {
                                        keycode: Some(code),
                                        ..
                                    } => Some(code),
                                    _ => None,
                                };
                                if code.is_some() {
                                    let sc = Scancode::from_keycode(code.unwrap()).unwrap();
                                    if sc == Scancode::Escape || sc == Scancode::CapsLock {
                                        // Terminate
                                        println!("Bye!");
                                        process::exit(0);
                                    }
                                    let c = keyboard::unmap(sc);
                                    if c.is_some() {
                                        break c.unwrap();
                                    }
                                }
                            };
                            self.registers[x] = keycode;
                        }
                        // FX15 - LD DT, VX  (set delay timer = VX)
                        0x15 => self.dt = self.registers[x],
                        // FX18 - LD ST, VX  (set sound timer = VX)
                        0x18 => self.st = self.registers[x],
                        // FX1E - ADD I, VX
                        0x1E => self.index = self.index + self.registers[x] as u16,
                        // FX29 - LD F, VX  (set I to location of sprite for digit VX)
                        0x29 => self.index = self.registers[x] as u16 * 0x05,
                        // FX33 - LD B, VX  (store BCD representation of VX in I, I+1 and I+2)
                        0x33 => {
                            let num = self.registers[x];
                            let h = num / 100;
                            let t = (num - h * 100) / 10;
                            let o = num - h * 100 - t * 10;
                            let i = self.index as usize;
                            self.ram[i] = h;
                            self.ram[i + 1] = t;
                            self.ram[i + 2] = o;
                        }
                        // FX55 - LD [I], VX  (set memory starting at I to values in V0 to VX)
                        0x55 => {
                            let n: usize = x;
                            for reg in 0..n + 1 {
                                self.ram[self.index as usize + reg] = self.registers[reg];
                            }
                        }
                        // FX65 - LD VX, [I]  (set registers V0 to VX to memory starting at I)
                        0x65 => {
                            let n: usize = x;
                            for reg in 0..n + 1 {
                                self.registers[reg] = self.ram[self.index as usize + reg];
                            }
                        }
                        _ => (),
                    }
                }
                // Default
                _ => (),
            };

            self.last_instruction_t = t;
        }
    }
}
