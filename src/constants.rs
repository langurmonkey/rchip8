pub const PROGRAM_LOC: usize = 0x200;

// RAM size in B
pub const RAM_SIZE: usize = 4096;
// Stack size in number of 16 b units
pub const STACK_SIZE: usize = 64;
// Number of registers
pub const N_REGISTERS: usize = 16;
// Display width in pixels
pub const DISPLAY_WIDTH: usize = 64;
// Display height in pixels
pub const DISPLAY_HEIGHT: usize = 32;
// Total number of pixels in display
pub const DISPLAY_LEN: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
