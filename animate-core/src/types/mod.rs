pub mod num;
pub mod string;

#[cfg(feature = "ratatui")]
pub mod ratatui {
    pub mod color;
    pub mod layout;
}

#[cfg(feature = "ratatui")]
pub use ratatui::*;
