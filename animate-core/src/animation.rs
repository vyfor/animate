use crate::{Activity, Time};

pub trait Animation {
    type Value;

    fn advance(&mut self, time: Time) -> Activity;
    fn value(&self) -> &Self::Value;
    fn target(&self) -> &Self::Value;
    fn to(&mut self, target: Self::Value);
}

impl<A: Animation + ?Sized> Animation for &mut A {
    type Value = A::Value;

    #[inline]
    fn advance(&mut self, time: Time) -> Activity {
        (**self).advance(time)
    }

    #[inline]
    fn value(&self) -> &Self::Value {
        (**self).value()
    }

    #[inline]
    fn target(&self) -> &Self::Value {
        (**self).target()
    }

    #[inline]
    fn to(&mut self, target: Self::Value) {
        (**self).to(target)
    }
}
