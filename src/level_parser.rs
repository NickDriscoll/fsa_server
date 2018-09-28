use std::vec::Vec;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem;
use std::string::String;
use entity_manager::EntityManager;
use byteorder::{ByteOrder, LittleEndian};

const LEVEL_DIRECTORY: &str = "levels/";

//An association between the type of entity and the byte they're encoded as
#[derive(Copy, Clone)]
pub enum EntityType {
	Player = 0x00,
	Building = 0x01
}

impl EntityType {
	fn resolve(i: u8) -> Option<EntityType> {
		match i {
			0x0 => { Some(EntityType::Player) }
			0x1 => { Some(EntityType::Building) }
			_ => { None }
		}
	}
}

pub fn save(path: &str, entities: &EntityManager) {
	const BUFFER_SIZE: usize = mem::size_of::<u8>() + 2 * mem::size_of::<f32>();

	let mut true_path = String::from(LEVEL_DIRECTORY);
	true_path.push_str(path);
	let mut level_file = match File::create(true_path) {
		Ok(file) => { file }
		Err(e) => {
			panic!("{}", e);
		}
	};

	for entity in entities.iter() {
		let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
		buffer[0] = entity.get_entity_type() as u8;
		let pos = entity.get_position();
		LittleEndian::write_f32(&mut buffer[1..5], pos.x);
		LittleEndian::write_f32(&mut buffer[5..9], pos.y);
		level_file.write(&buffer);
	}
}

pub fn load(path: &str, entities: &mut EntityManager) {
	//Open the file
	let mut level_file = match File::open(path) {
		Ok(file) => { file }
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
