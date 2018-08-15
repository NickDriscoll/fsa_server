extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct KeyboardManager<'a> {
	event_pump: &'a mut EventPump,
	keys_to_commands: HashMap<Keycode, Box<FnMut()>>
}

pub fn new(event_pump: &mut EventPump) -> KeyboardManager {
	KeyboardManager {
		event_pump: event_pump,
		keys_to_commands: HashMap::new()
	}
}

impl<'a> KeyboardManager<'a> {
	pub fn handle_input(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::KeyDown {keycode: Some(value), ..} => {
					match self.keys_to_commands.get_mut(&value) {
						Some(command) => {
							command();
						}
						None => {
							println!("You pressed the unmapped key: {}", value);
						}
					}
				}
				_ => { }
			}
		}
	}

	pub fn add_binding(&mut self, keycode: Keycode, command: Box<FnMut()>) {
		self.keys_to_commands.insert(keycode, command);
	}
}
