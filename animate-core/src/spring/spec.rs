use crate::interpolate::Interpolate;
use crate::spring::{Integrate, Spring};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringSpec {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub epsilon: f32,
}

impl Default for SpringSpec {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }
}

impl SpringSpec {
    pub const fn new(stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            stiffness,
            damping,
            mass,
            epsilon: 0.001,
        }
    }

    pub const fn stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness;
        self
    }

    pub const fn damping(mut self, damping: f32) -> Self {
        self.damping = damping;
        self
    }

    pub const fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub const fn epsilon(mut self, epsilon: f32) -> Self {
        self.epsilon = epsilon;
        self
    }

    pub fn build<T>(&self, initial: T) -> Spring<T>
    where
        T: Integrate + Interpolate + Clone + PartialEq,
    {
        Spring::from_spec(*self, initial)
    }

    #[inline]
    pub(crate) fn step(&self, pos: f32, target: f32, vel: f32, dt: f32) -> (f32, f32) {
        let displacement = pos - target;
        let accel = (-self.stiffness * displacement - self.damping * vel) / self.mass;
        let new_vel = vel + accel * dt;
        let new_pos = pos + new_vel * dt;

        (new_pos, new_vel)
    }
}
