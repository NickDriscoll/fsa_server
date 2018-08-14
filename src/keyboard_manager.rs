extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use command::Command;

pub struct KeyboardManager {
	event_pump: EventPump,
	keys_to_commands: HashMap<Keycode, Box<T: Command>>
}

pub fn new(event_pump: EventPump) -> KeyboardManager {
	KeyboardManager {
		event_pump: event_pump,
		keys_to_commands: HashMap::new()
	}
}

impl KeyboardManager {
	pub fn handle_keyboard(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::KeyDown {keycode: Some(Keycode::Q), ..} => {

				}
				Event::KeyDown {keycode: Some(value), ..} => {
					println!("You pressed {}", value);
				}
				_ => {

				}
			}
		}
	}

	pub fn add_mapping(&mut self, keycode: Keycode, command: Box<T: Command>) {

	}
}