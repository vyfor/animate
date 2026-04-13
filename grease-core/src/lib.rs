pub mod grease;

pub use grease::Grease;

pub trait Lerp {
    fn lerp(start: &Self, end: &Self, t: f64) -> Self;
}
