//! Basic colors and coloring support, see [`Color`]

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Self = Self::grayscale(255);
    pub const BLACK: Self = Self::grayscale(0);

    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub const fn grayscale(val: u8) -> Self {
        Self::new(val, val, val)
    }

    #[must_use]
    pub const fn from_array([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b }
    }

    pub fn paint<T: Display>(item: T, foreground: Option<Self>, background: Option<Self>) -> impl Display {
        let mut style = yansi::Paint::new(item);
        if let Some(foreground) = foreground { style = style.fg(foreground.into()); }
        if let Some(background) = background { style = style.bg(background.into()); }
        style
    }
}

impl From<Color> for yansi::Color {
    fn from(value: Color) -> Self {
        Self::RGB(value.r, value.g, value.b)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b }
    }
}

/// Creates a [`Color`] from a hex code string literal, see [`color_hex`] for the implementation
///
/// # Example
///
/// ```
/// # use canvas_tui::prelude::*;
/// assert_eq!(hex!("#ff00ff"), rgb(255, 0, 255));
/// ```
#[macro_export]
macro_rules! hex {
    ($lit:literal) => {
        $crate::prelude::Color::from_array($crate::color_from_hex!($lit))
    };
}

pub use crate::hex;

#[allow(clippy::inline_always)] // it's essentially an alias (hopefully)
#[inline(always)]
#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b)
}
