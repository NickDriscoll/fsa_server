extern crate sdl2;

struct GameManager {
	sdl_context: Sdl,
}

impl GameManager {
	pub fn init() {
		GameManager {
			sdl2::init();
		}
	}
}