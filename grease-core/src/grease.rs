use crate::Lerp;

pub struct Grease<T>
where
    T: Lerp + PartialEq,
{
    start: T,
    end: T,
    duration: f64,
}

impl<T> Grease<T>
where
    T: Lerp + PartialEq,
{
    pub fn new(start: T, end: T, duration: f64) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }

    pub fn set(&mut self, _target: T) {
        todo!()
    }

    pub fn get(&self) -> T {
        todo!()
    }
}
