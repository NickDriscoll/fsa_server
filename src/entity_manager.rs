use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::time::Duration;
use entity::Entity;
use sdl2::render::Canvas;
use sdl2::video::Window;

/*
	The purpose of this struct is to provide an API for
	accessing the list of entities
*/
pub struct EntityManager {
	entities: HashMap<u32, Box<Entity>>,
	next_index: u32
}

impl EntityManager {
	pub fn new() -> EntityManager {
		EntityManager {
			entities: HashMap::new(),
			next_index: 0
		}
	}

	//Adds an entity to the collection and returns its id
	pub fn add_entity(&mut self, entity: Box<Entity>) -> u32 {
		self.entities.insert(self.next_index, entity);
		self.next_index += 1;
		self.next_index - 1
	}

	pub fn get(&self, id: u32) -> Option<&Box<Entity>> {
		self.entities.get(&id)
	}

	pub fn get_mut(&mut self, id: u32) -> Option<&mut Box<Entity>> {
		self.entities.get_mut(&id)
	}

	pub fn clear(&mut self) {
		self.entities.clear();
		self.next_index = 0;
	}

	pub fn iter(&self) -> Values<u32, Box<Entity>> {
		self.entities.values()
	}

	pub fn update(&mut self, elapsed: Duration) {
		for entity in self.entities.values_mut() {
			entity.update(elapsed);
		}
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>) {
		//Have sorting code here

		for entity in self.entities.values() {
			entity.draw(canvas);
		}
	}
}
