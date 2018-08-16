extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector2;
use entity::Entity;

const SPEED: i32 = 1;

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
	pub fn move_left(&mut self) {
		self.velocity.x = -SPEED;
	}

	pub fn move_right(&mut self) {
		self.velocity.x = SPEED;
	}

	pub fn move_up(&mut self) {
		self.velocity.y = -SPEED;
	}

	pub fn move_down(&mut self) {
		self.velocity.y = SPEED;
	}

	pub fn halt_x_velocity(&mut self)
	{
		self.velocity.x = 0;
	}

	pub fn halt_y_velocity(&mut self)
	{
		self.velocity.y = 0;
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