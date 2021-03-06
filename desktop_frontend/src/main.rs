// crates
use backend::{self, Emulator, SCREEN_WIDTH};
use sdl2;
use std::io::Read;

// scale up 64x32 monitor
const SCALE_SIZE: u32 = 21;
// actual window width
const WINDOW_WIDTH: u32 = (backend::SCREEN_WIDTH as u32) * SCALE_SIZE;
// actual window height
const WINDOW_HEIGHT: u32 = (backend::SCREEN_HEIGHT as u32) * SCALE_SIZE;
// ticks per frame
const TICKS_PER_FRAME: usize = 20;

fn main() {
	// get arguments from command line
	let arguments: Vec<_> = std::env::args().collect();

	// must only have the game path, no other arguments
	if arguments.len() != 2 {
		println!("Usage: cargo run path_to_game");
		return;
	}

	// setup SDL2
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	// window for screen to be held in
	let window = video_subsystem.window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().opengl().build().unwrap();

	// actual screen for user
	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	// clear and display to user
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();

	// initialise an emulator object
	let mut chip8 = backend::Emulator::new();

	// load in ROM file, expect - if file doesn't exist
	let mut game_rom = std::fs::File::open(&arguments[1]).expect("Unable to open file.");
	// create buffer for game file
	let mut game_buffer = Vec::new();

	// load game from buffer to rom and chip8
	game_rom.read_to_end(&mut game_buffer).unwrap();
	chip8.load_rom(&game_buffer);

	'main_game_loop: loop {
		for event in event_pump.poll_iter() {
			match event {

				// clicks on red x button of window
				sdl2::event::Event::Quit{..} | sdl2::event::Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => {
					break 'main_game_loop;
				},

				// press a key down
				sdl2::event::Event::KeyDown{keycode: Some(key), ..} => {
					if let Some(k) = key_to_btn(key) {
						chip8.key_press(k, true);
					}
				},

				// lift a key up
				sdl2::event::Event::KeyUp{keycode: Some(key), ..} => {
					if let Some(k) = key_to_btn(key) {
						chip8.key_press(k, false);
					}
				},

				// other undefined event
				_ => ()
			}

			// ticks required during 1 frame
			for _ in 0..TICKS_PER_FRAME {
				chip8.tick();
			}

			// tick both timers
			chip8.tick_timers();
			// update screen
			draw_screen(&chip8, &mut canvas);
		}
	}
}

// clear screen by setting all to black, etc.
fn draw_screen(emulator: &Emulator, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
	// clear canvas as black
	canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
	canvas.clear();

	let screen_buffer = emulator.get_display();

	// set draw colour to white
	canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
	// iterate through each point to see if it should be drawn on
	for (i, pixel) in screen_buffer.iter().enumerate() {
		if *pixel {
			// convert 1D array index into a 2D (x, y) coordinate position
			let x: u32 = (i % SCREEN_WIDTH) as u32;
			let y: u32 = (i / SCREEN_WIDTH) as u32;

			// draw a rectangle at point (x, y), but scaled up
			let rectangle = sdl2::rect::Rect::new((x * SCALE_SIZE) as i32, (y * SCALE_SIZE) as i32, SCALE_SIZE, SCALE_SIZE);
			canvas.fill_rect(rectangle).unwrap();
		}
	}

	canvas.present();
}

// take in a Keycode and output an optional u8 value
fn key_to_btn(key: sdl2::keyboard::Keycode) -> Option<usize> {
	// switch
	match key {
		sdl2::keyboard::Keycode::Num1 => Some(0x1),
		sdl2::keyboard::Keycode::Num2 => Some(0x2),
		sdl2::keyboard::Keycode::Num3 => Some(0x3),
		sdl2::keyboard::Keycode::Num4 => Some(0xC),
		sdl2::keyboard::Keycode::Q => Some(0x4),
		sdl2::keyboard::Keycode::W => Some(0x5),
		sdl2::keyboard::Keycode::E => Some(0x6),
		sdl2::keyboard::Keycode::R => Some(0xD),
		sdl2::keyboard::Keycode::A => Some(0x7),
		sdl2::keyboard::Keycode::S => Some(0x8),
		sdl2::keyboard::Keycode::D => Some(0x9),
		sdl2::keyboard::Keycode::F => Some(0xE),
		sdl2::keyboard::Keycode::Z => Some(0xA),
		sdl2::keyboard::Keycode::X => Some(0x0),
		sdl2::keyboard::Keycode::C => Some(0xB),
		sdl2::keyboard::Keycode::V => Some(0xF),
		_ => None,
	}
}
