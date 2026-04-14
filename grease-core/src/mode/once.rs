use crate::macros::impl_ops;
use crate::{FRAME, GreaseState, IS_ANIMATING, Lerp};
use std::sync::atomic::Ordering;
use std::time::Instant;

#[derive(Debug)]
pub struct Once<T: Lerp + PartialEq>(pub(crate) GreaseState<T>);

impl<T: Lerp + PartialEq + Default> Once<T> {
    pub fn new(
        initial: T,
        duration: f64,
        easing: fn(f64) -> f64,
        interp: fn(&T, &T, f64) -> T,
    ) -> Self {
        Self(GreaseState::new(initial, duration, easing, interp))
    }

    pub fn set(&mut self, target: T) {
        let current = std::mem::take(self.0.current.get_mut());
        *self.0.start.get_mut() = current;
        *self.0.target.get_mut() = target;
        *self.0.started_at.get_mut() = Some(Instant::now());
    }

    pub fn get(&self) -> &T {
        let frame = FRAME.load(Ordering::Relaxed);
        unsafe {
            let last_frame = self.0.last_frame.get();
            let started_at = self.0.started_at.get();
            if *last_frame != frame {
                if let Some(started) = *started_at {
                    let elapsed = started.elapsed().as_secs_f64() * 1000.0;
                    let t = (elapsed / self.0.duration).clamp(0.0, 1.0);
                    *self.0.current.get() = (self.0.interp)(
                        &*self.0.start.get(),
                        &*self.0.target.get(),
                        (self.0.easing)(t),
                    );
                    *last_frame = frame;
                    if t >= 1.0 {
                        *started_at = None;
                    } else {
                        IS_ANIMATING.store(true, Ordering::Relaxed);
                    }
                }
            }
            &*self.0.current.get()
        }
    }

    pub fn target(&self) -> &T {
        unsafe {
            if (*self.0.started_at.get()).is_none() {
                &*self.0.current.get()
            } else {
                &*self.0.target.get()
            }
        }
    }
}

impl_ops!(Once);
