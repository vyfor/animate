pub mod grease;

use std::sync::atomic::{AtomicUsize, Ordering};

pub use grease::Grease;

pub static FRAME: AtomicUsize = AtomicUsize::new(0);

pub trait Lerp {
    fn lerp(start: &Self, end: &Self, t: f64) -> Self;
}

pub fn tick() {
    FRAME.fetch_add(1, Ordering::Relaxed);
}
