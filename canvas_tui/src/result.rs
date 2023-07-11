use std::ops::{Deref, DerefMut};

use crate::Error;
use crate::color::Color;

use super::canvas::{Canvas, Cell};
use super::shapes::{DrawnShape, Grid, Rect, Single};
use super::num::{Pos, Size, Vec2};

/// Holds the current canvas, as well as extra info from the last drawn object
///
/// Also can [dereference](Deref) into the inner canvas
#[derive(Debug)]
pub struct DrawInfo<'c, C: Canvas<Output = C>, S: DrawnShape> {
    output: &'c mut C,
    pub shape: S,
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> DrawInfo<'c, C, S> {
    pub(crate) fn new(output: &'c mut C, shape: S) -> Self {
        Self { output, shape }
    }
}

impl<'c, C: Canvas<Output = C>> DrawInfo<'c, C, Single> {
    pub(crate) fn single(output: &'c mut C, pos: Vec2) -> Self {
        Self { output, shape: Single { pos } }
    }
}

impl<'c, C: Canvas<Output = C>> DrawInfo<'c, C, Rect> {
    pub(crate) fn rect(output: &'c mut C, pos: Vec2, size: Vec2) -> Self {
        Self { output, shape: Rect { pos, size } }
    }
}

impl<'c, C: Canvas<Output = C>> DrawInfo<'c, C, Grid> {
    pub(crate) fn grid(output: &'c mut C, pos: Vec2, dims: Vec2, cell_size: Vec2, spacing: Vec2) -> Self {
        Self { output, shape: Grid { pos, dims, cell_size, spacing } }
    }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> Deref for DrawInfo<'c, C, S> {
    type Target = C;
    fn deref(&self) -> &Self::Target { self.output }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> DerefMut for DrawInfo<'c, C, S> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.output }
}

/// The result of a draw onto a canvas, holding the current canvas as well as some extra info from
/// the last drawn object
///
/// This implements [`Canvas`], forwarding the instructions to the canvas or propagating the error if it exists
///
/// Also see [`DrawResultMethods`]
///
/// # Example
///
/// ```
/// # use canvas_tui::prelude::*;
/// # fn main() -> Result<(), Error> {
/// #
/// let mut canvas = Basic::new(&(5, 5));
///
/// // draw each object to the canvas
/// canvas
///     .set(&(1, 1), 'a')
///     .text_absolute(&(0, 3), "hello")
///         // color the text white
///         .colored(Color::WHITE, None) 
///     .set(&(3, 1), 'b')?;
///
/// // .....
/// // .a.b.
/// // .....
/// // hello
/// // .....
/// assert_eq!(canvas.get(&(1, 3))?.text, 'e');
/// assert_eq!(canvas.get(&(3, 3))?.foreground, Some(Color::WHITE));
/// # Ok(()) }
/// ```
#[allow(clippy::module_name_repetitions)]
pub type DrawResult<'c, C, S> = Result<DrawInfo<'c, C, S>, Error>;

