extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2;
use entity::Entity;

const SPEED: i32 = 5;

pub struct Player {
	position: vector2::Vector2,
	velocity: vector2::Vector2
}

pub fn new(position: vector2::Vector2) -> Player {
	Player {
		position: position,
		velocity: vector2::new(0, 0)
	}
}

impl Player {
	fn move_left(&mut self) {
		self.position.x -= SPEED;
	}

	pub fn move_right(&mut self) {
		self.position.x += SPEED;
	}

	fn move_up(&mut self) {
		self.position.y -= SPEED;
	}

	fn move_down(&mut self) {
		self.position.y += SPEED;
	}
}

impl Entity for Player {
	fn update(&mut self) {
		self.position.x += self.velocity.x;
		self.position.y += self.velocity.y;
	}

	fn draw(&self, canvas: &mut Canvas<Window>) {		
		canvas.set_draw_color(Color::RGB(255, 0, 255));
		canvas.fill_rect(rect::Rect::new(self.position.x, self.position.y, 50, 50));
	}
}