pub trait Entity {
	fn update(&self);

	fn draw(canvas: Canvas);
}