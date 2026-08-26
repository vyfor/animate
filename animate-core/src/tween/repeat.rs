#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repeat {
    Once,
    Times(u32),
    Infinite,
}

impl Repeat {
    pub fn cycles(self) -> u64 {
        match self {
            Self::Once => 1,
            Self::Times(n) => (n.max(1)) as u64,
            Self::Infinite => u64::MAX,
        }
    }
}
