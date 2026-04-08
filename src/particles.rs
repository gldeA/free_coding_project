pub mod particles {
    use raylib::color::Color;

    use crate::Vector2i;

	pub trait Particle {
		fn update_position(&mut self) {

		}
		fn get_color(&self) -> Color;
	}

	pub struct Sand { }
	impl Sand {
		pub fn new() -> Sand { Sand { } }
	}
	impl Particle for Sand {
		fn get_color(&self) -> Color { Color::GOLD }
	}
}