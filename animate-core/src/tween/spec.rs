use crate::easing::{self, Easing};
use crate::interpolate::Interpolate;
use crate::tween::{Repeat, Tween};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct TweenSpec {
    pub duration: Duration,
    pub delay: Duration,
    pub easing: Easing,
    pub repeat: Repeat,
    pub alternate: bool,
}

impl PartialEq for TweenSpec {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
            && self.delay == other.delay
            && std::ptr::fn_addr_eq(self.easing, other.easing)
            && self.repeat == other.repeat
            && self.alternate == other.alternate
    }
}

impl Default for TweenSpec {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            delay: Duration::ZERO,
            easing: easing::linear,
            repeat: Repeat::Once,
            alternate: false,
        }
    }
}

impl TweenSpec {
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
            delay: Duration::ZERO,
            easing: easing::linear,
            repeat: Repeat::Once,
            alternate: false,
        }
    }

    pub const fn linear(duration: Duration) -> Self {
        Self::new(duration)
    }

    pub const fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub const fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub const fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub const fn repeat(mut self, repeat: Repeat) -> Self {
        self.repeat = repeat;
        self
    }

    pub const fn alternate(mut self, alternate: bool) -> Self {
        self.alternate = alternate;
        self
    }

    pub fn build<T>(&self, initial: T) -> Tween<T>
    where
        T: Interpolate + Clone + PartialEq,
    {
        Tween::from_spec(*self, initial)
    }
}
