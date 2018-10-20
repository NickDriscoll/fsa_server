use sdl2::render::TextureCreator;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use std::collections::HashMap;

pub struct TextureManager<'a> {
	texture_creator: TextureCreator<WindowContext>,
	texture_map: HashMap<u32, Texture<'a>>,
	next_id: u32
}

impl<'a> TextureManager<'a> {
	pub fn new(canvas: &Canvas<Window>) -> TextureManager<'a> {
		TextureManager {
			texture_creator: canvas.texture_creator(),
			texture_map: HashMap::new(),
			next_id: 0
		}
	}
}
