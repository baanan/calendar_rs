use std::fmt::Display;

use crate::{num::{Vec2, Size, SignedSize}, canvas::Canvas, Error};

/// Represents the position of an object in relation to the canvas
#[derive(Debug, Clone)]
pub enum Just {
    At(Vec2),
    Centered,

    LeftOfRow(isize),
    RightOfRow(isize),
    CenteredOnRow(isize),

    OffTopLeftBy(isize),
    OffTopRightBy(isize),
    OffBottomLeftBy(isize),
    OffBottomRightBy(isize),

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,

    CenterTop,
    CenterBottom,
    CenterLeft,
    CenterRight,

    OffsetFrom(Box<Just>, Vec2),
    OffsetFromUnchecked(Box<Just>, Vec2),
    AtUnchecked(Vec2),
}

impl Just {
    /// Gets the top left position of an object of size `object` in canvas `canvas` with this justification
    ///
    /// Most positions include a margin of 1 character, unless otherwise input
    ///
    /// # Errors
    ///
    /// - If the object can't fit into the canvas with the justification 
    /// (unless the justification is unchecked)
    pub fn get(&self, canvas: &impl Size, object: &impl Size) -> Result<Vec2, Error> {
        let canvas = Vec2::from_size(canvas).expect("expected a valid canvas size");
        let object = Vec2::from_size(object).expect("expected a valid object size");

        if object.width() > canvas.width() || object.height() > canvas.height() { return self.oob_error(canvas, object) }

        // PERF: test this
        // I don't know if this has a noticeable performance impact,
        // but it makes the code much nicer
        let min = Vec2::from(1);
        let max = canvas - object - 1;
        let center = (canvas - object) / 2;

        let width = canvas.width_signed();
        let height = canvas.height_signed();

        #[allow(clippy::use_self)]
        let pos = match self {
            // basic
            Just::At(pos) => *pos,
            Just::Centered => center,

            // sides of a row
            Just::LeftOfRow(y) => min.with_y(*y),
            Just::RightOfRow(y) => max.with_y(*y),
            Just::CenteredOnRow(y) => center.with_y(*y),

            // corners with a custom margin
            // (add 1 to max to remove the default margin)
            Just::OffTopLeftBy(off) => Vec2::from(*off),
            Just::OffTopRightBy(off) => Vec2::new((max.x + 1) - off, *off),
            Just::OffBottomLeftBy(off) => Vec2::new(*off, (max.y + 1) - off),
            Just::OffBottomRightBy(off) => (max + 1) - Vec2::from(*off),

            // corners with a margin of one
            Just::TopLeft => min,
            Just::TopRight => Vec2::new(max.x, min.y),
            Just::BottomLeft => Vec2::new(min.x, max.y),
            Just::BottomRight => max,

            // centers of the sides
            Just::CenterTop => Vec2::new(center.x, min.y),
            Just::CenterBottom => Vec2::new(center.x, max.y),
            Just::CenterLeft => Vec2::new(min.x, center.y),
            Just::CenterRight => Vec2::new(max.x, center.y),

            // offset
            Just::OffsetFrom(other, offset) => Self::compute_offset(other, *offset, canvas, object)?,
            Just::OffsetFromUnchecked(other, offset) => return Self::compute_offset(other, *offset, canvas, object),
            Just::AtUnchecked(pos) => return Ok(*pos),
        };

        let bottom_right = pos + object;
        // check if the bottom right is out of bounds
        if bottom_right.x > width || bottom_right.y > height { return self.oob_error(canvas, object); }

        Ok(pos)
    }

    fn compute_offset(other: &Self, offset: Vec2, canvas: Vec2, object: Vec2) -> Result<Vec2, Error> {
        other.get(&canvas, &object).map(|val| val + offset)
    }

    fn oob_error(&self, canvas: Vec2, object: Vec2) -> Result<Vec2, Error> {
        Err(Error::JustificationOutOfBounds { canvas, object, justification: self.clone() })
    }

    /// Creates a window in `canvas` that fits an object of size `size` positioned using this justification
    ///
    /// # Errors
    ///
    /// - If the object can't fit into the canvas with the justification
    pub fn window<'a, C: Canvas>(&'a self, canvas: &'a mut C, size: &impl Size) -> Result<C::Window<'_>, Error> {
        canvas.window(self, size)
    }

    /// Offsets this current justification with `offset`
    #[must_use]
    pub fn offset(self, offset: impl Into<Vec2>) -> Self {
        Self::OffsetFrom(Box::new(self), offset.into())
    }

    /// Offsets this current justification with `offset`,
    /// not checking if the offset position is in-bounds.
    ///
    /// This can be used to edit characters outside of the window
    #[must_use]
    pub fn offset_unchecked(self, offset: impl Into<Vec2>) -> Self {
        Self::OffsetFromUnchecked(Box::new(self), offset.into())
    }
}

impl Display for Just {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center() -> Result<(), Error> {
        // .....
        // .oxx.
        // .xxx.
        // .....
        assert_eq!(Just::Centered.get(&(5, 4), &(3, 2))?, (1, 1));
        Ok(())
    }

    #[test]
    fn center_prefers_left() -> Result<(), Error> {
        // .....
        // .ox..
        // .xx..
        // .....
        assert_eq!(Just::Centered.get(&(5, 4), &(2, 2))?, (1, 1));
        Ok(())
    }

    #[test]
    fn at() -> Result<(), Error> {
        let pos = (5, 10);
        assert_eq!(Just::At(pos.into()).get(&(100, 100), &(20, 20))?, pos);
        Ok(())
    }

    #[test]
    fn top_right() -> Result<(), Error> {
        // ......
        // ...ox.
        // ...xx.
        // ......
        // ......
        assert_eq!(Just::TopRight.get(&(6, 5), &(2, 2))?, (3, 1));
        Ok(())
    }

    #[test]
    fn top_right_offset() -> Result<(), Error> {
        // ......./
        // ....../.
        // ....ox..
        // ....xx..
        // ........
        assert_eq!(Just::OffTopRightBy(2).get(&(8, 5), &(2, 2))?, (4, 2));
        Ok(())
    }

    #[test]
    fn out_of_bounds() {
        assert!(matches!(Just::Centered.get(&(2, 2), &(5, 5)), Err(Error::JustificationOutOfBounds { .. })));
    }
}
