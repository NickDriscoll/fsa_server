use entity_manager::EntityManager;
use std::cell::RefCell;
use std::process;

#[derive(Copy, Clone)]
pub enum Command {
	Quit,
	MoveRight,
	MoveLeft,
	MoveUp,
	MoveDown,
	HaltX,
	HaltY
}

pub struct CommandEmitter<'a> {
	entity_manager: &'a RefCell<EntityManager>,
}

impl<'a> CommandEmitter<'a> {
	pub fn new(entmgr: &'a RefCell<EntityManager>) -> CommandEmitter<'a> {
		CommandEmitter {
			entity_manager: entmgr
		}
	}

	pub fn emit_command(&self, id: u32, command: Command) {
		self.emit_entity_command(id, command);
		self.emit_system_command(command);
	}

	fn emit_entity_command(&self, id: u32, command: Command) {
		match self.entity_manager.borrow_mut().get_mut(id) {
			Some(ent) => {
				ent.handle_command(command);
			}
			None => {
				println!("{} not in entity manager", id);
			}
		}
	}

	fn emit_system_command(&self, command: Command) {
		match command {
			Quit => {
				process::exit(0);
			}
			_ => {}
		}
	}
}
