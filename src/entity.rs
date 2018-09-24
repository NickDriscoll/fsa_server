use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use level_parser::EntityType;
use vector2::Vector2;

pub trait Entity {
	fn update(&mut self, elapsed: Duration);

	fn draw(&self, canvas: &mut Canvas<Window>);

	fn get_entity_type(&self) -> &EntityType;

	fn get_position(&self) -> &Vector2<f32>;
}
