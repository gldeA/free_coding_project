pub mod particles {
    use raylib::color::Color;

    use crate::Vector2i;

	pub trait Particle {
		fn get_position(&self) -> Vector2i;
		fn update_position(&mut self) {

		}
		fn get_color(&self) -> Color;
	}

	pub struct Sand { position: Vector2i, color: Color }
	impl Particle for Sand {
		fn get_position(&self) -> Vector2i { self.position }
		fn get_color(&self) -> Color { self.color }
	}
}