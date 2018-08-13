extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl-test", 800, 600)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let player: Player = {
		Vector2::new(10, 200);
		Vector2::new(0, 0);
	};

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				}
				_ => {}
			}
		}
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		
		player.update();
		player.draw();

		canvas.present();
	}
}
