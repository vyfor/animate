use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Time {
    pub elapsed: Duration,
    pub delta: Duration,
}

impl Time {
    #[inline]
    pub const fn new(elapsed: Duration, delta: Duration) -> Self {
        Self { elapsed, delta }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Clock {
    elapsed: Duration,
}

impl Clock {
    #[inline]
    pub const fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
        }
    }

    #[inline]
    pub const fn from_elapsed(elapsed: Duration) -> Self {
        Self { elapsed }
    }

    #[inline]
    pub fn advance(&mut self, delta: Duration) -> Time {
        self.elapsed = self.elapsed.saturating_add(delta);
        Time {
            elapsed: self.elapsed,
            delta,
        }
    }

    #[inline]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

impl From<Duration> for Clock {
    #[inline]
    fn from(elapsed: Duration) -> Self {
        Self::from_elapsed(elapsed)
    }
}
