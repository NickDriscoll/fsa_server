extern crate sdl2;

mod vector2;
mod player;
mod entity;
mod command;
mod keyboard_manager;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use entity::Entity;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl-test", 800, 600)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	//Initialize keyboard manager
	let mut keyboard_manager = keyboard_manager::new(event_pump);

	let mut player: player::Player = player::new(vector2::new(100, 100));

	'running: loop {
		//Handle input
		keyboard_manager.handle_keyboard();

		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		
		player.update();
		player.draw(&mut canvas);

		canvas.present();
	}
}
