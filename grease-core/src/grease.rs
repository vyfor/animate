use crate::{FRAME, Lerp};
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub struct Grease<T>
where
    T: Lerp + PartialEq,
{
    current: UnsafeCell<T>,
    start: UnsafeCell<T>,
    target: UnsafeCell<T>,
    started_at: UnsafeCell<Option<Instant>>,
    last_frame: UnsafeCell<usize>,
    duration: f64,
    easing: fn(f64) -> f64,
}

impl<T> Grease<T>
where
    T: Lerp + PartialEq + Default,
{
    pub fn new(initial: T, duration: f64, easing: fn(f64) -> f64) -> Self {
        Self {
            current: UnsafeCell::new(initial),
            start: UnsafeCell::new(T::default()),
            target: UnsafeCell::new(T::default()),
            started_at: UnsafeCell::new(None),
            last_frame: UnsafeCell::new(0),
            duration,
            easing,
        }
    }

    pub fn set(&mut self, target: T) {
        let current_val = std::mem::take(self.current.get_mut());
        *self.start.get_mut() = current_val;
        *self.target.get_mut() = target;
        *self.started_at.get_mut() = Some(Instant::now());
    }

    pub fn get(&self) -> &T {
        let frame = FRAME.load(Ordering::Relaxed);

        unsafe {
            let last_frame_ptr = self.last_frame.get();
            let started_at_ptr = self.started_at.get();

            if *last_frame_ptr != frame {
                if let Some(started) = *started_at_ptr {
                    let elapsed = started.elapsed().as_secs_f64() * 1000.0;
                    let t = (elapsed / self.duration).clamp(0.0, 1.0);
                    let interp = T::lerp(&*self.start.get(), &*self.target.get(), (self.easing)(t));

                    *self.current.get() = interp;
                    *last_frame_ptr = frame;

                    if t >= 1.0 {
                        *started_at_ptr = None;
                    }
                }
            }

            &*self.current.get()
        }
    }

    pub fn target(&self) -> &T {
        unsafe {
            if (*self.started_at.get()).is_none() {
                &*self.current.get()
            } else {
                &*self.target.get()
            }
        }
    }
}
