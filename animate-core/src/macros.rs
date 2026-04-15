macro_rules! impl_ops {
    ($ty:ident) => {
        impl<T, E, I> std::ops::Deref for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
        {
            type Target = T;
            fn deref(&self) -> &T {
                self.get()
            }
        }

        impl<T, E, I> std::fmt::Display for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default + std::fmt::Display,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.get().fmt(f)
            }
        }

        impl<T, E, I> std::ops::AddAssign<T> for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
            for<'b> &'b T: std::ops::Add<T, Output = T>,
        {
            fn add_assign(&mut self, rhs: T) {
                let v = self.target() + rhs;
                self.set(v);
            }
        }

        impl<T, E, I> std::ops::SubAssign<T> for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
            for<'b> &'b T: std::ops::Sub<T, Output = T>,
        {
            fn sub_assign(&mut self, rhs: T) {
                let v = self.target() - rhs;
                self.set(v);
            }
        }

        impl<T, E, I> std::ops::MulAssign<T> for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
            for<'b> &'b T: std::ops::Mul<T, Output = T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                let v = self.target() * rhs;
                self.set(v);
            }
        }

        impl<T, E, I> std::ops::DivAssign<T> for $ty<T, E, I>
        where
            T: $crate::Lerp + PartialEq + Default,
            E: Fn(f64) -> f64,
            I: Fn(&T, &T, f64) -> T,
            for<'b> &'b T: std::ops::Div<T, Output = T>,
        {
            fn div_assign(&mut self, rhs: T) {
                let v = self.target() / rhs;
                self.set(v);
            }
        }
    };
}

pub(crate) use impl_ops;
