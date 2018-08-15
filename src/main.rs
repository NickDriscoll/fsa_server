extern crate sdl2;

mod vector2;
mod player;
mod entity;
mod command;
mod keyboard_manager;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use entity::Entity;
use std::vec::Vec;
use std::cell;
use command::Command;
use command::quit_command;
use command::move_right_command;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("FSA clone", 1280, 720)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	//Create player
	let player = cell::RefCell::new(player::new(vector2::new(100, 200)));

	//Initialize keyboard manager
	let mut keyboard_manager = keyboard_manager::new(&mut event_pump);

	//Add mapping to close the game when you presss Escape
	keyboard_manager.add_binding(Keycode::Escape, Command::Quit(quit_command::new()));
	keyboard_manager.add_binding(Keycode::Q, Command::Quit(quit_command::new()));
	keyboard_manager.add_binding(Keycode::Right, Command::MoveRight(move_right_command::new(&player)));

	//Initialize vector of entities
	let mut entities: Vec<&cell::RefCell<Entity>> = Vec::new();

	//Add player to entities
	entities.push(&player);

	loop {
		//Handle input
		keyboard_manager.handle_input();

		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		
		//Update entities
		for entity in entities.iter_mut() {
			entity.borrow_mut().update();
		}

		//Draw entities
		for entity in entities.iter() {
			entity.borrow().draw(&mut canvas);
		}

		canvas.present();
	}
}
