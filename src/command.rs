/* This file contains definitions for all commands, as well as the enum that stores them */

use command::quit_command::QuitCommand;
use command::move_right_command::MoveRightCommand;

pub enum Command<'a> {
	Quit(QuitCommand),
	MoveRight(MoveRightCommand<'a>)
}

pub mod quit_command {
	use std::process; //Remove this when actual quit command is implemented
	
	pub struct QuitCommand {
	}

	pub fn new() -> QuitCommand {
		QuitCommand {

		}
	}

	impl QuitCommand {
		pub fn execute(&self) {
			process::exit(0);
		}
	}
}

pub mod move_right_command {
	use player::Player;
	use std::cell;

	pub struct MoveRightCommand<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> MoveRightCommand<'a> {
		MoveRightCommand {
			player: p
		}
	}

	impl<'a> MoveRightCommand<'a> {
		pub fn execute(&mut self) {
			self.player.borrow_mut().move_right();
		}
	}
}