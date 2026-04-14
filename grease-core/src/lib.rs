pub mod easing;
pub mod grease;
pub mod types;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub use easing::*;
pub use grease::Grease;

pub static FRAME: AtomicUsize = AtomicUsize::new(0);
pub static IS_ANIMATING: AtomicBool = AtomicBool::new(false);

pub trait Lerp {
    fn lerp(start: &Self, end: &Self, t: f64) -> Self;
}

pub fn tick() {
    FRAME.fetch_add(1, Ordering::Relaxed);
}

pub fn is_animating() -> bool {
    IS_ANIMATING.load(Ordering::Relaxed)
}
