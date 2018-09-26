/* This file contains definitions for all commands, as well as the enum that stores them */

use command::quit_command::QuitCommand;
use command::move_right_command::MoveRightCommand;
use command::move_left_command::MoveLeftCommand;
use command::move_up_command::MoveUpCommand;
use command::move_down_command::MoveDownCommand;
use command::halt_x_command::HaltX;
use command::halt_y_command::HaltY;
use command::Command::*;

pub enum Command<'a> {
	Quit(QuitCommand),
	MoveRight(MoveRightCommand<'a>),
	MoveLeft(MoveLeftCommand<'a>),
	MoveUp(MoveUpCommand<'a>),
	MoveDown(MoveDownCommand<'a>),
	HaltX(HaltX<'a>),
	HaltY(HaltY<'a>)
}

impl<'a> Command<'a> {
	pub fn execute(&mut self) {
		match self {
			Quit(c) => {
				c.execute();
			}
			MoveRight(c) => {
				c.execute();
			}
			MoveLeft(c) => {
				c.execute();
			}
			MoveUp(c) => {
				c.execute();
			}
			MoveDown(c) => {
				c.execute();
			}
			HaltX(c) => {
				c.execute();
			}
			HaltY(c) => {
				c.execute();
			}
		}
	}
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

pub mod move_left_command {
	use player::Player;
	use std::cell;

	pub struct MoveLeftCommand<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> MoveLeftCommand<'a> {
		MoveLeftCommand {
			player: p
		}
	}

	impl<'a> MoveLeftCommand<'a> {
		pub fn execute(&self) {
			self.player.borrow_mut().move_left();
		}
	}
}

pub mod move_up_command {
	use player::Player;
	use std::cell;

	pub struct MoveUpCommand<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> MoveUpCommand<'a> {
		MoveUpCommand {
			player: p
		}
	}

	impl<'a> MoveUpCommand<'a> {
		pub fn execute(&self) {
			self.player.borrow_mut().move_up();
		}
	}
}

pub mod move_down_command {
	use player::Player;
	use std::cell;

	pub struct MoveDownCommand<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> MoveDownCommand<'a> {
		MoveDownCommand {
			player: p
		}
	}

	impl<'a> MoveDownCommand<'a> {
		pub fn execute(&self) {
			self.player.borrow_mut().move_down();
		}
	}
}

pub mod halt_x_command {
	use player::Player;
	use std::cell;

	pub struct HaltX<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> HaltX<'a> {
		HaltX {
			player: p
		}
	}

	impl<'a> HaltX<'a> {
		pub fn execute(&self) {
			self.player.borrow_mut().halt_x_velocity();
		}
	}
}

pub mod halt_y_command {
	use player::Player;
	use std::cell;

	pub struct HaltY<'a> {
		player: &'a cell::RefCell<Player>
	}

	pub fn new<'a>(p: &'a cell::RefCell<Player>) -> HaltY<'a> {
		HaltY {
			player: p
		}
	}

	impl<'a> HaltY<'a> {
		pub fn execute(&self) {
			self.player.borrow_mut().halt_y_velocity();
		}
	}
}
