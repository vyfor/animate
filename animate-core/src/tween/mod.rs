mod repeat;
mod spec;

pub use repeat::Repeat;
pub use spec::TweenSpec;

use crate::easing::Easing;
use crate::interpolate::Interpolate;
use crate::{Activity, Animation, Time};
use std::fmt;
use std::ops::Deref;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Tween<T> {
    pub spec: TweenSpec,
    current: T,
    from: T,
    target: T,
    start: Option<Duration>,
    pending: bool,
    running: bool,
}

impl<T> Tween<T>
where
    T: Interpolate + Clone + PartialEq,
{
    pub fn new(initial: T) -> Self {
        Self::from_spec(TweenSpec::default(), initial)
    }

    pub fn from_spec(spec: TweenSpec, initial: T) -> Self {
        Self {
            spec,
            from: initial.clone(),
            target: initial.clone(),
            current: initial,
            start: None,
            pending: false,
            running: false,
        }
    }

    pub fn spec(mut self, spec: TweenSpec) -> Self {
        self.spec = spec;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.spec.duration = duration;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.spec.delay = delay;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.spec.easing = easing;
        self
    }

    pub fn repeat(mut self, repeat: Repeat) -> Self {
        self.spec.repeat = repeat;
        self
    }

    pub fn alternate(mut self, alternate: bool) -> Self {
        self.spec.alternate = alternate;
        self
    }

    pub fn to(&mut self, target: T) {
        if self.running || self.pending {
            if target == self.target {
                return;
            }
        } else if target == self.current {
            self.target = target;
            return;
        }
        self.target = target;
        self.pending = true;
    }

    pub fn is_running(&self) -> bool {
        self.running || self.pending
    }

    pub fn finish(&mut self) {
        self.current = self.target.clone();
        self.halt();
    }

    pub fn stop(&mut self) {
        self.halt();
    }

    fn halt(&mut self) {
        self.start = None;
        self.pending = false;
        self.running = false;
    }

    pub fn advance(&mut self, time: Time) -> Activity {
        if self.pending {
            self.from = self.current.clone();
            self.start = Some(time.elapsed);
            self.pending = false;
            self.running = true;
        }

        let Some(start) = self.start else {
            return Activity::NONE;
        };

        let active = time
            .elapsed
            .saturating_sub(start)
            .saturating_sub(self.spec.delay);

        if self.spec.duration.is_zero() {
            let changed = self.target != self.current;
            self.current = self.target.clone();
            self.halt();
            return Activity {
                changed,
                running: false,
                finished: true,
            };
        }

        let total_cycles = self.spec.repeat.cycles();
        let duration_nanos = self.spec.duration.as_nanos();
        let cycle = (active.as_nanos() / duration_nanos) as u64;

        if cycle >= total_cycles {
            let end = self.cycle_end(total_cycles - 1);
            let changed = end != self.current;
            self.current = end;
            self.halt();
            return Activity {
                changed,
                running: false,
                finished: true,
            };
        }

        let frac = (active.as_nanos() % duration_nanos) as f32 / duration_nanos as f32;
        let t = if self.spec.alternate && cycle % 2 == 1 {
            1.0 - frac
        } else {
            frac
        };
        let value = T::lerp(&self.from, &self.target, (self.spec.easing)(t));
        let changed = value != self.current;
        self.current = value;
        Activity {
            changed,
            running: true,
            finished: false,
        }
    }

    fn cycle_end(&self, cycle: u64) -> T {
        if self.spec.alternate && cycle % 2 == 1 {
            self.from.clone()
        } else {
            self.target.clone()
        }
    }
}

impl<T: Default> Default for Tween<T>
where
    T: Interpolate + Clone + PartialEq,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Deref for Tween<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.current
    }
}

impl<T> fmt::Display for Tween<T>
where
    T: Interpolate + Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.current.fmt(f)
    }
}

impl<T> Animation for Tween<T>
where
    T: Interpolate + Clone + PartialEq,
{
    type Value = T;

    #[inline]
    fn advance(&mut self, time: Time) -> Activity {
        Tween::advance(self, time)
    }

    #[inline]
    fn value(&self) -> &T {
        &self.current
    }

    #[inline]
    fn target(&self) -> &T {
        &self.target
    }

    #[inline]
    fn to(&mut self, target: T) {
        Tween::to(self, target);
    }
}
