pub mod easing;
mod macros;
pub mod mode;
pub mod types;

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub use easing::*;
pub use mode::*;

pub static FRAME_TIME: AtomicUsize = AtomicUsize::new(0);
pub static IS_ANIMATING: AtomicBool = AtomicBool::new(false);

pub trait Animate {
    type Value;
    fn get(&self) -> &Self::Value;
    fn set(&mut self, target: Self::Value);
    fn target(&self) -> &Self::Value;
}

#[derive(Debug)]
pub(crate) struct AnimateState<T, E, I>
where
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T,
{
    pub current: UnsafeCell<T>,
    pub start: UnsafeCell<T>,
    pub target: UnsafeCell<T>,
    pub started_at: UnsafeCell<Option<usize>>,
    pub last_update: UnsafeCell<usize>,
    pub duration: f64,
    pub easing: E,
    pub interp: I,
}

impl<T: Default, E, I> AnimateState<T, E, I>
where
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T,
{
    pub fn new(initial: T, duration: f64, easing: E, interp: I) -> Self {
        Self {
            current: UnsafeCell::new(initial),
            start: UnsafeCell::new(Default::default()),
            target: UnsafeCell::new(Default::default()),
            started_at: UnsafeCell::new(None),
            last_update: UnsafeCell::new(0),
            duration,
            easing,
            interp,
        }
    }
}

pub trait Lerp {
    fn lerp(start: &Self, end: &Self, t: f64) -> Self;
}

#[inline(always)]
pub fn tick(delta: usize) {
    FRAME_TIME.fetch_add(delta, Ordering::Relaxed);
    IS_ANIMATING.store(false, Ordering::Relaxed);
}

pub fn is_animating() -> bool {
    IS_ANIMATING.load(Ordering::Relaxed)
}
