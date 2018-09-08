extern crate sdl2;

use sdl2::image;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use vector2;
use std::time::Duration;
use entity::Entity;

pub struct Tile<'a> {
	position: vector2::Vector2<f32>,
	texture: &'a Texture<'a>
}

pub fn new<'a>(x: f32, y: f32, t: &'a Texture<'a>) -> Tile<'a> {
	Tile {
		position: vector2::new(x, y),
		texture: t
	}
}

impl<'a> Entity for Tile<'a> {
	fn update(&mut self, elapsed: Duration) { }

	fn draw(&self, canvas: &mut Canvas<Window>)	{
		canvas.copy(self.texture, None, None);
	}
}