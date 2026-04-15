use crate::macros::impl_ops;
use crate::{Animate, AnimateState, FRAME_TIME, IS_ANIMATING, Lerp};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Alternate<T, E, I>(pub(crate) AnimateState<T, E, I>)
where
    T: Lerp + PartialEq,
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T;

impl<T, E, I> Alternate<T, E, I>
where
    T: Lerp + PartialEq + Default,
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T,
{
    pub fn new(initial: T, duration: f64, easing: E, interp: I) -> Self {
        Self(AnimateState::new(initial, duration, easing, interp))
    }
}

impl<T, E, I> Animate for Alternate<T, E, I>
where
    T: Lerp + PartialEq + Default,
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T,
{
    type Value = T;

    fn update(&mut self) {
        let inner = &mut self.0.inner;

        if inner.pending {
            inner.start = std::mem::take(&mut inner.current);
            inner.pending = false;
        }

        if let Some(start_t) = inner.started_at {
            if self.0.duration > 0.0 {
                let now = FRAME_TIME.load(Ordering::Relaxed);
                let elapsed = now.saturating_sub(start_t) as f64;
                let cycle = (elapsed / self.0.duration) as u64;
                let t_raw = (elapsed % self.0.duration) / self.0.duration;
                let t = if cycle % 2 == 0 { t_raw } else { 1.0 - t_raw };

                inner.current = (self.0.interp)(&inner.start, &inner.target, (self.0.easing)(t));
            }
            IS_ANIMATING.store(true, Ordering::Relaxed);
        }
    }

    fn get(&self) -> &T {
        &self.0.inner.current
    }

    fn set(&mut self, target: T) {
        let now = FRAME_TIME.load(Ordering::Relaxed);
        let inner = &mut self.0.inner;

        inner.target = target;
        inner.started_at = Some(now);
        inner.pending = true;
    }

    fn target(&self) -> &T {
        &self.0.inner.target
    }
}

impl_ops!(Alternate);
