use std::vec::Vec;
use std::collections::HashMap;
use entity::Entity;

/*
	The purpose of this struct is to provide an API for
	accessing the list of entities
*/
pub struct EntityManager {
	entities: HashMap<u32, Box<Entity>>,
	next_index: u32
}

impl<'a> EntityManager {
	pub fn new() -> EntityManager {
		EntityManager {
			entities: HashMap::new(),
			next_index: 0
		}
	}

	pub fn add_entity(&mut self, entity: Box<Entity>) -> u32 {
		self.entities.insert(self.next_index, entity);
		self.next_index += 1;
		self.next_index - 1
	}

	pub fn get_entity(&self, id: u32) -> Option<&Box<Entity>> {
		self.entities.get(&id)
	}

	pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Box<Entity>> {
		self.entities.get_mut(&id)
	}
}
