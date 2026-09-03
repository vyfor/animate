use crate::driver::Driver;
use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, Settled, SpringSpec};
use crate::tween::TweenSpec;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transition {
    Tween(TweenSpec),
    Spring(SpringSpec),
}

impl Transition {
    pub const fn tween(duration: Duration) -> TweenSpec {
        TweenSpec::new(duration)
    }

    pub const fn linear(duration: Duration) -> TweenSpec {
        TweenSpec::linear(duration)
    }

    pub const fn spring() -> SpringSpec {
        SpringSpec::new(100.0, 10.0, 1.0)
    }

    pub fn build<T>(&self, initial: T) -> Driver<T>
    where
        T: Integrate + Interpolate + Clone + PartialEq + Distance,
        T::Velocity: Settled,
    {
        match self {
            Self::Tween(spec) => Driver::Tween(spec.build(initial)),
            Self::Spring(spec) => Driver::Spring(spec.build(initial)),
        }
    }
}

impl From<TweenSpec> for Transition {
    fn from(spec: TweenSpec) -> Self {
        Self::Tween(spec)
    }
}

impl From<SpringSpec> for Transition {
    fn from(spec: SpringSpec) -> Self {
        Self::Spring(spec)
    }
}
