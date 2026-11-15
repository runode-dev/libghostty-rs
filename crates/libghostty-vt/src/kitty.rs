//! Handling various protocols pioneered by Kitty.
#![cfg_attr(
    feature = "kitty-graphics",
    doc = "",
    doc = "This includes the [Kitty graphics protocol](graphics)."
)]

pub mod graphics;

#[cfg(feature = "kitty-graphics")]
pub use graphics::Graphics;
