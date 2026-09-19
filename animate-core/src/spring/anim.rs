use crate::SpringSpec;

pub trait Integrate: Sized {
    type Velocity: Copy + Default + core::fmt::Debug;

    fn integrate(
        &self,
        target: &Self,
        velocity: &Self::Velocity,
        params: SpringSpec,
        dt: f32,
    ) -> (Self, Self::Velocity);
}
