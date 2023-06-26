use crate::{num::{Size, SignedSize}, justification::Just};

use super::{color::Color, num::Vec2};
use array2d::Array2D;
use crate::Error;

// TODO: something like Result, but it always has an Ok
// like an ErrorAccumulator (don't name it that, that's horrible)
// .log_error() -> O, .send_error_to(output) -> O, .into_result() -> Result<O, E>
//
// actually no, I don't think there is a situation where
// someone will rather want to draw parts of the object
// and not just error early
//
// impl Canvas for Result<Canvas, Error>,
// draw stuff if there's a canvas, or pass through the error if it's there
// then have a widget for an error
//
// maybe Basic::new(...).pipe().set(...).draw_if_error(widget.error())
// pipe creates a (Canvas, Result<(), Error>) which again impls canvas
// much like the other result, but always keeps the canvas
//
// no, the Canvas trait doesn't work with that
// maybe make it so Canvas always returns itself?
// and then implement it on Result<Canvas, Error>
//
// just have Canvas always return Result<&mut Canvas, Error>
// and implement Canvas for Result<&mut Canvas, Error>
// which passes through the error

pub trait CanvasOutput<'o, C: Canvas> {
    /// Get the result form of the output
    fn res(self) -> Result<(), Error>;
    /// Get an ok value
    fn ok(canvas: &'o mut C) -> Self;
    /// Should the drawing be skipped (the current canvas has an error)?
    fn return_early(canvas: &'o mut C) -> Option<Self> where Self: Sized;
}

impl<'o, C: Canvas> CanvasOutput<'o, C> for Result<(), Error> {
    fn res(self) -> Result<(), Error> { self }
    fn ok(_canvas: &'o mut C) -> Self { Ok(()) }
    fn return_early(_canvas: &'o mut C) -> Option<Self> { None }
}

pub trait Canvas : Size + Sized {
    type Output<'o>: CanvasOutput<'o, Self> where Self: 'o;

    /// Writes `chr` onto the canvas at `pos`
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn set(&mut self, pos: impl Into<Vec2>, chr: char) -> Self::Output<'_>;
    /// Highlights `pos` with `foreground` and `background`, if they are given
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn highlight(
        &mut self,
        pos: impl Into<Vec2>,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Self::Output<'_>;
    /// Gets the character and highlight at `pos`
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn get(&self, pos: impl Into<Vec2>) -> Result<Cell, Error>;
    /// Creates a window of size `size` onto the canvas at `pos`
    fn window_absolute(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) -> Window;
    /// Creates a window of size `size` onto the canvas at a position determined by `justification`
    ///
    /// # Errors
    ///
    /// - If there is not enough room to create the window
    fn window<'a>(&'a mut self, justification: &'a Just, size: &impl Size) -> Result<Window, Error> {
        Ok(self.window_absolute(justification.get(self, size)?, Vec2::from_size(size).expect("expected object size to be valid")))
    }
    /// Creates a [`PipeableCanvas`] from the current canvas
    fn pipe(self) -> PipeableCanvas<Self> {
        PipeableCanvas::new(self)
    }
    /// Writes some text on the canvas at pos
    fn text_absolute(&mut self, string: &str, justification: &Just) -> Self::Output<'_> {
        if let Some(ret) = Self::Output::return_early(self) { return ret; }
        Self::Output::ok(self)
    }
}

pub struct Cell {
    pub text: char,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

pub struct Basic {
    dims: Vec2,
    text: Array2D<char>,
    foreground: Array2D<Option<Color>>,
    background: Array2D<Option<Color>>,
}

impl Basic {
    pub fn new(size: &impl Size) -> Self {
        Self {
            dims: Vec2::from_size(size).expect("must have valid size"),
            text: Array2D::filled_with(' ', size.width(), size.height()),
            foreground: Array2D::filled_with(None, size.width(), size.height()),
            background: Array2D::filled_with(None, size.width(), size.height()),
        }
    }
}

impl Size for Basic {
    fn width(&self) -> usize { self.dims.width() }
    fn height(&self) -> usize { self.dims.height() }
}

impl Canvas for Basic {
    type Output<'a> = Result<(), Error>;

    fn set(&mut self, pos: impl Into<Vec2>, chr: char) -> Self::Output<'_> {
        let pos = pos.into();
        let (x, y) = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        self.text.set(x, y, chr).map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        Ok(())
    }

    fn highlight(&mut self, pos: impl Into<Vec2>, foreground: Option<Color>, background: Option<Color>) -> Self::Output<'_> {
        let pos = pos.into();
        let pos = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        if matches!(foreground, Some(_)) { self.foreground[pos] = foreground; }
        if matches!(background, Some(_)) { self.background[pos] = background; }   
        Ok(())
    }

    fn get(&self, pos: impl Into<Vec2>) -> Result<Cell, Error> {
        let pos = pos.into();
        if pos.x > self.dims.width_signed() || pos.y > self.dims.height_signed() {
            return Err(Error::OutOfBounds(pos.x, pos.y));
        }
        let pos = pos.try_into()?;
        
        Ok(Cell {
            text: self.text[pos],
            foreground: self.foreground[pos],
            background: self.background[pos],
        })
    }

    fn window_absolute(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) -> Window {
        Window {
            canvas: self,
            offset: pos.into(),
            size: size.into(),
        }
    }
}

pub struct Window<'a> {
    canvas: &'a mut Basic,
    offset: Vec2,
    size: Vec2,
}

