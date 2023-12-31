//! Utilities based on the result of something drawn to a canvas. See [`DrawResultMethods`].
//!
//! - [`DrawResult`] implements [`Canvas`] based on its underlying canvas, propagating errors if
//! they're encountered. 
//! - [`Canvas::when_error`] can be used to automatically recover from an error,
//! which then [`discard_result`](DrawResultMethods::discard_result) or
//! [`log_result`](DrawResultMethods::log_result) can be helpful. 
//! - Other methods are provided to modify the most recently drawn item such as
//! [`colored`](DrawResultMethods::colored) or [`draw_inside`](DrawResultMethods::draw_inside)

use log::{error, Level};

use crate::Error;
use crate::color::Color;
use crate::shapes::GrowFrom;

use super::canvas::{Canvas, Cell};
use super::shapes::{DrawnShape, Grid, Rect, Single};
use super::num::{Pos, Size, Vec2};

/// Holds the current canvas, as well as extra info from the last drawn object
///
/// Use [`Self::canvas`] or [`Self::canvas_mut`] to access the inner canvas
#[derive(Debug)]
pub struct DrawInfo<'c, C: Canvas<Output = C>, S: DrawnShape> {
    output: &'c mut C,
    pub shape: S,
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> DrawInfo<'c, C, S> {
    pub(crate) fn new(output: &'c mut C, shape: S) -> Self {
        Self { output, shape }
    }

    pub fn canvas(&self) -> &C { self.output }
    pub fn canvas_mut(&mut self) -> &mut C { self.output }
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

// impl<'c, C: Canvas<Output = C>, S: DrawnShape> Deref for DrawInfo<'c, C, S> {
//     type Target = C;
//     fn deref(&self) -> &Self::Target { self.output }
// }

// impl<'c, C: Canvas<Output = C>, S: DrawnShape> DerefMut for DrawInfo<'c, C, S> {
//     fn deref_mut(&mut self) -> &mut Self::Target { self.output }
// }

