use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub trait Entity {
	fn update(&mut self, elapsed: Duration);

	fn draw(&self, canvas: &mut Canvas<Window>);
}
