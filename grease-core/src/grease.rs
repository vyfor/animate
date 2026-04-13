use crate::{FRAME, Lerp};
use std::cell::Cell;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub struct Grease<T>
where
    T: Lerp + PartialEq + Copy,
{
    current: Cell<T>,
    start: Cell<T>,
    end: Cell<T>,
    started_at: Cell<Option<Instant>>,
    last_frame: Cell<usize>,
    duration: f64,
    easing: fn(f64) -> f64,
}

impl<T> Grease<T>
where
    T: Lerp + PartialEq + Copy,
{
    pub fn new(initial: T, duration: f64, easing: fn(f64) -> f64) -> Self {
        Self {
            current: Cell::new(initial),
            start: Cell::new(initial),
            end: Cell::new(initial),
            started_at: Cell::new(None),
            last_frame: Cell::new(0),
            duration,
            easing,
        }
    }

    pub fn set(&mut self, target: T) {
        self.start.set(self.get());
        self.end.set(target);
        self.started_at.set(Some(Instant::now()));
    }

    pub fn get(&self) -> T {
        let frame = FRAME.load(Ordering::Relaxed);
        if self.last_frame.get() == frame {
            return self.current.get();
        }

        let started = match self.started_at.get() {
            Some(t) => t,
            None => return self.current.get(),
        };

        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        let t = (elapsed / self.duration).clamp(0.0, 1.0);
        let interp = T::lerp(&self.start.get(), &self.end.get(), (self.easing)(t));

        self.current.set(interp);
        self.last_frame.set(frame);

        if t >= 1.0 {
            self.started_at.set(None);
        }

        self.current.get()
    }

    pub fn end(&self) -> T {
        self.end.get()
    }
}
