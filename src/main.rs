extern crate rand;

use std::{fmt, time, ops::{Add, Index, IndexMut, Sub}};
use rand::seq::SliceRandom;
use raylib::{RaylibHandle, color::Color, ffi::ConfigFlags, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};

mod particles;

use crate::particles::particles::{Balloon, Particle, Rock, Sand};

const TICK_RATE: time::Duration = time::Duration::from_millis(0);
const MAX_BRUSH_SIZE: i32 = 50;

fn main() {
	let (mut handle, thread) = raylib::init()
		.resizable()
		.size(800, 600)
		.title("Particle Simulator")
		.build();

	let mut grid = Grid::new(5);
	let mut prev_tick = time::Instant::now();
	let mut selected: fn() -> Option<Box<dyn Particle>> = || Some(Box::new(Sand::new()));
	let mut brush_size = 20;

	handle.set_target_fps(9999999);

	while !handle.window_should_close() {
		if handle.is_window_resized() { grid.resize(&handle); }

		if handle.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_BUTTON_RIGHT) && handle.is_cursor_on_screen() {
			grid.set_area_screen_relative(handle.get_mouse_position(), || None, &handle, brush_size);
		}
		if handle.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) && handle.is_cursor_on_screen() {
			grid.set_area_screen_relative(handle.get_mouse_position(), selected, &handle, brush_size);
		}
		if handle.get_mouse_wheel_move() != 0.0 {
			brush_size = (brush_size + handle.get_mouse_wheel_move() as i32).clamp(1, MAX_BRUSH_SIZE);
		}

		if handle.is_key_pressed(raylib::ffi::KeyboardKey::KEY_S) { selected = || Some(Box::new(Sand::new())); }
		if handle.is_key_pressed(raylib::ffi::KeyboardKey::KEY_B) { selected = || Some(Box::new(Balloon::new())); }
		if handle.is_key_pressed(raylib::ffi::KeyboardKey::KEY_R) { selected = || Some(Box::new(Rock::new())); }

		if time::Instant::now().checked_duration_since(prev_tick).unwrap() >= TICK_RATE {
			prev_tick = time::Instant::now();

			grid.update();

			let mut draw_handle = handle.begin_drawing(&thread);
			draw_handle.clear_background(Color::WHITE);

			grid.draw(&mut draw_handle);

			draw_handle.draw_circle_lines_v(draw_handle.get_mouse_position(), ((brush_size - 1) * grid.get_grid_size() as i32) as f32, Color::LIGHTGRAY);
		}
	}
}

struct Grid {
	vector: Vec<Option<Box<dyn Particle>>>, 
	height: usize, 
	width: usize,
	grid_size: usize, // The number of pixels per box in the grid
}
impl Grid {
	fn new(grid_size: usize) -> Self {
		Self { vector: Vec::new(), height: 0, width: 0, grid_size }
	}

	fn is_in_bounds(&self, position: Vector2i) -> bool {
		!(position.x < 0 || position.y < 0 || position.x >= self.width as i32 || position.y >= self.height as i32)
	}

	fn get(&self, position: Vector2i) -> Option<&dyn Particle> {
		if position.x < 0 || position.y < 0 || position.x >= self.width as i32 || position.y >= self.height as i32 { return None; }
		self.vector[(position.y * self.width as i32 + position.x) as usize].as_deref()
	}
	
	fn set(&mut self, position: Vector2i, value: Option<Box<dyn Particle>>) {
		if self.is_in_bounds(position) { self.vector[(position.y * self.width as i32 + position.x) as usize] = value; }
		else { panic!("ERROR: Tried to set position {position}! Grid is only {} x {}.", self.width, self.height); }
	}

	fn move_item(&mut self, initial_position: Vector2i, final_position: Vector2i) {
		let particle = self.vector[(initial_position.y * self.width as i32 + initial_position.x) as usize].take();
		self.set(final_position, particle);
	}
	
	fn set_area_screen_relative(&mut self, position: Vector2, particle_generator: fn() -> Option<Box<dyn Particle>>, handle: &RaylibHandle, brush_size: i32) {
		let mut offset_positions = Vec::new();
		let radius = brush_size - 1;
		for y in -radius..=radius {
			for x in -radius..=radius {
				if x * x + y * y <= radius * radius {
					offset_positions.push(Vector2i::new(x, y));
				}
			}
		}

		for offset_position in offset_positions { self.set_screen_relative(position, particle_generator(), handle, offset_position); }
	}

