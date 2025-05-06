pub mod process;
pub mod ScopeManager;

pub mod Util {
	use std::sync::atomic::{AtomicIsize, AtomicU64};
	use rand::{seq::SliceRandom, Rng};

	static COUNTER: AtomicU64 = AtomicU64::new(0);

	pub fn generate_8_digit_id() -> String {
		let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		format!("{:08}", id)
	}
}