use crate::Lerp;
use std::cell::Cell;
use std::time::Instant;

pub struct Grease<T>
where
    T: Lerp + PartialEq + Copy,
{
    current: Cell<T>,
    start: Cell<T>,
    end: Cell<T>,
    started_at: Cell<Option<Instant>>,
    duration: f64,
}

impl<T> Grease<T>
where
    T: Lerp + PartialEq + Copy,
{
    pub fn new(initial: T, duration: f64) -> Self {
        Self {
            current: Cell::new(initial),
            start: Cell::new(initial),
            end: Cell::new(initial),
            started_at: Cell::new(None),
            duration,
        }
    }

    pub fn set(&mut self, target: T) {
        self.start.set(self.get());
        self.end.set(target);
        self.started_at.set(Some(Instant::now()));
    }

    pub fn get(&self) -> T {
        if let Some(started) = self.started_at.get() {
            let elapsed = started.elapsed().as_secs_f64() * 1000.0;
            let t = (elapsed / self.duration).clamp(0.0, 1.0);
            // todo: apply easing
            let interp = T::lerp(&self.start.get(), &self.end.get(), t);

            self.current.set(interp);

            if t >= 1.0 {
                self.started_at.set(None);
            }
        }

        self.current.get()
    }
}
