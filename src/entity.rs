use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
	fn update(&mut self);

	fn draw(&self, canvas: &mut Canvas<Window>);
}