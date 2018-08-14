extern crate sdl2;

mod vector2;
mod player;
mod entity;
mod keyboard_manager;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use entity::Entity;
use std::vec::Vec;
use std::process; //Remove this when actual quit command is implemented

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl-test", 800, 600)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();

	//Initialize keyboard manager
	let mut keyboard_manager = keyboard_manager::new(event_pump);

	//Add mapping to close the game when you presss Escape
	keyboard_manager.add_binding(Keycode::Escape, || { process::exit(0); });

	//Create player
	let mut player = player::new(vector2::new(100, 200));

	//Initialize vector of entities
	let mut entities: Vec<&mut Entity> = Vec::new();

	//Add player to entities
	entities.push(&mut player);
	'running: loop {
		//Handle input
		keyboard_manager.handle_keyboard();

		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		
		//Update entities
		for entity in entities.iter_mut() {
			entity.update();
		}

		//Draw entities
		for entity in entities.iter() {
			entity.draw(&mut canvas);
		}

		canvas.present();
	}
}