impl<'a> Size for Window<'a> {
    fn width(&self) -> usize { self.size.width() }
    fn height(&self) -> usize { self.size.height() }
}

impl<'a> Canvas for Window<'a> {
    type Output<'o> = Result<(), Error> where Self: 'o;

    fn set(&mut self, pos: impl Into<Vec2>, chr: char) -> Self::Output<'_> {
        self.canvas.set(pos.into() + self.offset, chr)
    }

    fn get(&self, pos: impl Into<Vec2>) -> Result<Cell, Error> {
        self.canvas.get(pos.into() + self.offset)
    }

    fn highlight(
        &mut self,
        pos: impl Into<Vec2>,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Self::Output<'_> {
        self.canvas.highlight(pos.into() + self.offset, foreground, background)
    }

    fn window_absolute(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) -> Window {
        Window {
            canvas: self.canvas,
            offset: pos.into() + self.offset,
            size: size.into(),
        }
    }
}

#[must_use = "the pipe may have an error"]
pub struct PipeableCanvas<C: Canvas> {
    canvas: C,
    error: Result<(), Error>,
}

impl<C: Canvas> PipeableCanvas<C> {
    pub fn unwrap(&self) -> &C {
        // TODO: needless clone
        self.error.clone().unwrap();
        &self.canvas
    }
}

impl<'o, C: Canvas> CanvasOutput<'o, PipeableCanvas<C>> for &'o mut PipeableCanvas<C> {
    fn res(self) -> Result<(), Error> {
        self.error.clone()
    }
    fn ok(canvas: &'o mut PipeableCanvas<C>) -> Self {
        canvas
    }
    fn return_early(canvas: &'o mut PipeableCanvas<C>) -> Option<Self> {
        canvas.error.is_err().then(|| canvas)
    }
}

impl<C: Canvas> PipeableCanvas<C> {
    fn new(canvas: C) -> Self {
        Self { canvas, error: Ok(()) }
    }
}

#[allow(unused_variables)]
impl<C: Canvas> Canvas for PipeableCanvas<C> {
    type Output<'a> = &'a mut PipeableCanvas<C> where C: 'a;

    fn set(&mut self, pos: impl Into<Vec2>, chr: char) -> Self::Output<'_> {
        // propagate error
        if self.error.is_err() { return self; }
        let result = self.canvas.set(pos, chr).res();
        if result.is_err() { self.error = result; }
        self
    }

    fn highlight(
        &mut self,
        pos: impl Into<Vec2>,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Self::Output<'_> {
        todo!()
    }

    fn get(&self, pos: impl Into<Vec2>) -> Result<Cell, Error> {
        todo!()
    }

    fn window_absolute(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>) -> Window {
        todo!()
    }

    fn pipe(self) -> PipeableCanvas<Self> {
        panic!("cannot pipe an already piped canvas");
    }
}

impl<C: Canvas> Size for PipeableCanvas<C> {
    fn width(&self) -> usize { self.canvas.width() }
    fn height(&self) -> usize { self.canvas.height() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn oob_set() {
        let mut canvas = Basic::new(&Vec2::new(5, 5));
        assert!(canvas.set((10, 5), 'a')
            .is_err_and(|err| matches!(err, Error::OutOfBounds(10, 5))));
    }

    #[test]
    fn window() -> Result<(), Error> {
        let mut canvas = Basic::new(&Vec2::new(5, 5));
        let mut window = canvas.window_absolute((1, 1), (2, 2));
        window.set((1, 1), 'a')?;
        assert_eq!(canvas.get((2, 2))?.text, 'a');
        Ok(())
    }

    #[test]
    fn pipe() {
        let canvas = Basic::new(&Vec2::new(5, 5));
        canvas.pipe()
            .set((1, 1), 'a')
            .set((2, 2), 'b');
    }

    #[test]
    fn pipe_window() {
        let mut canvas = Basic::new(&Vec2::new(5, 5));
        let window = canvas.window_absolute((1, 1), (2, 2));
        window.pipe()
            .set((1, 1), 'a')
            .unwrap();
    }
}