	fn set_screen_relative(&mut self, position: Vector2, particle: Option<Box<dyn Particle>>, handle: &RaylibHandle, offset: Vector2i) {
		let screen_error = self.get_screen_error(handle);
		let x = (((position.x.round() as i32 - screen_error.x) / self.grid_size as i32) + offset.x).clamp(0, self.width as i32 - 1);
		let y = (((position.y.round() as i32 - screen_error.y) / self.grid_size as i32) + offset.y).clamp(0, self.height as i32 - 1);
		self.set(Vector2i::new(x, y), particle);
	}

	fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
		let screen_error = self.get_screen_error(draw_handle);
		let screen_size = Vector2i::new(draw_handle.get_screen_width(), draw_handle.get_screen_height());
		draw_handle.draw_line(screen_error.x, screen_error.y, screen_size.x - screen_error.x, screen_error.y, Color::BLACK);
		draw_handle.draw_line(screen_error.x, screen_error.y, screen_error.x, screen_size.y - screen_error.y, Color::BLACK);
		draw_handle.draw_line(screen_size.x - screen_error.x, screen_error.y, screen_size.x - screen_error.x, screen_size.y - screen_error.y, Color::BLACK);
		draw_handle.draw_line(screen_error.x, screen_size.y - screen_error.y, screen_size.x - screen_error.x, screen_size.y - screen_error.y, Color::BLACK);

		for (i, particle) in self.vector.iter().enumerate() {
			if particle.is_some() {
				let particle = particle.as_ref().unwrap();
				draw_handle.draw_rectangle(((i % self.width) * self.grid_size) as i32 + screen_error.x, ((i / self.width) * self.grid_size) as i32 + screen_error.y,
					self.grid_size as i32, self.grid_size as i32, particle.get_color());
			}
		}
	}

	fn get_screen_error(&self, handle: &RaylibHandle) -> Vector2i {
		Vector2i::new((handle.get_screen_width() - (self.width * self.grid_size) as i32) / 2, (handle.get_screen_height() - (self.height * self.grid_size) as i32) / 2)
	}

	fn resize(&mut self, handle: &RaylibHandle) {
		self.height = handle.get_screen_height() as usize / self.grid_size;
		self.width = handle.get_screen_width() as usize / self.grid_size;
		self.vector.resize_with(self.height * self.width, || None);
	}

	fn flat_index_to_2d(&self, index: usize) -> Vector2i {
		Vector2i::new((index % self.width) as i32, (index / self.width) as i32)
	}

	fn vector_to_flat_index(&self, position: Vector2i) -> usize {
		(position.y * self.width as i32 + position.x) as usize
	}

	fn update(&mut self) {
		let mut particle_position_updates = Vec::new();
		let mut new_vector: Vec<Option<Box<dyn Particle>>> = Vec::with_capacity(self.width * self.height);
		new_vector.resize_with(self.height * self.width, || None);

		for (i, particle) in self.vector.iter().enumerate() {
			if particle.is_some() { particle_position_updates.push((self.flat_index_to_2d(i), particle.as_ref().unwrap().update_position(self.flat_index_to_2d(i), self))); }
		}

		for update in &particle_position_updates {
			if update.0 == update.1 {
				new_vector[self.vector_to_flat_index(update.0)] = self.vector[(update.0.y * self.width as i32 + update.0.x) as usize].take();
			}
		}
		let mut rng = rand::rng();
		particle_position_updates.shuffle(&mut rng);
		for update in particle_position_updates {
			if update.0 != update.1 {
				if new_vector[self.vector_to_flat_index(update.1)].is_none() {
					new_vector[self.vector_to_flat_index(update.1)] = self.vector[(update.0.y * self.width as i32 + update.0.x) as usize].take();
				}
				else {
					new_vector[self.vector_to_flat_index(update.0)] = self.vector[(update.0.y * self.width as i32 + update.0.x) as usize].take();
				}
			}
		}
		self.vector = new_vector;
	}

	fn get_grid_size(&self) -> usize { self.grid_size }
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
impl fmt::Debug for Vector2i {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
