extern crate sdl2;

mod vector2;
mod player;
mod entity;
mod command;
mod keyboard_manager;
mod event_handler;
mod network_manager;
mod tile;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::image;
use sdl2::image::LoadTexture;
use entity::Entity;
use std::vec::Vec;
use std::cell;
use std::time::Instant;
use command::Command;
use command::quit_command;
use command::move_right_command;
use command::move_left_command;
use command::move_up_command;
use command::move_down_command;
use command::halt_x_command;
use command::halt_y_command;
use network_manager::TouchButtons;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let image_context = image::init(image::INIT_PNG).unwrap();

	let window = video_subsystem.window("FSA", 1280, 720)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();
	let texture_creator = canvas.texture_creator();

	//Create player
	//Consider array of players parallel to the bitmask_maps
	let player = cell::RefCell::new(player::new(vector2::new(100.0, 200.0)));

	let background_texture = texture_creator.load_texture("assets/grass.png").unwrap();

	//Initialize keyboard manager
	let mut keyboard_manager = keyboard_manager::new();

	//Add mapping to close the game when you presss Escape
	keyboard_manager.add_keydown_binding(Keycode::Escape, Command::Quit(quit_command::new()));
	keyboard_manager.add_keydown_binding(Keycode::Q, Command::Quit(quit_command::new()));
	keyboard_manager.add_keydown_binding(Keycode::Right, Command::MoveRight(move_right_command::new(&player)));
	keyboard_manager.add_keydown_binding(Keycode::Left, Command::MoveLeft(move_left_command::new(&player)));
	keyboard_manager.add_keydown_binding(Keycode::Up, Command::MoveUp(move_up_command::new(&player)));
	keyboard_manager.add_keydown_binding(Keycode::Down, Command::MoveDown(move_down_command::new(&player)));

	keyboard_manager.add_keyup_binding(Keycode::Up, Command::HaltY(halt_y_command::new(&player)));
	keyboard_manager.add_keyup_binding(Keycode::Down, Command::HaltY(halt_y_command::new(&player)));
	keyboard_manager.add_keyup_binding(Keycode::Left, Command::HaltX(halt_x_command::new(&player)));
	keyboard_manager.add_keyup_binding(Keycode::Right, Command::HaltX(halt_x_command::new(&player)));

	//Initialize network manager
	let mut network_manager = network_manager::begin_listening();

	//Add the network bindings
	network_manager.add_touchdown_binding(TouchButtons::Left as u8, Command::MoveLeft(move_left_command::new(&player)), 1);
	network_manager.add_touchdown_binding(TouchButtons::Down as u8, Command::MoveDown(move_down_command::new(&player)), 1);
	network_manager.add_touchdown_binding(TouchButtons::Up as u8, Command::MoveUp(move_up_command::new(&player)), 1);
	network_manager.add_touchdown_binding(TouchButtons::Right as u8, Command::MoveRight(move_right_command::new(&player)), 1);

	network_manager.add_touchup_binding(TouchButtons::Left as u8, Command::HaltX(halt_x_command::new(&player)), 1);
	network_manager.add_touchup_binding(TouchButtons::Right as u8, Command::HaltX(halt_x_command::new(&player)), 1);
	network_manager.add_touchup_binding(TouchButtons::Down as u8, Command::HaltY(halt_y_command::new(&player)), 1);
	network_manager.add_touchup_binding(TouchButtons::Up as u8, Command::HaltY(halt_y_command::new(&player)), 1);

	//Initialize event handler
	let mut event_handler = event_handler::new(event_pump, &mut keyboard_manager);

	//Initialize vector of entities
	let mut entities: Vec<&cell::RefCell<Entity>> = Vec::new();

	//Add player to entities
	entities.push(&player);

	let mut previous_instant = Instant::now();

	loop {
		let current_instant = Instant::now();

		//Handle events
		event_handler.handle_events();

		//Handle network input
		network_manager.handle_input();

		//Clear the screen
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		
		//Update entities
		for entity in entities.iter_mut() {
			entity.borrow_mut().update(current_instant.duration_since(previous_instant));
		}

		//Draw background
		canvas.copy(&background_texture, None, None);

		//Draw entities
		for entity in entities.iter() {
			entity.borrow().draw(&mut canvas);
		}

		canvas.present();
		previous_instant = current_instant;
	}
}
