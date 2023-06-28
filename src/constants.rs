// Starting address of user programs
pub const PROGRAM_LOC: usize = 0x200;

// Default foreground color
pub const DEF_FG_COL: &str = "ABAECB";
pub const DEF_FG: (u8, u8, u8) = (171, 171, 203);
// Default background color
pub const DEF_BG_COL: &str = "101020";
pub const DEF_BG: (u8, u8, u8) = (16, 16, 32);
// Default number of instructions per second
pub const DEF_IPS_STR: &str = "1000";
// Default screen scale factor
pub const DEF_SCALE_STR: &str = "10";

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
