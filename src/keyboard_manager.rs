use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use command::Command;

pub struct KeyboardManager<'a> {
	keydown_commands: HashMap<Keycode, Command<'a>>,
	keyup_commands: HashMap<Keycode, Command<'a>>
}

pub fn new<'a>() -> KeyboardManager<'a> {
	KeyboardManager {
		keydown_commands: HashMap::new(),
		keyup_commands: HashMap::new()
	}
}

impl<'a> KeyboardManager<'a> {
	pub fn handle_keydown_event(&mut self, keycode: Keycode) {
		match self.keydown_commands.get_mut(&keycode) {
			Some(command) => {
				command.execute();
			}
			None => {
				println!("You pressed the unbound key: {}", keycode);
			}
		}
	}

	pub fn handle_keyup_event(&mut self, keycode: Keycode) {
		match self.keyup_commands.get_mut(&keycode) {
			Some(command) => {
				command.execute();
			}
			None => { }
		}
	}

	pub fn add_keydown_binding(&mut self, keycode: Keycode, command: Command<'a>) {
		self.keydown_commands.insert(keycode, command);	
	}

	pub fn add_keyup_binding(&mut self, keycode: Keycode, command: Command<'a>) {
		self.keyup_commands.insert(keycode, command);
	}
}
