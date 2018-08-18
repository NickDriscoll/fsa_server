pub struct Vector2<T> {
	pub x: T,
	pub y: T
}

pub fn new<T>(x: T, y: T) -> Vector2<T> {
	Vector2 {
		x: x,
		y: y
	}
}
