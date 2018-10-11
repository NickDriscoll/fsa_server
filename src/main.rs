extern crate sdl2;
extern crate byteorder;

mod vector2;
mod player;
mod entity;
mod command;
mod keyboard_manager;
mod event_handler;
mod network_manager;
mod prop;
mod level_parser;
mod entity_manager;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use vector2::Vector2;
use std::cell::RefCell;
use std::time::Instant;
use command::Command;
use command::CommandEmitter;
use network_manager::TouchButtons;
use player::Player;
use keyboard_manager::KeyboardManager;
use network_manager::NetworkManager;
use entity_manager::EntityManager;
use event_handler::EventHandler;
use level_parser::EntityType;
use prop::Prop;

const MAX_PLAYERS: usize = 4;

fn init_keyboard<'a>(cmdemit: &'a CommandEmitter, player_ids: [u32; MAX_PLAYERS]) -> KeyboardManager<'a> {
	let mut keyboard_manager = KeyboardManager::new(cmdemit);

	//Add key bindings
	keyboard_manager.add_keydown_binding(Keycode::Escape, (0, Command::Quit));
	keyboard_manager.add_keydown_binding(Keycode::Q, (0, Command::Quit));
	keyboard_manager.add_keydown_binding(Keycode::Down, (player_ids[0], Command::MoveDown));
	keyboard_manager.add_keydown_binding(Keycode::Up, (player_ids[0], Command::MoveUp));
	keyboard_manager.add_keydown_binding(Keycode::Left, (player_ids[0], Command::MoveLeft));
	keyboard_manager.add_keydown_binding(Keycode::Right, (player_ids[0], Command::MoveRight));

	keyboard_manager.add_keyup_binding(Keycode::Left, (player_ids[0], Command::HaltX));
	keyboard_manager.add_keyup_binding(Keycode::Right, (player_ids[0], Command::HaltX));
	keyboard_manager.add_keyup_binding(Keycode::Up, (player_ids[0], Command::HaltY));
	keyboard_manager.add_keyup_binding(Keycode::Down, (player_ids[0], Command::HaltY));

	keyboard_manager
}

fn init_network<'a>(cmdemit: &'a CommandEmitter, player_ids: [u32; MAX_PLAYERS]) -> NetworkManager<'a> {
	let mut network_manager = NetworkManager::begin_listening(cmdemit);

	network_manager.add_touchdown_binding(TouchButtons::Left as u8, (player_ids[0], Command::MoveLeft), 1);
	network_manager.add_touchdown_binding(TouchButtons::Right as u8, (player_ids[0], Command::MoveRight), 1);
	network_manager.add_touchdown_binding(TouchButtons::Up as u8, (player_ids[0], Command::MoveUp), 1);
	network_manager.add_touchdown_binding(TouchButtons::Down as u8, (player_ids[0], Command::MoveDown), 1);

	network_manager.add_touchup_binding(TouchButtons::Left as u8, (player_ids[0], Command::HaltX), 1);
	network_manager.add_touchup_binding(TouchButtons::Right as u8, (player_ids[0], Command::HaltX), 1);
	network_manager.add_touchup_binding(TouchButtons::Up as u8, (player_ids[0], Command::HaltY), 1);
	network_manager.add_touchup_binding(TouchButtons::Down as u8, (player_ids[0], Command::HaltY), 1);

	network_manager
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("FSA", 1280, 720).build().unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	let texture_creator = canvas.texture_creator();

	let background_texture = texture_creator.load_texture("assets/grass.png").unwrap();
	let adobe_texture = texture_creator.load_texture("assets/adobe.png").unwrap();

	let mut player_ids: [u32; MAX_PLAYERS] = [0; MAX_PLAYERS];
	let entity_manager = RefCell::new(EntityManager::new());
	player_ids[0] = entity_manager.borrow_mut().add_entity(Box::new(Player::new(Vector2::new(100.0, 200.0))));
	//let prop_id = entity_manager.borrow_mut().add_entity(Box::new(Prop::new(adobe_texture, Vector2::new(100.0, 100.0), Rect::new(0, 0, 100, 100), EntityType::Building)));

	let command_emitter = CommandEmitter::new(&entity_manager);
	let mut keyboard_manager = init_keyboard(&command_emitter, player_ids);
	let mut network_manager = init_network(&command_emitter, player_ids);

	let mut event_handler = EventHandler::new(sdl_context.event_pump().unwrap(), &mut keyboard_manager);

	let mut previous_instant = Instant::now();

	loop {
		let current_instant = Instant::now();

		//Handle sdl events
		event_handler.handle_events();

		//Handle network input
		network_manager.handle_input();
		
		//Update entities
		entity_manager.borrow_mut().update(current_instant.duration_since(previous_instant));

		//Clear the screen
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();

		//Draw background
		match canvas.copy(&background_texture, None, None) {
			Err(s) => {
				println!("Error drawing background: {}", s);
			}
			_ => {}
		}

		//Draw entities
		entity_manager.borrow().draw(&mut canvas);

		canvas.present();
		previous_instant = current_instant;
	}
}
