use crate::macros::impl_ops;
use crate::{AnimateState, FRAME_TIME, IS_ANIMATING, Lerp};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Alternate<T: Lerp + PartialEq>(pub(crate) AnimateState<T>);

impl<T: Lerp + PartialEq + Default> Alternate<T> {
    pub fn new(
        initial: T,
        duration: f64,
        easing: fn(f64) -> f64,
        interp: fn(&T, &T, f64) -> T,
    ) -> Self {
        Self(AnimateState::new(initial, duration, easing, interp))
    }

    pub fn set(&mut self, target: T) {
        let now = FRAME_TIME.load(Ordering::Relaxed);
        let current = std::mem::take(self.0.current.get_mut());
        *self.0.start.get_mut() = current;
        *self.0.target.get_mut() = target;
        *self.0.started_at.get_mut() = Some(now);
    }

    pub fn get(&self) -> &T {
        let now = FRAME_TIME.load(Ordering::Relaxed);
        unsafe {
            let last_update = self.0.last_update.get();
            let started_at = self.0.started_at.get();

            if *last_update != now {
                if let Some(start_t) = *started_at {
                    let elapsed = (now - start_t) as f64;
                    let cycle = (elapsed / self.0.duration) as u64;
                    let t_raw = (elapsed % self.0.duration) / self.0.duration;
                    let t = if cycle % 2 == 0 { t_raw } else { 1.0 - t_raw };

                    *self.0.current.get() = (self.0.interp)(
                        &*self.0.start.get(),
                        &*self.0.target.get(),
                        (self.0.easing)(t),
                    );
                    *last_update = now;
                    IS_ANIMATING.store(true, Ordering::Relaxed);
                }
            }
            &*self.0.current.get()
        }
    }

    pub fn target(&self) -> &T {
        unsafe { &*self.0.target.get() }
    }
}

impl_ops!(Alternate);
