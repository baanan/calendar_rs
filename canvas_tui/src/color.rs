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
}
