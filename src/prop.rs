extern crate sdl2;

use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;
use vector2::Vector2;

pub struct Prop {
	texture: Texture,
	position: Vector<f32>,
	bounding_box: Rect
}

pub fn new() -> Prop {
	Prop {

	}
}

impl Entity for Prop {
	fn update(&mut self, duration: Duration) { }

	fn draw(&self, canvas: Canvas<Window>) {

	}
}