/// Extra methods that can be run on a [`DrawResult`]
pub trait DrawResultMethods<'c, C: Canvas<Output = C>, S: DrawnShape>: Sized {
    /// Colors the last drawn object with `foreground` and `background`
    /// 
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 3));
    /// canvas.text(&Just::Centered, "foo").colored(Color::WHITE, None)?;
    ///
    /// // .....
    /// // .foo.
    /// // .....
    /// assert_eq!(canvas.get(&(2, 1))?.foreground, Some(Color::WHITE));
    /// assert_eq!(canvas.get(&(2, 0))?.foreground, None);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (after [`Self::grow_profile`])
    fn colored(
        self,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<'c, C, S>;
    /// Colors the last drawn object with `foreground`
    ///
    /// See [`Self::colored`]
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (when after [`Self::grow_profile`])
    fn foreground(self, foreground: impl Into<Option<Color>>) -> DrawResult<'c, C, S> {
        self.colored(foreground, None)
    }
    /// Colors the last drawn object with `background`
    ///
    /// See [`Self::colored`]
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (when after [`Self::grow_profile`])
    fn background(self, background: impl Into<Option<Color>>) -> DrawResult<'c, C, S> {
        self.colored(None, background)
    }
    /// Expands the stored profile of the last drawn object, not changing the canvas
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(7, 3));
    /// canvas.text(&Just::Centered, "foo").grow_profile(&(1, 0)).foreground(Color::WHITE)?;
    ///
    /// // .......
    /// // .-foo-. (color represented by -)
    /// // .......
    /// assert_eq!(canvas.get(&(0, 1))?.foreground, None);
    /// assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Color::WHITE));
    /// assert_eq!(canvas.get(&(3, 1))?.foreground, Some(Color::WHITE));
    /// assert_eq!(canvas.get(&(5, 1))?.foreground, Some(Color::WHITE));
    /// assert_eq!(canvas.get(&(6, 1))?.foreground, None);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// - If the result is an error
    fn grow_profile(self, size: &impl Size) -> DrawResult<'c, C, S::Grown>;
    /// Gets the profile of the inside of the last drawn object by shrinking the bounds by 1
    ///
    /// Equivalent to [`Self::grow_bounds(&(-1, -1))`]
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::filled_with_text(&(5, 5), '.');
    ///
    /// canvas.rect(&Just::Centered, &(3, 3), &box_chars::LIGHT)
    ///     .inside().filled(' ');
    ///
    /// // .....
    /// // .┌─┐.
    /// // .│ │.
    /// // .└─┘.
    /// // .....
    /// assert_eq!(canvas.get(&(2, 0))?.text, '.');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// assert_eq!(canvas.get(&(2, 2))?.text, ' ');
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// - If the result is an error
    fn inside(self) -> DrawResult<'c, C, S::Grown>;
    /// Fills the profile with `chr`
    ///
    /// # Example
    ///
    /// See [`Self::inside`]
    ///
    /// # Errors
    ///
    /// - If the result is an error
    fn filled(self, chr: char) -> DrawResult<'c, C, S>;
    /// Fills the inside of the last drawn object with `chr`
    ///
    /// Equivalent to [`Self::grow_bounds(&(-1, -1))`]
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::filled_with_text(&(5, 5), '.');
    ///
    /// canvas.rect(&Just::Centered, &(3, 3), &box_chars::LIGHT)
    ///     .fill_inside(' ');
    ///
    /// // .....
    /// // .┌─┐.
    /// // .│ │.
    /// // .└─┘.
    /// // .....
    /// assert_eq!(canvas.get(&(2, 0))?.text, '.');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// assert_eq!(canvas.get(&(2, 2))?.text, ' ');
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// - If the result is an error
    ///
    /// # Returns
    ///
    /// The result of the draw call
    fn fill_inside(self, chr: char) -> DrawResult<'c, C, S::Grown>;
    /// Ignore the result, especially for when the canvas is using
    /// [`when_error`](Canvas::when_error)
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 5));
    ///
    /// canvas
    ///     .when_error(|canvas, _err| {
    ///         canvas.set(&(1, 1), 'a')?;
    ///         Ok(())
    ///     })
    ///     .set(&(10, 10), 'b') // throws error
    ///     .discard_result(); // supresses #[must_use]
    /// # Ok(()) }
    /// ```
    fn discard_result(&self) {}
    /// Uses `drawer` to draw on the inside of the last drawn object
    ///
    /// See [`DrawnShape::draw`] for more information
    ///
    /// # Errors
    ///
    /// - If the result is already an error
    /// - If the drawer returns an error
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(9, 7));
    ///
    /// canvas
    ///     .grid_absolute(&(1, 1), &(2, 1), &(2, 2), &box_chars::LIGHT)
    ///         .draw_inside(Box::new(|mut canvas, cell| {
    ///             canvas.text(&Just::Centered, &format!("{}{}", cell.x, cell.y))?; 
    ///             Ok(())
    ///         }))?;
    ///
    /// // .........
    /// // .┌──┬──┐.
    /// // .│00│10│.
    /// // .├──┼──┤.
    /// // .│01│11│.
    /// // .└──┴──┘.
    /// // .........
    /// assert_eq!(canvas.get(&(2, 2))?.text, '0');
    /// assert_eq!(canvas.get(&(3, 2))?.text, '0');
    /// assert_eq!(canvas.get(&(5, 2))?.text, '1');
    /// # Ok(()) }
    /// ```
    fn draw_inside(self, drawer: <S::Grown as DrawnShape>::Drawer<C>) -> DrawResult<'c, C, S::Grown>;
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> DrawResultMethods<'c, C, S> for DrawResult<'c, C, S> {
    fn colored(
        self,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<'c, C, S> {
        self.and_then(|DrawInfo { output, shape }|
            shape.color(output, foreground, background)
        )
    }

    fn grow_profile(self, size: &impl Size) -> DrawResult<'c, C, S::Grown> {
        self.map(|DrawInfo { output, shape }|
            DrawInfo { output, shape: shape.grow(size) }
        )
    }

    fn inside(self) -> DrawResult<'c, C, S::Grown> {
        self.map(|DrawInfo { output, shape }| {
            DrawInfo { output, shape: shape.grow(&(-1, -1)) }
        })
    }

    fn filled(self, chr: char) -> DrawResult<'c, C, S> {
        self.and_then(|DrawInfo { output, shape }|
            shape.fill(output, chr)
        )
    }

    fn fill_inside(self, chr: char) -> DrawResult<'c, C, S::Grown> {
        self.and_then(|DrawInfo { output, shape }|
            shape.grow(&(-1, -1)).fill(output, chr)
        )
    }

    fn draw_inside(self, drawer: <S::Grown as DrawnShape>::Drawer<C>) -> DrawResult<'c, C, S::Grown> {
        self.inside().and_then(|DrawInfo { output, shape }|
            shape.draw(output, drawer)
        )
    }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> Size for DrawResult<'c, C, S> {
    fn width(&self) -> isize { self.as_ref().expect("asked for the width of an errored canvas").width() }
    fn height(&self) -> isize { self.as_ref().expect("asked for the height of an errored canvas").height() }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> Canvas for DrawResult<'c, C, S> { 
    type Output = C;
    type Window<'w> = C::Window<'w> where Self: 'w, Self::Output: 'w;

    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut C, Error> {
        match self {
            Ok(canvas) => canvas.set_without_catch(pos, chr),
            Err(err) => Err(err.clone()),
        }
    }

    fn highlight_without_catch(
        &mut self,
        pos: Vec2,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut C, Error> {
        match self {
            Ok(canvas) => canvas.highlight_without_catch(pos, foreground, background),
            Err(err) => Err(err.clone()),
        }
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        match self {
            Ok(canvas) => canvas.get(pos),
            Err(err) => Err(err.clone()),
        }
    }

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<C::Window<'_>, Error> {
        match self {
            Ok(canvas) => canvas.window_absolute(pos, size),
            Err(err) => Err(err.clone()),
        }
    }

    #[allow(clippy::unwrap_used)]
    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self.as_mut().unwrap() }
    fn error(&self) -> Result<(), Error> { self.as_ref().map(|_| ()).map_err(Clone::clone) }
    fn throw(&mut self, err: &Error) {
        if let Ok(canvas) = self { canvas.throw(err) }
    }
}