/// The result of a draw onto a canvas, holding the current canvas as well as some extra info from
/// the last drawn object
///
/// This also implements [`Canvas`], forwarding the instructions to the canvas or propagating the error if it exists
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
///
/// - Methods such as [`colored`](Self::colored) or [`filled_with`](Self::filled_with) can be used to
/// modify the last drawn object.
/// - Other methods such as [`grow_profile`](Self::grow_profile) or [`expand_profile`](Self::expand_profile) can be used to modify the stored profile of the last drawn object, which allows the previous methods to color or fill different areas of the canvas
/// - Some methods are common mixtures of the previous two such as
/// [`fill_inside`](Self::fill_inside) (equivalent to calling [`inside`](Self::inside) then
/// [`filled_with`](Self::filled_with))
/// - The rest allow the user to discard the result if it is already dealt with through
/// [`Canvas::when_error`]
pub trait DrawResultMethods<'c, C: Canvas<Output = C>, S: DrawnShape>: Sized {
    /// Colors the last drawn object with `foreground` and `background`
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (after [`Self::grow_profile`])
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
    /// Fills the profile with `chr`
    ///
    /// # Errors
    ///
    /// - If the result is an error
    ///
    /// # Example
    ///
    /// See [`Self::inside`]
    fn filled_with(self, chr: char) -> DrawResult<'c, C, S>;
    /// Fills the inside of the last drawn object with `chr`
    ///
    /// Equivalent to [`result`](Self)[`.inside()`](Self::inside)[`.filled_with(chr)`](Self::filled_with)
    ///
    /// **Note:** The profile returned is the same as before the method was called
    ///
    /// # Errors
    ///
    /// - If the result is an error
    ///
    /// # Returns
    ///
    /// The result of the draw call
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
    fn fill_inside(self, chr: char) -> DrawResult<'c, C, <S::Grown as DrawnShape>::Grown>;
    /// Expands the stored profile of the last drawn object, not changing the canvas
    ///
    /// # Errors
    ///
    /// - If the result is an error
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
    fn grow_profile(self, size: &impl Size) -> DrawResult<'c, C, S::Grown>;
    /// Expands the canvas to the new x and y, if given
    ///
    /// # Errors
    ///
    /// - If the result is an error
    fn expand_profile(self, x: impl Into<Option<isize>>, y: impl Into<Option<isize>>, from: GrowFrom) -> DrawResult<'c, C, S::Grown>;
    /// Gets the profile of the inside of the last drawn object by shrinking the bounds by 1
    ///
    /// Equivalent to [`result`](Self)[`.grow_bounds(&(-1, -1))`](Self::grow_profile)
    ///
    /// # Errors
    ///
    /// - If the result is an error
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::filled_with_text(&(5, 5), '.');
    ///
    /// canvas.rect(&Just::Centered, &(3, 3), &box_chars::LIGHT)
    ///     .inside().filled_with(' ');
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
    fn inside(self) -> DrawResult<'c, C, S::Grown>;
    /// Uses `drawer` to draw on the inside of the profile
    ///
    /// For [`Single`] and [`Rect`], the drawer is just given a [window](Canvas::window) into the profile. 
    /// For [`Grid`], the drawer is run on each cell and as such takes in a cell position and the window.
    /// See [`DrawnShape::draw`] for more information
    ///
    /// **Note:** The profile returned is the same as before the method was called
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
    /// canvas.grid(&Just::Centered, &(2, 1), &(2, 2), &box_chars::LIGHT)
    ///     .draw_inside(Box::new(|mut canvas, cell| {
    ///         canvas.text(&Just::Centered, &format!("{}{}", cell.x, cell.y))?; 
    ///         Ok(())
    ///     }))?;
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
    fn draw_inside(self, drawer: <S::Grown as DrawnShape>::Drawer<C>) -> DrawResult<'c, C, <S::Grown as DrawnShape>::Grown>;
    /// Ignore the result, especially for when the canvas is using
    /// [`when_error`](Canvas::when_error)
    ///
    /// # Example32
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
    /// Discards the info inside, returning only the possible error
    #[allow(clippy::missing_errors_doc)]
    fn discard_info(self) -> Result<(), Error>;
    /// Logs the error at [`Level::Error`] if it exists and discards the result
    fn log_result(self);
    /// Logs the error at `level` if it exists and discards the result
    fn log_result_with(self, level: Level);
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

    fn expand_profile(self, x: impl Into<Option<isize>>, y: impl Into<Option<isize>>, from: GrowFrom) -> DrawResult<'c, C, <S as DrawnShape>::Grown> {
        self.map(|DrawInfo { output, shape }|
            DrawInfo { output, shape: shape.expand_to(x.into(), y.into(), from) }
        )
    }

    fn inside(self) -> DrawResult<'c, C, S::Grown> {
        self.map(|DrawInfo { output, shape }| {
            DrawInfo { output, shape: shape.grow(&(-1, -1)) }
        })
    }

    fn filled_with(self, chr: char) -> DrawResult<'c, C, S> {
        self.and_then(|DrawInfo { output, shape }|
            shape.fill(output, chr)
        )
    }

    fn fill_inside(self, chr: char) -> DrawResult<'c, C, <S::Grown as DrawnShape>::Grown> {
        self.and_then(|DrawInfo { output, shape }|
            shape.grow(&(-1, -1)).fill(output, chr).grow_profile(&(1, 1))
        )
    }

    fn draw_inside(self, drawer: <S::Grown as DrawnShape>::Drawer<C>) -> DrawResult<'c, C, <S::Grown as DrawnShape>::Grown> {
        self.inside().and_then(|DrawInfo { output, shape }|
            shape.draw(output, drawer).grow_profile(&(1, 1))
        )
    }

    fn discard_info(self) -> Result<(), Error> { self.map(|_| ()) }

    fn log_result(self) {
        if let Err(err) = self {
            error!("{}", err);
        }
    }

    fn log_result_with(self, level: Level) {
        if let Err(err) = self {
            log::log!(level, "{}", err);
        }
    }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> Size for DrawResult<'c, C, S> {
    fn width(&self) -> isize { self.as_ref().expect("asked for the width of an errored canvas").canvas().width() }
    fn height(&self) -> isize { self.as_ref().expect("asked for the height of an errored canvas").canvas().height() }
}

impl<'c, C: Canvas<Output = C>, S: DrawnShape> Canvas for DrawResult<'c, C, S> { 
    type Output = C;
    type Window<'w> = C::Window<'w> where Self: 'w, Self::Output: 'w;

    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut C, Error> {
        match self {
            Ok(info) => info.canvas_mut().set_without_catch(pos, chr),
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
            Ok(info) => info.canvas_mut().highlight_without_catch(pos, foreground, background),
            Err(err) => Err(err.clone()),
        }
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        match self {
            Ok(info) => info.canvas().get(pos),
            Err(err) => Err(err.clone()),
        }
    }

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<C::Window<'_>, Error> {
        match self {
            Ok(info) => info.canvas_mut().window_absolute(pos, size),
            Err(err) => Err(err.clone()),
        }
    }

    fn error(&self) -> Result<(), Error> { self.as_ref().map(|_| ()).map_err(Clone::clone) }
    fn throw(&mut self, err: &Error) {
        if let Ok(info) = self { info.canvas_mut().throw(err) }
    }
    fn base_canvas(&mut self) -> Result<&mut Self::Output, Error> {
        self.as_mut().map(DrawInfo::canvas_mut).map_err(|err| err.clone())
    }
}

