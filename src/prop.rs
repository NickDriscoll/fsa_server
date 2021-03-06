use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;
use vector2::Vector2;
use entity::Entity;
use level_parser::EntityType;
use command::Command;

pub struct Prop<'a> {
	texture: Texture<'a>,
	position: Vector2<f32>,
	drawing_box: Rect,
	bounding_box: Rect,
	destination_box: Rect,
	entity_type: EntityType
}

impl<'a> Prop<'a> {	
	pub fn new(t: Texture<'a>, p: Vector2<f32>, draw_box: Rect, e_type: EntityType) -> Prop<'a> {
		let texture_query = t.query();
		let x = p.x;
		let y = p.y;

		let dest = Rect::new(p.x as i32, p.y as i32, draw_box.width(), draw_box.height());

		Prop {
			texture: t,
			position: p,
			drawing_box: draw_box,
			bounding_box: Rect::new(x as i32, y as i32, texture_query.width, texture_query.height),
			destination_box: dest,
			entity_type: e_type
		}
	}
}

impl<'a> Entity for Prop<'a> {
	fn update(&mut self, _duration: Duration) {}

	fn draw(&self, canvas: &mut Canvas<Window>) {
		canvas.copy(&self.texture, self.drawing_box, self.destination_box);
	}

	fn get_entity_type(&self) -> EntityType {
		self.entity_type
	}

	fn get_position(&self) -> &Vector2<f32> {
		&self.position
	}

	fn handle_command(&mut self, _command: Command) {}
}
