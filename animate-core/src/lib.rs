pub mod easing;
mod macros;
pub mod mode;
pub mod types;

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Instant,
};

pub use easing::*;
pub use mode::*;

pub static FRAME: AtomicUsize = AtomicUsize::new(0);
pub static IS_ANIMATING: AtomicBool = AtomicBool::new(false);

pub trait Animate {
    type Value;
    fn get(&self) -> &Self::Value;
    fn set(&mut self, target: Self::Value);
    fn target(&self) -> &Self::Value;
}

#[derive(Debug)]
pub(crate) struct AnimateState<T> {
    pub current: UnsafeCell<T>,
    pub start: UnsafeCell<T>,
    pub target: UnsafeCell<T>,
    pub started_at: UnsafeCell<Option<Instant>>,
    pub last_frame: UnsafeCell<usize>,
    pub duration: f64,
    pub easing: fn(f64) -> f64,
    pub interp: fn(&T, &T, f64) -> T,
}

impl<T: Default> AnimateState<T> {
    pub fn new(
        initial: T,
        duration: f64,
        easing: fn(f64) -> f64,
        interp: fn(&T, &T, f64) -> T,
    ) -> Self {
        Self {
            current: UnsafeCell::new(initial),
            start: UnsafeCell::new(Default::default()),
            target: UnsafeCell::new(Default::default()),
            started_at: UnsafeCell::new(None),
            last_frame: UnsafeCell::new(0),
            duration,
            easing,
            interp,
        }
    }
}

pub trait Lerp {
    fn lerp(start: &Self, end: &Self, t: f64) -> Self;
}

pub fn tick() {
    FRAME.fetch_add(1, Ordering::Relaxed);
}

pub fn is_animating() -> bool {
    IS_ANIMATING.load(Ordering::Relaxed)
}
