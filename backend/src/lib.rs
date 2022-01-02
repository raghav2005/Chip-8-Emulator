// screen size will need to be accessed by frontend
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

// 4 KB of RAM
const RAM_SIZE: usize = 4096;
// 16 registers
const NO_OF_REGISTERS: usize = 16;
// 32 bytes of stack - paired in 2 bytes so 16
const STACK_SIZE: usize = 16;
// 16 keys for a Chip-8
const NO_OF_KEYS: usize = 16;

// class to manage emulator (main object - handles running the game + passes information back and forth from frontend)
pub struct Emulator {
    // program counter - keeps track of the index of the current instruction
    pc: u16,
    // Random Access Memory
    ram: [u8; RAM_SIZE],
    // display is monochromatic so can use a 1-bit display, so we're using bool
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    // V registers - V0 through VF
    v_registers: [u8; NO_OF_REGISTERS],
    // I register - indexes into RAM for reads and writes
    i_register: u16,
    // stack - LIFO, not general purpose, used when entering/exiting subroutine
    stack: [u16; STACK_SIZE],
    // delay timer - counts down every cycle and perform action at 0
    delay_timer: u8,
    // sound timer - counts down every cycle and emits sound at 0
    sound_timer: u8,
}