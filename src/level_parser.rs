use std::vec::Vec;
use std::cell;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem;
use entity::Entity;

//An association between the type of entity and the byte they're encoded as
enum EntityType {
	Player = 0x00,
	Building = 0x01
}

use level_parser::EntityType::*;
impl EntityType {
	fn resolve(i: u8) -> Option<EntityType> {
		match i {
			0x0 => { Some(Player) }
			0x1 => { Some(Building) }
			_ => { None }
		}
	}
}

pub fn save(path: &str, entities: &mut Vec<&cell::RefCell<Entity>>) {

}

pub fn parse(path: &str, entities: &mut Vec<&cell::RefCell<Entity>>) {
	//Open the file
	let mut level_file = match File::open(path) {
		Ok(file) => {
			file
		}
		Err(e) => {
			panic!("{}", e);
		}
	};

	//Buffer needs to hold one identifier byte and then two f32's
	const BUFFER_SIZE: usize = mem::size_of::<u8>() + 2 * mem::size_of::<f32>();
	let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

	//Parse until EOF
	loop {
		match level_file.read(&mut buffer) {
			Ok(0) => {
				//EOF was reached in this case
				break;
			}
			Ok(n) => {
				if n != BUFFER_SIZE {
					panic!("File length not multiple of {}!", BUFFER_SIZE);
				}

				//Create an entity of the right type and put it in entities
				match EntityType::resolve(buffer[0]) {
					Some(entity_type) => {
						
					}
					None => {
						panic!("{} is not valid input to EntityType::resolve()!", buffer[0]);
					}
				}
			}
			Err(e) => {
				println!("{}", e);
			}
		}
	}
}
