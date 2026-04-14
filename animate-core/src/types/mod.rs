pub mod num;
pub mod string;
#[cfg(feature = "ratatui")]
pub mod ratatui {
    pub mod color;
}

#[cfg(feature = "ratatui")]
pub use ratatui::*;
