#![allow(non_snake_case)]

pub mod Methods;
pub mod Types;

pub mod Extras {
	use rand::Rng;

	pub fn one_in_n_chance(n: u32) -> bool {
		let mut rng = rand::rng();
		rng.random_range(0..n) == 0
	}
}
