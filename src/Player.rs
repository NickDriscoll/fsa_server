extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect;
use Vector2::Vector2;

const SPEED: i32 = 5;

struct Player {
	position: Vector2,
	velocity: Vector2
}

impl Player {
	fn new(pos: Vector2, vel: Vector2) {
		
	}

	fn move_left(&self) {
		position.x -= SPEED;
	}

	fn move_right(&self) {
		position.x += SPEED;
	}

	fn move_up(&self) {
		position.y -= SPEED;
	}

	fn move_down(&self) {
		position.y += SPEED;
	}
}

impl Entity for Player {
	fn update() {
		
	}

	fn draw(canvas: Canvas) {		
		canvas.set_draw_color(Color::RGB(255, 0, 255));
		canvas.fill_rect(rect::Rect::new(position.x, position.y, 50, 50));
	}
}