#[cfg(feature = "alloc")]
pub mod string;

pub mod num;

#[cfg(feature = "ratatui")]
pub mod ratatui {
    pub mod color;
    pub mod layout;
    pub mod style;
}

#[cfg(feature = "ratatui")]
pub use ratatui::*;
