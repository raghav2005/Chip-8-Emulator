// TODO ONCE FINISHED: REPLACE STACK AND STACK POINTER USING VECTORS

use std::arch::x86_64::_SIDD_CMP_EQUAL_EACH;

// crates
use rand::random;

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
	
	// delay timer - counts down every frame (-1) and perform action at 0
	delay_timer: u8,
	// sound timer - counts down every frame (-1) and emits sound at 0
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

	// reset emulator without having to create a new object
	pub fn reset(&mut self) {
		self.pc = START_ADDRESS;
		self.ram = [0; RAM_SIZE];
		self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
		
		self.v_registers = [0; NO_OF_REGISTERS];
		self.i_register = 0;
		
		self.stack_pointer = 0;
		self.stack = [0; STACK_SIZE];
		
		self.keys = [false; NO_OF_KEYS];
		
		self.delay_timer = 0;
		self.sound_timer = 0;
		
		self.ram[..CHAR_SPRITE_ARR_SIZE].copy_from_slice(&CHAR_SPRITE_ARR);
	}

	// tick - 1 F-D-E cycle
	pub fn tick(&mut self) {
		// fetch
		let opcode: u16 = self.fetch();

		// decode & execute
		self.execute_opcode(opcode);
	}

	// modify timers every frame
	pub fn tick_timers(&mut self) {
		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}
		if self.sound_timer > 0 {
			if self.sound_timer == 1 {
				// TODO: BEEP audio - research on your own
			}
			self.sound_timer -= 1;
		}
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

	// fetch instruction / opcode we need to format - operands included in opcode for Chip-8
	fn fetch(&mut self) -> u16 {
		// Big Endian, so most significant bit is stored first
		let first_byte: u16 = self.ram[self.pc as usize] as u16;
		let second_byte: u16 = self.ram[(self.pc + 1) as usize] as u16;

		// left-shift by a byte, and | is the same as +
		let opcode: u16 = (first_byte << 8) | second_byte;
		self.pc += 2;

		opcode
	}

	// decode and execute each opcode / instruction
	fn execute_opcode(&mut self, opcode: u16) {
		// separate each hex "digit" for pattern matching
		// bitwise AND and right-shift for this
		let digit_1: u16 = (opcode & 0xF000) >> 12;
		let digit_2: u16 = (opcode & 0x0F00) >> 8;
		let digit_3: u16 = (opcode & 0x00F0) >> 4;
		let digit_4: u16 = opcode & 0x000F;

		// match all opcodes - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
		match (digit_1, digit_2, digit_3, digit_4) {
			
			// NOP
			(0, 0, 0, 0) => return,
			
			// CLS
			(0, 0, 0xE, 0) => {
				// reset screen to be empty
				self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
			},

			// RET
			(0, 0, 0xE, 0xE) => {
				// return to previous function so pop from stack
				let return_address: u16 = self.stack_pop();
				self.pc = return_address;
			},

			// JMP NNN
			(1, _, _, _) => {
				let new_address: u16 = opcode & 0xFFF;
				self.pc = new_address;
			},

			// CALL NNN
			(2, _, _, _) => {
				let new_address: u16 = opcode & 0xFFF;

				// add current pc to stack
				self.stack_push(self.pc);
				// jump to given address
				self.pc = new_address;
			},

			// SKIP VX == NN
			(3, _, _, _) => {
				// Rust requires array indexing to be done with usize
				let register_no: usize = digit_2 as usize;
				let new_address: u8 = (opcode & 0xFF) as u8;

				if self.v_registers[register_no] == new_address {
					// skip the next opcode
					self.pc += 2;
				}
			},

			// SKIP VX != NN
			(4, _, _, _) => {
				// Rust requires array indexing to be done with usize
				let register_no: usize = digit_2 as usize;
				let new_address: u8 = (opcode & 0xFF) as u8;

				if self.v_registers[register_no] != new_address {
					// skip to next opcode
					self.pc += 2;
				}
			},

			// SKIP VX == VY
			(5, _, _, 0) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;

				if self.v_registers[register_x] == self.v_registers[register_y] {
					// skip to next opcode
					self.pc += 2;
				}
			},

			// VX = NN
			(6, _, _, _) => {
				// Rust requires array indexing to be done with usize
				let register_no: usize = digit_2 as usize;
				let new_address: u8 = (opcode & 0xFF) as u8;
				self.v_registers[register_no] = new_address;
			},

			// VX += NN
			(7, _, _, _) => {
				// Rust requires array indexing to be done with usize
				let register_no: usize = digit_2 as usize;
				let new_address: u8 = (opcode & 0xFF) as u8;
				// use wrapping_add incase of overflow
				self.v_registers[register_no] = self.v_registers[register_no].wrapping_add(new_address);
			},

			// VX = VY
			(8, _, _, 0) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				self.v_registers[register_x] = self.v_registers[register_y];
			},

			// VX |= VY
			(8, _, _, 1) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				self.v_registers[register_x] |= self.v_registers[register_y];
			},

			// VX &= VY
			(8, _, _, 2) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				self.v_registers[register_x] &= self.v_registers[register_y];
			},

			// VX ^= VY
			(8, _, _, 3) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				self.v_registers[register_x] ^= self.v_registers[register_y];
			},

			// VX += VY
			(8, _, _, 4) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				
				// use overflowing_add to get tuple of addition and bool of
				// whether carrying a bit or not
				let (new_register_x, carry) = self.v_registers[register_x].overflowing_add(self.v_registers[register_y]);
				let new_register_f: u8 = if carry {1} else {0};

				self.v_registers[register_x] = new_register_x;
				self.v_registers[0xF] = new_register_f;
			},

			// VX -= VY
			(8, _, _, 5) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				
				// use overflowing_add to get tuple of addition and bool of
				// whether carrying a bit or not
				let (new_register_x, borrow) = self.v_registers[register_x].overflowing_sub(self.v_registers[register_y]);
				let new_register_f: u8 = if borrow {0} else {1};

				self.v_registers[register_x] = new_register_x;
				self.v_registers[0xF] = new_register_f;
			},

			// VX >>= 1
			(8, _, _, 6) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let bit_to_drop: u8 = self.v_registers[register_x] & 1;
				
				self.v_registers[register_x] >>= 1;
				self.v_registers[0xF] = bit_to_drop;
			},

			// VX = VY - VX
			(8, _, _, 7) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;
				
				// use overflowing_add to get tuple of addition and bool of
				// whether carrying a bit or not
				let (new_register_x, borrow) = self.v_registers[register_y].overflowing_sub(self.v_registers[register_x]);
				let new_register_f: u8 = if borrow {0} else {1};

				self.v_registers[register_x] = new_register_x;
				self.v_registers[0xF] = new_register_f;
			},

			// VX <<= 1
			(8, _, _, 0xE) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let bit_to_drop: u8 = (self.v_registers[register_x] >> 7) & 1;
				
				self.v_registers[register_x] <<= 1;
				self.v_registers[0xF] = bit_to_drop;
			},

			// SKIP VX != VY
			(9, _, _, 0) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_y: usize = digit_3 as usize;

				if self.v_registers[register_x] != self.v_registers[register_y] {
					// skip to next opcode
					self.pc += 2;
				}
			},

			// I = NNN
			(0xA, _, _, _) => {
				let new_address: u16 = opcode & 0xFFF;
				self.i_register = new_address;
			},

			// JMP V0 + NNN
			(0xB, _, _, _) => {
				let new_address: u16 = opcode & 0xFFF;
				self.pc = (self.v_registers[0] as u16) + new_address;
			},

			// VX = rand() & NN
			(0xC, _, _, _) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let new_address: u8 = (opcode & 0xFF) as u8;

				let random_number: u8 = random();

				self.v_registers[register_x] = random_number & new_address;
			},

			// DRAW
			(0xD, _, _, _) => {
				// get (x, y) co-ordinates for sprite
				let x_coord: u16 = self.v_registers[digit_2 as usize] as u16;
				let y_coord: u16 = self.v_registers[digit_3 as usize] as u16;
				// last digit is how many pixels tall the sprite is
				let col_height: u16 = digit_4;

				// keep track of flipped pixels
				let mut flipped_pixel: bool = false;

				// iterate over each row in sprite
				for each_row in 0..col_height {
					// determine memory address of row's stored data
					let address: u16 = self.i_register + each_row as u16;
					let pixel: u8 = self.ram[address as usize];

					// iterate over each column in the row
					for each_col in 0..8 {
						// use mask to fetch current pixel's bit
						// only flip if a 1
						if (pixel & (0b10000000) >> each_col) != 0 {
							// sprites wrap around screen so %
							let x_index: usize = (x_coord + each_col) as usize % SCREEN_WIDTH;
							let y_index: usize = (y_coord + each_row) as usize % SCREEN_HEIGHT;

							// get pixel's index for 1D screen array
							let pixel_index: usize = x_index + SCREEN_WIDTH * y_index;

							// check to flip pixel and set
							flipped_pixel |= self.screen[pixel_index];
							self.screen[pixel_index] ^= true;
						}
					}
				}

				// put necessary in VF register
				self.v_registers[0xF] = if flipped_pixel {1} else {0};
			},

			// SKIP KEY PRESS
			(0xE, _, 9, 0xE) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_vx: u8 = self.v_registers[register_x];
				
				// if index stored in VX is pressed
				let key: bool = self.keys[register_vx as usize];
				if key {
					// skip the next opcode
					self.pc += 2;
				}
			},

			// SKIP KEY RELEASE
			(0xE, _, 0xA, 1) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_vx: u8 = self.v_registers[register_x];
				
				// if index stored in VX is not pressed
				let key: bool = self.keys[register_vx as usize];
				if !key {
					// skip the next opcode
					self.pc += 2;
				}
			},

			// VX = DT
			(0xF, _, 0, 7) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				self.v_registers[register_x] = self.delay_timer;
			},
		
			// WAIT KEY
			(0xF, _, 0, 0xA) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let mut pressed: bool = false;

				// go through all keys
				for key in 0..self.keys.len() {
					// a key is pressed
					if self.keys[key] {
						self.v_registers[register_x] = key as u8;
						pressed = true;
						break;
					}
				}

				// do this until a key is pressed
				if !pressed {
					// redo opcode
					self.pc -= 2;
				}

				// this wasn't implemented in a loop because by being in a
				// loop, we would prevent the key press code from running,
				// causing the loop to never end
				// however, this is inefficient, so:
				// TODO: re-implement this with some sort of asynchronous checking
			},

			// DT = VX
			(0xF, _, 1, 5) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				self.delay_timer = self.v_registers[register_x];
			},

			// ST = VX
			(0xF, _, 1, 8) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				self.sound_timer = self.v_registers[register_x];
			},

			// I += VX
			(0xF, _, 1, 0xE) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let register_vx: u16 = self.v_registers[register_x] as u16;
				
				self.i_register = self.i_register.wrapping_add(register_vx);
			},

			// I = CHAR SPRITE
			(0xF, _, 2, 9) => {
				// Rust requires array indexing to be done with usize
				let register_x: usize = digit_2 as usize;
				let current: u16 = self.v_registers[register_x] as u16;

				self.i_register = current * 5;
			},

			// _ is a wildcard - won't run into this, but Rust requires it
			(_, _, _, _) => unimplemented!("{} opcode unimplemented", opcode),
		}
	}
}
