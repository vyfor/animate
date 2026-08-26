use crate::{Clock, Time};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static FRAME_TIME: AtomicU64 = AtomicU64::new(0);
static LAST_DELTA: AtomicU64 = AtomicU64::new(0);

pub fn tick(delta_ms: u64) {
    LAST_DELTA.store(delta_ms, Ordering::Relaxed);
    FRAME_TIME.fetch_add(delta_ms, Ordering::Relaxed);
}

pub fn frame_time() -> u64 {
    FRAME_TIME.load(Ordering::Relaxed)
}

pub fn delta() -> f64 {
    LAST_DELTA.load(Ordering::Relaxed) as f64 / 1000.0
}

pub fn time() -> Time {
    Time::new(
        Duration::from_millis(FRAME_TIME.load(Ordering::Relaxed)),
        Duration::from_millis(LAST_DELTA.load(Ordering::Relaxed)),
    )
}

pub fn clock() -> Clock {
    Clock::from_elapsed(Duration::from_millis(FRAME_TIME.load(Ordering::Relaxed)))
}
