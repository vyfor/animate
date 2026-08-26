pub trait Interpolate: Sized {
    fn lerp(from: &Self, to: &Self, t: f32) -> Self;
}
