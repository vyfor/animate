mod anim;
mod distance;
mod settled;
mod spec;

pub use anim::Integrate;
pub use distance::Distance;
pub use settled::Settled;
pub use spec::SpringSpec;

use crate::interpolate::Interpolate;
use crate::{Activity, Animation, Time};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Spring<T: Integrate> {
    pub spec: SpringSpec,
    current: T,
    target: T,
    velocity: T::Velocity,
    running: bool,
}

impl<T> Spring<T>
where
    T: Integrate + Interpolate + Clone + PartialEq,
{
    pub fn new(initial: T) -> Self {
        Self::from_spec(SpringSpec::default(), initial)
    }

    pub fn from_spec(spec: SpringSpec, initial: T) -> Self {
        Self {
            spec,
            target: initial.clone(),
            current: initial,
            velocity: T::Velocity::default(),
            running: false,
        }
    }

    pub fn spec(mut self, spec: SpringSpec) -> Self {
        self.spec = spec;
        self
    }

    pub fn stiffness(mut self, stiffness: f32) -> Self {
        self.spec.stiffness = stiffness;
        self
    }

    pub fn damping(mut self, damping: f32) -> Self {
        self.spec.damping = damping;
        self
    }

    pub fn mass(mut self, mass: f32) -> Self {
        self.spec.mass = mass;
        self
    }

    pub fn epsilon(mut self, epsilon: f32) -> Self {
        self.spec.epsilon = epsilon;
        self
    }

    pub fn to(&mut self, target: T) {
        if !self.running && target == self.current {
            self.target = target;
            return;
        }
        if self.running && target == self.target {
            return;
        }
        self.target = target;
        self.running = true;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.velocity = T::Velocity::default();
    }

    pub fn finish(&mut self) {
        self.current = self.target.clone();
        self.velocity = T::Velocity::default();
        self.running = false;
    }

    pub fn velocity(&self) -> f32
    where
        T::Velocity: Settled,
    {
        self.velocity.magnitude()
    }
}

impl<T> Spring<T>
where
    T: Integrate + Interpolate + Clone + PartialEq + Distance,
    T::Velocity: Settled,
{
    pub fn advance(&mut self, time: Time) -> Activity {
        if !self.running {
            return Activity::NONE;
        }

        let dt = time.delta.as_secs_f32();
        if dt <= 0.0 {
            return Activity::RUNNING;
        }

        // assume 16ms deltas
        let steps = ((dt / (1.0 / 60.0)).ceil() as u32).clamp(1, 120); // max 2s, todo: revisit in future.
        let h = dt / steps as f32;
        let epsilon = self.spec.epsilon;

        let mut current = self.current.clone();
        let mut velocity = self.velocity;
        let mut settled = false;

        for _ in 0..steps {
            let (next, next_velocity) = current.integrate(&self.target, &velocity, self.spec, h);
            current = next;
            velocity = next_velocity;
            if current.distance(&self.target) < epsilon && velocity.is_within_epsilon(epsilon) {
                settled = true;
                break;
            }
        }

        let changed = current != self.current;

        if settled {
            self.current = self.target.clone();
            self.velocity = T::Velocity::default();
            self.running = false;
            Activity {
                changed,
                running: false,
                finished: true,
            }
        } else {
            self.current = current;
            self.velocity = velocity;
            Activity {
                changed,
                running: true,
                finished: false,
            }
        }
    }
}

impl<T: Integrate + Default> Default for Spring<T>
where
    T: Interpolate + Clone + PartialEq,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Integrate> Deref for Spring<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.current
    }
}

impl<T> fmt::Display for Spring<T>
where
    T: Integrate + Interpolate + Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.current.fmt(f)
    }
}

impl<T> Animation for Spring<T>
where
    T: Integrate + Interpolate + Clone + PartialEq + Distance,
    T::Velocity: Settled,
{
    type Value = T;

    #[inline]
    fn advance(&mut self, time: Time) -> Activity {
        Spring::advance(self, time)
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
        Spring::to(self, target);
    }
}
