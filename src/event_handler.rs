extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use keyboard_manager::KeyboardManager;

pub struct EventHandler<'a> {
	event_pump: EventPump,
	keyboard_manager: KeyboardManager<'a>
}

impl<'a> EventHandler<'a> {
	pub fn new(e: EventPump, k: KeyboardManager<'a>) -> EventHandler<'a> {
		EventHandler {
			event_pump: e,
			keyboard_manager: k
		}
	}

	pub fn handle_events(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::KeyDown {keycode: Some(code), ..} => {
					self.keyboard_manager.handle_keydown_event(code);
				}
				Event::KeyUp {keycode: Some(code), ..} => {
					self.keyboard_manager.handle_keyup_event(code);
				}
				_ => { }
			}
		}
	}
}
