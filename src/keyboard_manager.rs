extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use command::Command;

pub struct KeyboardManager<'a> {
	event_pump: &'a mut EventPump,
	keys_to_commands: HashMap<Keycode, Command<'a>>
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
						Some(command_enum) => {
							match command_enum {
								Command::Quit(command) => {
									command.execute();
								}
								Command::MoveRight(command) => {
									command.execute();
								}
								Command::MoveLeft(command) => {
									command.execute();
								}
								Command::MoveUp(command) => {
									command.execute();
								}
								Command::MoveDown(command) => {
									command.execute();
								}
							}
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

	pub fn add_binding(&mut self, keycode: Keycode, command: Command<'a>) {
		self.keys_to_commands.insert(keycode, command);
	}
}
