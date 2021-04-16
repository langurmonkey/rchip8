use crate::constants;
use crate::debug;
use crate::time;

// Models the CHIP-8 machine
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
    // Program counter
    pub pc: usize,
    // Delay timer: 8 b
    pub delay_timer: u8,
    // Sound timer: 8 b
    pub sound_timer: u8,
    // Display memory
    pub display: [u8; constants::DISPLAY_LEN],
    // Flag: update display (draw has been called)
    pub display_update_flag: bool,
    // Flag: clear display
    pub display_clear_flag: bool,
    // Flag: run in debug mode
    debug_mode: bool,
    // Last timer time
    last_timer_t: u128,
    // Last instruction time
    last_instruction_t: u128,
}

impl Chip8 {
    // Initializes the machine with the given ROM data [Vec<u8>] and start time [ns]
    pub fn initialize(rom: Vec<u8>, start_t: u128, debug_mode: bool) -> Self {
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
        let delay_timer: u8 = 0;
        // Sound timer: 8 b
        let sound_timer: u8 = 0;
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
            delay_timer,
            sound_timer,
            display,
            display_update_flag: false,
            display_clear_flag: false,
            debug_mode,
            last_timer_t: start_t,
            last_instruction_t: start_t,
        }
    }

    // Runs a CPU cycle with the given current time t [ns]
    pub fn cycle(&mut self, t: u128) {
        self.display_update_flag = false;
        self.display_clear_flag = false;
        // TIMERS
        // Decrement delay_timer and sound_timer 60 times per second
        // if their value is > 0
        if t - self.last_timer_t > 16_666 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.last_timer_t = t;
        }

        // LOG SPEED
        // let spf: f64 = (dt as f64) * 1.0e-9;
        // let fps: f64 = 1.0 / spf;
        // println!("Frame time: {} s, Frame rate: {} Hz", spf, fps);

        // INTERPRET

        if t - self.last_instruction_t > constants::INSTRUCTION_TIME_NS {
            if self.pc >= constants::RAM_SIZE {
                panic!("Reached the end!");
            }
            // RUN INSTRUCTION
            let instr: u16 = ((self.ram[self.pc] as u16) << 8) | self.ram[self.pc + 1] as u16;
            self.pc += 2;

            // INSTRUCTION: 0xIXYN // 0x00NN // 0x0NNN
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
                // 00E0 - CLS
                0x0000 => {
                    self.display.iter_mut().for_each(|m| *m = 0);
                    self.display_clear_flag = true;
                }
                // 1NNN - JMP
                0x1000 => self.pc = nnn as usize,
                // 6XNN - LD  VX, NN
                0x6000 => self.registers[x] = nn as u8,
                // 7XNN - ADD  VX, NN
                0x7000 => self.registers[x] += nn as u8,
                0x8000 => {
                    match n {
                        // 8XY0 - LD VX, VY
                        0 => self.registers[x] = self.registers[y],
                        // 8XY1 - OR VX, VY
                        1 => self.registers[x] = self.registers[x] | self.registers[y],
                        // 8XY2 - AND VX, VY
                        2 => self.registers[x] = self.registers[x] & self.registers[y],
                        // 8XY3 - XOR VX, VY
                        3 => self.registers[x] = self.registers[x] ^ self.registers[y],
                        // 8XY4 - ADD VX, VY
                        4 => {
                            let res = self.registers[x] as usize + self.registers[y] as usize;
                            if res > 255 {
                                // Carry to VF
                                self.registers[0x0F] = 1;
                            } else {
                                self.registers[0x0F] = 0;
                            }
                            self.registers[x] = res as u8;
                        }
                        // Default
                        _ => (),
                    }
                }
                // ANNN - LD  I, NNN
                0xA000 => self.index = nnn,
                // DXYN - DRW  VX, VY, N
                0xD000 => {
                    self.registers[0x0F] = 0;
                    let xpos: usize = self.registers[x] as usize % constants::DISPLAY_WIDTH;
                    let ypos: usize = self.registers[y] as usize % constants::DISPLAY_HEIGHT;
                    for row in 0..n {
                        // Fetch byte
                        let byte: u8 = self.ram[(self.index + row) as usize];
                        // Current Y
                        let cy = ypos + row as usize;
                        // Loop over bits
                        for col in 0..8_usize {
                            // Current X
                            let cx = xpos + col;
                            let pixel = self.display[cy * constants::DISPLAY_WIDTH + cx];
                            let mask: u8 = 0x01 << 7 - col;
                            if byte & mask != 0 {
                                // Bit is on
                                if pixel != 0 {
                                    // Display pixel is on
                                    self.display[cy * constants::DISPLAY_WIDTH + cx] = 0;
                                    self.registers[0x0F] = 1;
                                } else {
                                    // Display pixel is off
                                    self.display[cy * constants::DISPLAY_WIDTH + cx] = 1;
                                }
                            } else {
                                // Bit is off
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
                // Default
                _ => (),
            };

            self.last_instruction_t = t;
        }
    }
}
