use std::{ops::{Add, Sub, Neg, Mul, Div}, fmt::Display};

use crate::Error;

/// An immutable pairing of two numbers, most commonly representing either a [position](Pos) or [size](Size) 
///
/// Most operations work on these, and multiplication is element-wise (Hadamard)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec2 { pub x: isize, pub y: isize, }

impl Vec2 {
    #[must_use]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Creates a Vec2 from a generic [`Size`]
    ///
    /// # Errors
    ///
    /// - If the size's width or height is too big to fit into an [`isize`]
    pub fn from_size(size: &impl Size) -> Result<Self, Error> {
        (size.width(), size.height()).try_into()
    }

    /// Creates a Vec2 from a generic [`Pos`]
    pub fn from_pos(pos: &impl Pos) -> Self {
        Self::new(pos.x(), pos.y())
    }

    #[must_use]
    pub const fn with_x(&self, x: isize) -> Self {
        Self { x, ..*self }
    }

    #[must_use]
    pub const fn with_y(&self, y: isize) -> Self {
        Self { y, ..*self }
    }

    #[must_use]
    pub const fn add_x(&self, off: isize) -> Self {
        Self { x: self.x + off, ..*self }
    }

    #[must_use]
    pub const fn add_y(&self, off: isize) -> Self {
        Self { y: self.y + off, ..*self }
    }

    #[must_use]
    pub const fn sub_x(&self, off: isize) -> Self {
        Self { x: self.x - off, ..*self }
    }

    #[must_use]
    pub const fn sub_y(&self, off: isize) -> Self {
        Self { y: self.y - off, ..*self }
    }
}


/// Something that could represent a position
///
/// Most commonly one of:
/// - `(isize, isize)` (for constant positions)
/// - [`Vec2`] (for variable positions)
pub trait Pos {
    fn x(&self) -> isize;
    fn y(&self) -> isize;
}

/// Something that represents or has a size
///
/// Most commonly one of:
/// - `(usize, usize)` (for constant sizes)
/// - [`Vec2`] (for variable sizes)
/// - [`Canvas`](crate::prelude::Canvas)
pub trait Size {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

/// Something that has a size, but signed 
///
/// It's often useful when passing around [`Vec2`]s
pub trait SignedSize {
    fn width_signed(&self) -> isize;
    fn height_signed(&self) -> isize;
}

impl Pos for Vec2 {
    fn x(&self) -> isize { self.x }
    fn y(&self) -> isize { self.y }
}

impl Size for Vec2 {
    fn width(&self) -> usize { self.x.try_into().expect("width given is negative, expected positive") }
    fn height(&self) -> usize { self.y.try_into().expect("height given is negative, expected positive") }
}

impl SignedSize for Vec2 {
    fn width_signed(&self) -> isize { self.x }
    fn height_signed(&self) -> isize { self.y }
}


impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y } = self;
        write!(f, "({x}, {y})")
    }
}


impl From<isize> for Vec2 {
    fn from(value: isize) -> Self {
        Self { x: value, y: value }
    }
}


impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}

impl From<Vec2> for (isize, isize) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}

impl TryFrom<Vec2> for (usize, usize) {
    type Error = crate::Error;
    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        let Vec2 { x, y } = value;
        let x: usize = x.try_into().map_err(|_| Self::Error::NegativeIndex(x))?;
        let y: usize = y.try_into().map_err(|_| Self::Error::NegativeIndex(y))?;
        Ok((x, y))
    }
}

impl TryFrom<(usize, usize)> for Vec2 {
    type Error = crate::Error;
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let (x, y) = value;
        let x = x.try_into().map_err(|_| Self::Error::TooLarge("x value", x))?;
        let y = y.try_into().map_err(|_| Self::Error::TooLarge("y value", y))?;
        Ok(Self { x, y })
    }
}

impl PartialEq<(isize, isize)> for Vec2 {
    fn eq(&self, other: &(isize, isize)) -> bool {
        let (x, y) = *other;
        self.x == x && self.y == y
    }
}

impl Pos for (isize, isize) {
    fn x(&self) -> isize { self.0 }
    fn y(&self) -> isize { self.1 }
}

impl Size for (usize, usize) {
    fn width(&self) -> usize { self.0 }
    fn height(&self) -> usize { self.1 }
}

impl SignedSize for (isize, isize) {
    fn width_signed(&self) -> isize { self.0 }
    fn height_signed(&self) -> isize { self.1 }
}



impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<isize> for Vec2 {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<isize> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

// element-wise / hadamard
impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Div<isize> for Vec2 {
    type Output = Self;
    fn div(self, rhs: isize) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}
