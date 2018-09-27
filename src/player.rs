use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use vector2;
use entity::Entity;
use level_parser::EntityType;
use command::Command;
use move_right_command::MoveRightCommand;

const SPEED: f32 = 400.0;

pub struct Player {
	position: vector2::Vector2<f32>,
	velocity: vector2::Vector2<f32>
}

pub fn new(position: vector2::Vector2<f32>) -> Player {
	Player {
		position: position,
		velocity: vector2::new(0.0, 0.0)
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
		self.velocity.x = 0.0;
	}

	pub fn halt_y_velocity(&mut self)
	{
		self.velocity.y = 0.0;
	}
}

impl Entity for Player {
	fn update(&mut self, elapsed: Duration) {
		self.position.x += self.velocity.x as f32 * (elapsed.subsec_millis() as f32 / 1000.0);
		self.position.y += self.velocity.y as f32 * (elapsed.subsec_millis() as f32 / 1000.0);
	}

	fn draw(&self, canvas: &mut Canvas<Window>) {		
		canvas.set_draw_color(Color::RGB(255, 0, 255));
		canvas.fill_rect(rect::Rect::new(self.position.x as i32, self.position.y as i32, 25, 25));
	}

	fn get_entity_type(&self) -> &EntityType {
		&EntityType::Player
	}

	fn get_position(&self) -> &vector2::Vector2<f32> {
		&self.position
	}

	fn handle_command(&mut self, command: Command) {
		match command {
			MoveRight => {
				self.move_right();
			}
			_ => { }
		}
	}
}
