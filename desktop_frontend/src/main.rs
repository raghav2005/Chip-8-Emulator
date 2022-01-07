// TODO: modify backend as well so keys has 1 more (escape) so can click escape on keyboard to quit game

// crates
use backend;
use sdl2;
use std::env;

// scale up 64x32 monitor
const SCALE_SIZE: u32 = 21;
// actual window width
const WINDOW_WIDTH: u32 = (backend::SCREEN_WIDTH as u32) * SCALE_SIZE;
// actual window height
const WINDOW_HEIGHT: u32 = (backend::SCREEN_HEIGHT as u32) * SCALE_SIZE;

fn main() {
	// get arguments from command line
	let arguments: Vec<_> = env::args().collect();

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

	let mut game_rom = std::fs::File::open(&arguments[1]).expect("Unable to open file.");


	'main_game_loop: loop {
		for event in event_pump.poll_iter() {
			match event {

				// clicks on red x button of window
				sdl2::event::Event::Quit{..} => {
					break 'main_game_loop;
				},

				// other undefined event
				_ => ()
			}
		}
	}
}
