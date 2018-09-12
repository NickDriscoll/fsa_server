use std::vec::Vec;
use std::cell;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem;

struct LevelParser {

}

//An association between the type of entity and the byte they're encoded as
enum EntityType {

}

pub fn parse(path: &str, entities: &mut Vec<&cell::RefCell<Entity>>) {
	//Open the file
	let level_file = match File::open(path) {
		Ok(file) => {
			file
		}
		Err(e) => {
			panic!("{}", e);
		}
	}

	//Buffer needs to hold one identifier byte and then two f32's
	const BUFFER_SIZE = mem::size_of<u8>() + 2 * mem::size_of<f32>();
	let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

	//Parse until EOF
	loop {
		match level_file.read(&mut buffer) => {
			Ok(0) => {
				//EOF was reached in this case
				break;
			}
			Ok(n) => {
				if n  != BUFFER_SIZE {
					panic!("File length not multiple of {}!", BUFFER_SIZE);
				}

				//Create an entity of the right type and put it in entities
				
			}
			Err(e) => {
				println!("{}", e);
			}
		}
	}
}
