pub mod particles {
    use raylib::color::Color;

    use crate::{Grid, Vector2i};

	pub trait Particle {
		fn update_position(&self, position: Vector2i, _grid: &Grid) -> Vector2i { position }
		fn get_color(&self) -> Color;
	}

	pub struct Sand { }
	impl Sand {
		pub fn new() -> Sand { Sand { } }
	}
	impl Particle for Sand {
		fn update_position(&self, position: Vector2i, grid: &Grid) -> Vector2i {
			if grid.is_in_bounds(position + Vector2i::new(0, 1)) && grid.get(position + Vector2i::new(0, 1)).is_none() {
				return position + Vector2i::new(0, 1);
			}
			let check_left_first = rand::random_bool(0.5);
			if check_left_first {
				if grid.is_in_bounds(position + Vector2i::new(-1, 1)) && grid.get(position + Vector2i::new(-1, 1)).is_none() {
					return position + Vector2i::new(-1, 1);
				}
				else if grid.is_in_bounds(position + Vector2i::new(1, 1)) && grid.get(position + Vector2i::new(1, 1)).is_none() {
					return position + Vector2i::new(1, 1);
				}
			}
			else {
				if grid.is_in_bounds(position + Vector2i::new(1, 1)) && grid.get(position + Vector2i::new(1, 1)).is_none() {
					return position + Vector2i::new(1, 1);
				}
				else if grid.is_in_bounds(position + Vector2i::new(-1, 1)) && grid.get(position + Vector2i::new(-1, 1)).is_none() {
					return position + Vector2i::new(-1, 1);
				}
			}
			return position;
		}

		fn get_color(&self) -> Color { Color::GOLD }
	}


	pub struct Balloon { }
	impl Balloon {
		pub fn new() -> Balloon { Balloon { } }
	}
	impl Particle for Balloon {
		fn update_position(&self, position: Vector2i, grid: &Grid) -> Vector2i {
			if grid.is_in_bounds(position + Vector2i::new(0, -1)) && grid.get(position + Vector2i::new(0, -1)).is_none() {
				return position + Vector2i::new(0, -1);
			}
			let check_left_first = rand::random_bool(0.5);
			if check_left_first {
				if grid.is_in_bounds(position + Vector2i::new(-1, -1)) && grid.get(position + Vector2i::new(-1, -1)).is_none() {
					return position + Vector2i::new(-1, -1);
				}
				else if grid.is_in_bounds(position + Vector2i::new(1, -1)) && grid.get(position + Vector2i::new(1, -1)).is_none() {
					return position + Vector2i::new(1, -1);
				}
			}
			else {
				if grid.is_in_bounds(position + Vector2i::new(1, -1)) && grid.get(position + Vector2i::new(1, -1)).is_none() {
					return position + Vector2i::new(1, -1);
				}
				else if grid.is_in_bounds(position + Vector2i::new(-1, -1)) && grid.get(position + Vector2i::new(-1, -1)).is_none() {
					return position + Vector2i::new(-1, -1);
				}
			}
			return position;
		}

		fn get_color(&self) -> Color { Color::RED }
	}

	pub struct Rock {}
	impl Rock {
		pub fn new() -> Rock { Rock{} }
	}
	impl Particle for Rock {
		fn get_color(&self) -> Color { Color::GRAY }
	}
}