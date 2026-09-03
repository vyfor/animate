use crate::interpolate::Interpolate;
use crate::spring::{Distance, Integrate, SpringSpec};

impl Interpolate for f32 {
    #[inline]
    fn lerp(from: &Self, to: &Self, t: f32) -> Self {
        from + (to - from) * t
    }
}

impl Interpolate for f64 {
    #[inline]
    fn lerp(from: &Self, to: &Self, t: f32) -> Self {
        from + (to - from) * t as f64
    }
}

impl Integrate for f32 {
    type Velocity = f32;

    #[inline]
    fn integrate(&self, target: &Self, velocity: &f32, params: SpringSpec, dt: f32) -> (Self, f32) {
        let (pos, vel) = params.step(*self, *target, *velocity, dt);
        (pos, vel)
    }
}

impl Integrate for f64 {
    type Velocity = f32;

    #[inline]
    fn integrate(&self, target: &Self, velocity: &f32, params: SpringSpec, dt: f32) -> (Self, f32) {
        let (pos, vel) = params.step(*self as f32, *target as f32, *velocity, dt);
        (pos as f64, vel)
    }
}

impl Distance for f32 {
    #[inline]
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).abs()
    }
}

impl Distance for f64 {
    #[inline]
    fn distance(&self, other: &Self) -> f32 {
        (*self as f32 - *other as f32).abs()
    }
}

macro_rules! impl_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl Integrate for $t {
                type Velocity = f32;
                #[inline]
                fn integrate(&self, target: &Self, velocity: &f32, params: SpringSpec, dt: f32) -> (Self, f32) {
                    let (pos, vel) = params.step(*self as f32, *target as f32, *velocity, dt);
                    (pos.round() as $t, vel)
                }
            }

            impl Interpolate for $t {
                #[inline]
                fn lerp(from: &Self, to: &Self, t: f32) -> Self {
                    (*from as f32 + (*to as f32 - *from as f32) * t).round() as $t
                }
            }

            impl Distance for $t {
                #[inline]
                fn distance(&self, other: &Self) -> f32 {
                    (*self as f32 - *other as f32).abs()
                }
            }
        )*
    };
}

impl_int!(usize, isize, u64, i64, u32, i32, u16, i16, u8, i8);
