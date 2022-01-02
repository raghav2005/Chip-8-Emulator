// TODO ONCE FINISHED: REPLACE STACK AND STACK POINTER USING VECTORS

// screen size will need to be accessed by frontend
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

// start address - ROM loaded in from 512th byte in RAM
const START_ADDRESS: u16 = 0x200;
// 4 KB of RAM
const RAM_SIZE: usize = 4096;
// 16 registers
const NO_OF_REGISTERS: usize = 16;
// 32 bytes of stack - paired in 2 bytes so 16
const STACK_SIZE: usize = 16;
// 16 keys for a Chip-8
const NO_OF_KEYS: usize = 16;

// 5 bytes per character, 16 characters, 5 * 16 = 80
const CHAR_SPRITE_ARR_SIZE: usize = 80;
// array of each character's font display values in hex
const CHAR_SPRITE_ARR: [u8; CHAR_SPRITE_ARR_SIZE] = [
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
	0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

// class to manage emulator (main object - handles running the game + passes information back and forth from frontend)
#[allow(dead_code)]
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
	// stack pointer to indicate where we are in stack (rather than using an
	// actual stack from the std lib as WebAssembly doesn't fully support std)
	stack_pointer: u16,
	// stack - LIFO, not general purpose, used when entering/exiting subroutine
	stack: [u16; STACK_SIZE],
	// keys/buttons of the chip-8 emulator
	keys: [bool; NO_OF_KEYS],
	// delay timer - counts down every cycle and perform action at 0
	delay_timer: u8,
	// sound timer - counts down every cycle and emits sound at 0
	sound_timer: u8,
}

impl Emulator {
	// new constructor for Emulator class    
	pub fn new() -> Self {
		let mut new_emulator = Self {
			pc: START_ADDRESS,
			ram: [0; RAM_SIZE],
			screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
			v_registers: [0; NO_OF_REGISTERS],
			i_register: 0,
			stack_pointer: 0,
			stack: [0; STACK_SIZE],
			keys: [false; NO_OF_KEYS],
			delay_timer: 0,
			sound_timer: 0,
		};

		new_emulator.ram[..CHAR_SPRITE_ARR_SIZE].copy_from_slice(&CHAR_SPRITE_ARR);

		new_emulator
	}

	// pushes a value to the stack and sets pointer to new element
	fn stack_push(&mut self, value_to_push: u16) {
		self.stack[self.stack_pointer as usize] = value_to_push;
		self.stack_pointer += 1;
	}

	// pops a value from the stack and sets pointer to previous element
	fn stack_pop(&mut self) -> u16 {
		self.stack_pointer -= 1;
		self.stack[self.stack_pointer as usize]
	}
}
