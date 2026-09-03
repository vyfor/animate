pub trait Settled {
    fn magnitude(&self) -> f32;

    #[inline]
    fn is_within_epsilon(&self, epsilon: f32) -> bool {
        self.magnitude() < epsilon
    }
}

impl Settled for f32 {
    #[inline]
    fn magnitude(&self) -> f32 {
        self.abs()
    }
}

macro_rules! impl_settled {
    ($($t:ty),* $(,)?) => {
        $(
            impl Settled for $t {
                #[inline]
                fn magnitude(&self) -> f32 {
                    *self as f32
                }
            }
        )*
    };
}

impl_settled!(usize, isize, u64, i64, u32, i32, u16, i16, u8, i8);

macro_rules! impl_settled_array {
    ($($n:expr),* $(,)?) => {
        $(
            impl Settled for [f32; $n] {
                #[inline]
                fn magnitude(&self) -> f32 {
                    self.iter().map(|v| v * v).sum::<f32>().sqrt()
                }
            }
        )*
    };
}

impl_settled_array!(2, 3, 4, 5, 6);
