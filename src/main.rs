use std::{fmt, ops::{Add, Index, IndexMut, Sub}};
use raylib::math::Vector2;

mod particles;

use crate::particles::particles::{Particle};

fn main() {
	
}

struct Grid {
	vector: Vec<Option<Box<dyn Particle>>>, 
	height: u16, 
	width: u16,
	grid_size: u8, // The number of pixels per box in the grid
}
impl Grid {
	fn new(height: u16, width: u16, grid_size: u8) -> Self {
		let mut vector = Vec::with_capacity((height * width) as usize);
		vector.resize_with((height * width) as usize, || None);
		Self { vector, height, width, grid_size }
	}
	fn get(&self, position: Vector2i) -> Option<&dyn Particle> {
		self.vector[(position.y * self.width as i32 + position.x) as usize].as_deref()
	}
	fn set<P: Particle + 'static>(&mut self, position: Vector2i, value: P) {
		self.vector[(position.y * self.width as i32 + position.x) as usize] = Some(Box::new(value));
	}
	fn draw(&self) {
		// TODO: Implement
	}
}

#[derive(Copy, Clone)]
struct Vector2i { x: i32, y: i32 }
impl Vector2i {
	fn new(x: i32, y: i32) -> Vector2i { Self{x, y} }

	// Returns the floor if there are decimal points, does not round
	fn from_vector2(vec: Vector2) -> Vector2i { Vector2i::new(vec.x as i32, vec.y as i32) }

	fn within(&self, top_left: Vector2i, bottom_right: Vector2i) -> bool {
		self.x > top_left.x && self.x < bottom_right.x && self.y > top_left.y && self.y < bottom_right.y
	}
}
impl Index<usize> for Vector2i {
	type Output = i32;
	fn index(&self, index: usize) -> &Self::Output {
		if index == 0 { return &self.x; }
		else if index == 1 { return &self.y; }
		else { panic!("Index out of bounds! Vector2i has length of 2."); }
	}
}
impl IndexMut<usize> for Vector2i {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if index == 0 { return &mut self.x; }
		else if index == 1 { return &mut self.y; }
		else { panic!("Index out of bounds! Vector2i has length of 2."); }
	}
}
impl Add for Vector2i {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Vector2i::new(self.x + other.x, self.y + other.y)
	}
}
impl Sub for Vector2i {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Vector2i::new(self.x - other.x, self.y - other.y)
	}
}
impl fmt::Display for Vector2i {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
