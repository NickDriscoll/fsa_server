use entity_manager::EntityManager;

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
	entity_manager: &'a EntityManager
}

impl<'a> CommandEmitter<'a> {
	pub fn new(entmgr: &'a EntityManager) -> CommandEmitter<'a> {
		CommandEmitter {
			entity_manager: entmgr
		}
	}

	pub fn emit_entity_command(&self, id: u32, command: Command) {
		match self.entity_manager.get_mut(id) {
			Some(ent) => {
				ent.handle_command(command);
			}
			None => {
				println!("{} not in entity manager", id);
			}
		}
	}
}
