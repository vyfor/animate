use crate::SpringParams;

pub trait Integrate: Sized {
    type Velocity: Copy + Default + std::fmt::Debug;

    fn integrate(
        &self,
        target: &Self,
        velocity: &Self::Velocity,
        params: SpringParams,
        dt: f32,
    ) -> (Self, Self::Velocity);
}
