pub trait Distance {
    fn distance(&self, other: &Self) -> f32;
}
