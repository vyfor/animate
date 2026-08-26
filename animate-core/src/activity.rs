#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Activity {
    pub changed: bool,
    pub running: bool,
    pub finished: bool,
}

impl Activity {
    pub const NONE: Self = Self { changed: false, running: false, finished: false };
    pub const CHANGED: Self = Self { changed: true, running: false, finished: false };
    pub const RUNNING: Self = Self { changed: false, running: true, finished: false };
    pub const FINISHED: Self = Self { changed: false, running: false, finished: true };

    #[inline]
    pub const fn changed(self) -> bool {
        self.changed
    }

    #[inline]
    pub const fn running(self) -> bool {
        self.running
    }

    #[inline]
    pub const fn finished(self) -> bool {
        self.finished
    }

    #[inline]
    pub const fn any(self) -> bool {
        self.changed || self.running || self.finished
    }
}

impl std::ops::BitOr for Activity {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self {
            changed: self.changed || rhs.changed,
            running: self.running || rhs.running,
            finished: self.finished || rhs.finished,
        }
    }
}

impl std::ops::BitOrAssign for Activity {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
