use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use command::Command;
use command::CommandEmitter;

pub struct KeyboardManager<'a> {
	keydown_commands: HashMap<Keycode, (u32, Command)>,
	keyup_commands: HashMap<Keycode, (u32, Command)>,
	command_emitter: &'a CommandEmitter<'a>
}

impl<'a> KeyboardManager<'a> {
	pub fn new(cmdemit: &'a CommandEmitter) -> KeyboardManager<'a> {
		KeyboardManager {
			keydown_commands: HashMap::new(),
			keyup_commands: HashMap::new(),
			command_emitter: cmdemit
		}
	}

	pub fn handle_keydown_event(&mut self, keycode: Keycode) {
		match self.keydown_commands.get_mut(&keycode) {
			Some((id, command)) => {
				self.command_emitter.emit_command(*id, *command);
			}
			None => {
				println!("You pressed the unbound key: {}", keycode);
			}
		}
	}

	pub fn handle_keyup_event(&mut self, keycode: Keycode) {
		match self.keyup_commands.get_mut(&keycode) {
			Some((id, command)) => {
				self.command_emitter.emit_command(*id, *command);
			}
			None => { }
		}
	}

	pub fn add_keydown_binding(&mut self, keycode: Keycode, command: (u32, Command)) {
		self.keydown_commands.insert(keycode, command);	
	}

	pub fn add_keyup_binding(&mut self, keycode: Keycode, command: (u32, Command)) {
		self.keyup_commands.insert(keycode, command);
	}

	pub fn clear_bindings(&mut self) {
		self.keydown_commands.clear();
		self.keyup_commands.clear();
	}
}
