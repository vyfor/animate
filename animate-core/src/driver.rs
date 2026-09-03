use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, Settled, Spring};
use crate::transition::Transition;
use crate::tween::Tween;
use crate::{Activity, Animation, Time};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum Driver<T: Integrate> {
    Tween(Tween<T>),
    Spring(Spring<T>),
}

impl<T> Driver<T>
where
    T: Integrate + Interpolate + Clone + PartialEq + Distance,
    T::Velocity: Settled,
{
    pub fn advance(&mut self, time: Time) -> Activity {
        match self {
            Driver::Tween(t) => t.advance(time),
            Driver::Spring(s) => s.advance(time),
        }
    }

    pub fn to(&mut self, target: T) {
        match self {
            Driver::Tween(t) => t.to(target),
            Driver::Spring(s) => s.to(target),
        }
    }

    pub fn value(&self) -> &T {
        match self {
            Driver::Tween(t) => t.value(),
            Driver::Spring(s) => s.value(),
        }
    }

    pub fn target(&self) -> &T {
        match self {
            Driver::Tween(t) => t.target(),
            Driver::Spring(s) => s.target(),
        }
    }

    pub fn is_running(&self) -> bool {
        match self {
            Driver::Tween(t) => t.is_running(),
            Driver::Spring(s) => s.is_running(),
        }
    }

    pub fn stop(&mut self) {
        match self {
            Driver::Tween(t) => t.stop(),
            Driver::Spring(s) => s.stop(),
        }
    }

    pub fn transition(&mut self, transition: Transition) {
        match (self, transition) {
            (Driver::Tween(t), Transition::Tween(spec)) => {
                t.spec = spec;
            }
            (Driver::Spring(s), Transition::Spring(spec)) => {
                s.spec = spec;
            }
            (this, transition) => {
                *this = transition.build(this.value().clone());
            }
        }
    }
}

impl<T: Integrate> Deref for Driver<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Driver::Tween(t) => t.deref(),
            Driver::Spring(s) => s.deref(),
        }
    }
}

impl<T: Integrate + fmt::Display> fmt::Display for Driver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Driver::Tween(t) => t.fmt(f),
            Driver::Spring(s) => s.fmt(f),
        }
    }
}

impl<T> Animation for Driver<T>
where
    T: Integrate + Interpolate + Clone + PartialEq + Distance,
    T::Velocity: Settled,
{
    type Value = T;

    #[inline]
    fn advance(&mut self, time: Time) -> Activity {
        self.advance(time)
    }

    #[inline]
    fn value(&self) -> &T {
        self.value()
    }

    #[inline]
    fn target(&self) -> &T {
        self.target()
    }

    #[inline]
    fn to(&mut self, target: T) {
        self.to(target);
    }
}
