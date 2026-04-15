use crate::macros::impl_ops;
use crate::{AnimateState, FRAME_TIME, IS_ANIMATING, Lerp};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Cycle<T, E, I>(pub(crate) AnimateState<T, E, I>)
where
    T: Lerp + PartialEq,
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T;

impl<T, E, I> Cycle<T, E, I>
where
    T: Lerp + PartialEq + Default,
    E: Fn(f64) -> f64,
    I: Fn(&T, &T, f64) -> T,
{
    pub fn new(initial: T, duration: f64, easing: E, interp: I) -> Self {
        Self(AnimateState::new(initial, duration, easing, interp))
    }

    pub fn set(&mut self, target: T) {
        let now = FRAME_TIME.load(Ordering::Relaxed);
        let inner = self.0.inner.get_mut();
        inner.start = std::mem::take(&mut inner.current);
        inner.target = target;
        inner.started_at = Some(now);
    }

    pub fn get(&self) -> &T {
        let now = FRAME_TIME.load(Ordering::Relaxed);
        unsafe {
            let inner = &mut *self.0.inner.get();
            if inner.last_update != now {
                if let Some(start_t) = inner.started_at {
                    if self.0.duration > 0.0 {
                        let elapsed = now.saturating_sub(start_t) as f64;
                        let t = (elapsed % self.0.duration) / self.0.duration;

                        inner.current =
                            (self.0.interp)(&inner.start, &inner.target, (self.0.easing)(t));
                    }
                    inner.last_update = now;
                    IS_ANIMATING.store(true, Ordering::Relaxed);
                }
            }
            &inner.current
        }
    }

    pub fn target(&self) -> &T {
        unsafe { &(*self.0.inner.get()).target }
    }
}

impl_ops!(Cycle);
