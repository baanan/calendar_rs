use crate::{num::{Size, Pos}, justification::Just, prelude::box_chars, shapes::Grid, result::{DrawResult, DrawInfo}, widgets::Widget};

use super::{color::Color, num::Vec2, shapes::{Rect, Single}};
use array2d::Array2D;
use itertools::iproduct;
use crate::Error;

#[allow(clippy::missing_const_for_fn)]
fn discard_reference<T>(_: T) {}

macro_rules! catch {
    ($s:ident.$($tt:tt)*) => {
        let res = $s.$($tt)*.map(discard_reference);
        $s.catch(res)?;
    };
}

/// Utility function to check if an object with pos `pos` and size `size` fits in a canvas of size
/// `canvas`. The resulting error will use the name `name` to refer to the object.
#[allow(clippy::missing_errors_doc)]
pub fn check_bounds(pos: Vec2, size: Vec2, canvas: &impl Size, name: &'static str) -> Result<(), Error> {
    let canvas = Vec2::from_size(canvas);
    let outer = pos + size;
    if outer.x > canvas.x || outer.y > canvas.y {
        return Err(Error::ItemTooBig { pos, size, canvas, name })
    }
    Ok(())
}

fn full_grid_size(cell_size: Vec2, dims: Vec2) -> Vec2 {
    (cell_size + 1) * dims + 1
}

/// A cell of a canvas, holding the text and highlight
pub struct Cell {
    pub text: char,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

/// A canvas of text and color
///
/// See [`Basic`] for a generic canvas
///
/// # Examples
///
/// ```
/// # use canvas_tui::prelude::*;
/// # fn main() -> Result<(), Error> {
/// #
/// let mut canvas = Basic::new(&(5, 5));
///
/// canvas
///     .set(&(1, 1), 'a')
///     .set(&(2, 2), 'b')
///     .text_absolute(&(0, 3), "hello")?;
///
/// // .....
/// // .a...
/// // ..b..
/// // hello
/// // .....
/// assert_eq!(canvas.get(&(1, 3))?.text, 'e');
/// # Ok(()) }
/// ```
pub trait Canvas : Size + Sized {
    /// The output canvas
    ///
    /// This is [`Self`] in normal canvases ([`Basic`] or [`Window`]),
    /// but the base canvas for piping instructions through a [`Result`]
    type Output: Canvas<Output = Self::Output>;
    /// The type of the window of this canvas
    ///
    /// For most cases, this will be [`Window<Self>`]
    /// unless there is some wrapper around the canvas
    type Window<'w>: Canvas where Self: 'w, Self::Output: 'w;
    /// Writes `chr` onto the canvas at `pos`, without [catching](Self::catch) any errors.
    ///
    /// **Note:** This is mainly meant to be used internally, see [set](Canvas::set) instead
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut Self::Output, Error>;
    /// Highlights `pos` with `foreground` and `background`, if they are given,
    /// without [catching](Self::catch) any errors.
    ///
    /// **Note:** This is mainly meant to be used internally, see [highlight](Canvas::highlight) instead
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn highlight_without_catch(
        &mut self,
        pos: Vec2,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut Self::Output, Error>;
    /// Writes `chr` onto the canvas at `pos`
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// #
    /// let mut canvas = Basic::new(&(3, 3));
    /// canvas.set(&(1, 1), 'a')?;
    ///
    /// assert_eq!(canvas.get(&(1, 1))?.text, 'a');
    /// # Ok(()) }
    /// ```
    fn set(&mut self, pos: &impl Pos, chr: char) -> DrawResult<Self::Output, Single> {
        let canvas = self.base_canvas()?;
        let pos = Vec2::from_pos(pos);
        let res = canvas.set_without_catch(pos, chr);
        // this little dance is needed to make sure there isn't multiple mutable borrows between
        // the result and the throw
        if let Err(err) = res { canvas.throw(&err); Err(err) }
        // this unwrap is fine because the error is already checked
        else { Ok(DrawInfo::single(canvas, pos)) }
    }
    /// Highlights `pos` with `foreground` and `background`, if they are given
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(3, 3));
    /// canvas.highlight(&(1, 1), Color::grayscale(255), None)?;
    ///
    /// let cell = canvas.get(&(1, 1))?;
    /// assert_eq!(cell.foreground, Some(Color::grayscale(255)));
    /// assert_eq!(cell.background, None);
    /// # Ok(()) }
    /// ```
    fn highlight(
        &mut self,
        pos: &impl Pos,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<Self::Output, Single> {
        let canvas = self.base_canvas()?;
        // see set
        let pos = Vec2::from_pos(pos);
        let res = canvas.highlight_without_catch(pos, foreground.into(), background.into());
        if let Err(err) = res { canvas.throw(&err); Err(err) }
        else { Ok(DrawInfo::single(canvas, pos)) }
    }
    /// Gets the character and highlight at `pos`
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(3, 3));
    /// canvas
    ///     .set(&(1, 1), 'a')
    ///     .highlight(&(1, 1), Some(Color::grayscale(255)), None)?;
    ///
    /// let cell = canvas.get(&(1, 1))?;
    /// assert_eq!(cell.text, 'a');
    /// assert_eq!(cell.foreground, Some(Color::grayscale(255)));
    /// assert_eq!(cell.background, None);
    /// # Ok(()) }
    /// ```
    fn get(&self, pos: &impl Pos) -> Result<Cell, Error>;
    /// Creates a window of size `size` onto the canvas at `pos`
    ///
    /// # Errors
    ///
    /// - If there is an outstanding error in the canvas
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(4, 4)); // marked by .
    /// let mut window = canvas.window_absolute(&(1, 1), &(2, 2))?; // marked by -
    /// window.set(&(1, 1), '*')?;
    ///
    /// // ....
    /// // .--.
    /// // .-*.
    /// // ....
    /// assert_eq!(canvas.get(&(2, 2))?.text, '*');
    /// # Ok(()) }
    /// ```
    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Self::Window<'_>, Error>;
    /// Creates a window of size `size` onto the canvas at a position determined by `justification`
    ///
    /// # Errors
    ///
    /// - If there is not enough room to create the window
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(4, 4)); // marked by .
    /// let mut window = canvas.window(&Just::Centered, &(2, 2))?; // marked by -
    /// window.set(&(1, 1), '*')?;
    ///
    /// // ....
    /// // .--.
    /// // .-*.
    /// // ....
    /// assert_eq!(canvas.get(&(2, 2))?.text, '*');
    /// # Ok(()) }
    /// ```
    fn window<'a>(&'a mut self, justification: &'a Just, size: &impl Size) -> Result<Self::Window<'_>, Error> {
        self.error()?;
        let pos = self.catch(justification.get(self, size))?;
        self.window_absolute(&pos, size)
    }
    /// Attaches a callback to whenever an error is thrown 
    ///
    /// See [`ErrorCatcher`] and [`Canvas::throw`]
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 5))
    ///     .when_error(|canvas, _err| {
    ///         canvas.set(&(1, 1), 'a')?;
    ///         Ok(())
    ///     });
    ///
    /// let res = canvas.set(&(10, 10), 'b'); // throws error
    ///
    /// assert!(matches!(res, Err(Error::OutOfBounds(..))));
    /// assert_eq!(canvas.get(&(1, 1))?.text, 'a'); // when_error was run
    /// # Ok(()) }
    /// ```
    fn when_error<F: Fn(&mut Self, &Error) -> Result<(), Error>>(self, callback: F) -> ErrorCatcher<Self, F> {
        ErrorCatcher { canvas: self, callback }
    }
    /// Prints the canvas without color to stdout
    ///
    /// # Errors
    ///
    /// - If the canvas has an outstading error (see [`DrawResult`])
    fn print_monochrome(&self) -> Result<(), Error> {
        self.error()?;
        let canvas = Vec2::from_size(self);
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                print!("{}", self.get(&(x, y)).expect("in-bounds get to not fail").text);
            }
            println!();
        }
        Ok(())
    }
    /// Prints the canvas with color to stdout
    ///
    /// # Errors
    ///
    /// - If the canvas has an outstading error (see [`DrawResult`])
    fn print(&self) -> Result<(), Error> {
        self.error()?;
        let canvas = Vec2::from_size(self);
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let cell = self.get(&(x, y)).expect("in-bounds get to not fail");
                print!("{}", Color::paint(cell.text, cell.foreground, cell.background));
            }
            println!();
        }
        Ok(())
    }
    /// Fills the canvas with `chr`
    ///
    /// # Errors
    ///
    /// - If the canvas has an outstading error (see [`DrawResult`])
    fn fill(&mut self, chr: char) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;
        let size = Vec2::from_size(canvas);
        for pos in iproduct!(0..size.width(), 0..size.height()) {
            canvas.set(&pos, chr)?;
        }
        Ok(DrawInfo::rect(canvas, Vec2::ZERO, size))
    }
    /// Highlights a box of the canvas starting at `pos` and extending bottom right for `size`
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 5));
    /// canvas.highlight_box(&(1, 1), &(3, 3), Color::WHITE, None)?; // represented by █
    ///
    /// // .....
    /// // .███.
    /// // .███.
    /// // .███.
    /// // .....
    /// assert_eq!(canvas.get(&(2, 2))?.foreground, Some(Color::WHITE));
    /// assert_eq!(canvas.get(&(0, 0))?.foreground, None);
    /// # Ok(()) }
    /// ```
    fn highlight_box(
        &mut self,
        pos: &impl Pos,
        size: &impl Size,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;

        let pos = Vec2::from_pos(pos);
        let size = Vec2::from_size(size);
        canvas.catch(check_bounds(pos, size, canvas, "highlight"))?;
        
        let foreground = foreground.into();
        let background = background.into();

        for offset in iproduct!(0..size.width(), 0..size.height()) {
            let coord = pos + Vec2::from(offset);
            canvas.highlight(&coord, foreground, background)?;
        }

        Ok(DrawInfo::rect(canvas, pos, size))
    }
    /// Sets a box of the canvas with `chr` starting at `pos` and extending bottom right for `size`
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 4));
    /// canvas.fill_box(&(1, 1), &(3, 2), 'x')?;
    ///
    /// // .....
    /// // .xxx.
    /// // .xxx.
    /// // .....
    /// assert_eq!(canvas.get(&(2, 2))?.text, 'x');
    /// assert_eq!(canvas.get(&(0, 0))?.text, ' ');
    /// # Ok(()) }
    /// ```
    fn fill_box(
        &mut self,
        pos: &impl Pos,
        size: &impl Size,
        chr: char,
    ) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;

        let pos = Vec2::from_pos(pos);
        let size = Vec2::from_size(size);
        canvas.catch(check_bounds(pos, size, canvas, "highlight"))?;

        for offset in iproduct!(0..size.width(), 0..size.height()) {
            let coord = pos + Vec2::from(offset);
            canvas.set(&coord, chr)?;
        }

        Ok(DrawInfo::rect(canvas, pos, size))
    }
    /// Writes some text on the canvas at `pos`
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 3));
    /// canvas.text(&Just::Centered, "hello")?;
    ///
    /// // .....
    /// // hello
    /// // .....
    /// assert_eq!(canvas.get(&(1, 1))?.text, 'e');
    /// # Ok(()) }
    /// ```
    fn text(&mut self, justification: &Just, string: &str) -> DrawResult<Self::Output, Rect> {
        self.error()?;
        let len = string.chars().count()
            .try_into()
            .map_err(|_| Error::TooLarge("string length", string.len()));
        let size = (self.catch(len)?, 1);
        let pos = self.catch(justification.get(self, &size))?;
        self.text_absolute(&pos, string)
    }
    /// Writes some text on the canvas at `pos`
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 3));
    /// canvas.text_absolute(&(0, 1), "hello")?;
    ///
    /// // .....
    /// // hello
    /// // .....
    /// assert_eq!(canvas.get(&(1, 1))?.text, 'e');
    /// # Ok(()) }
    /// ```
    fn text_absolute(&mut self, pos: &impl Pos, string: &str) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;

        let canvas_size = Vec2::from_size(canvas);
        let pos = Vec2::from_pos(pos);
        for (charnum, chr) in (0..).zip(string.chars()) {
            let charpos = pos.add_x(charnum);
            catch!(canvas.set_without_catch(charpos, chr)
                // add a nice error
                .map_err(|_| Error::TextOverflow { starting: pos, text: string.to_owned(), ending: charpos, canvas: canvas_size })
            );
        }

        let textsize = canvas.catch((string.chars().count(), 1).try_into())?;
        Ok(DrawInfo::rect(canvas, pos, textsize))
    }
    /// Draws a box onto the canvas using `justification` with size `size`
    ///
    /// See `DrawResultMethods::draw_inside` to draw on the inside of the rect
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    /// 
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 5));
    /// canvas.rect(&Just::Centered, &(3, 3), &box_chars::LIGHT)?;
    ///
    /// // .....
    /// // .┌─┐.
    /// // .│.│.
    /// // .└─┘.
    /// // .....
    /// assert_eq!(canvas.get(&(1, 1))?.text, '┌');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// # Ok(()) }
    /// ```
    fn rect(&mut self, justification: &Just, size: &impl Size, chars: &'static box_chars::Chars) -> DrawResult<Self::Output, Rect> {
        self.error()?;
        let pos = self.catch(justification.get(self, size))?;
        self.rect_absolute(&pos, size, chars)
    }
    /// Draws a box onto the canvas at `pos` with size `size`
    ///
    /// See `DrawResultMethods::draw_inside` to draw on the inside of the rect
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    /// 
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(5, 5));
    /// canvas.rect_absolute(&(1, 1), &(3, 3), &box_chars::LIGHT)?;
    ///
    /// // .....
    /// // .┌─┐.
    /// // .│.│.
    /// // .└─┘.
    /// // .....
    /// assert_eq!(canvas.get(&(1, 1))?.text, '┌');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// # Ok(()) }
    /// ```
    fn rect_absolute(&mut self, pos: &impl Pos, size: &impl Size, chars: &'static box_chars::Chars) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;

        let size = Vec2::from_size(size);
        let pos = Vec2::from_pos(pos);
        canvas.catch(check_bounds(pos, size, canvas, "rect"))?;

        let top = 0;
        let bottom = size.height() - 1;
        let left = 0;
        let right = size.width() - 1;

        for x in (left + 1)..right {
            canvas.set(&(pos + (x, top)), chars.horizontal())?;
            canvas.set(&(pos + (x, bottom)), chars.horizontal())?;
        }

        for y in (top + 1)..bottom {
            canvas.set(&(pos + (left, y)), chars.vertical())?;
            canvas.set(&(pos + (right, y)), chars.vertical())?;
        }

        // set corners                             udlr
        canvas.set(&(pos + (left, top)),     chars[0b0101])?;
        canvas.set(&(pos + (right, top)),    chars[0b0110])?;
        canvas.set(&(pos + (left, bottom)),  chars[0b1001])?;
        canvas.set(&(pos + (right, bottom)), chars[0b1010])?;

        Ok(DrawInfo::rect(canvas, pos, size))
    }
    /// Draws a box onto the canvas with justification `just`, grid dimensions `dims`, cell size
    /// `cell_size`, and using box chars `chars` 
    ///
    /// See `DrawResultMethods::draw_inside` to draw on the inside of the grid
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(9, 7));
    /// canvas.grid(&Just::Centered, &(2, 1), &(2, 2), &box_chars::LIGHT)?;
    ///
    /// // .........
    /// // .┌──┬──┐.
    /// // .│..│..│.
    /// // .├──┼──┤.
    /// // .│..│..│.
    /// // .└──┴──┘.
    /// // .........
    /// assert_eq!(canvas.get(&(1, 1))?.text, '┌');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// assert_eq!(canvas.get(&(1, 3))?.text, '├');
    /// assert_eq!(canvas.get(&(4, 3))?.text, '┼');
    /// # Ok(()) }
    /// ```
    fn grid(
        &mut self,
        justification: &Just,
        cell_size: &impl Size,
        dims: &impl Size,
        chars: &'static box_chars::Chars
    ) -> DrawResult<Self::Output, Grid> {
        self.error()?;
        let cell_size = Vec2::from_size(cell_size);
        let dims = Vec2::from_size(dims);
        let pos = self.catch(justification.get(self, &full_grid_size(cell_size, dims)))?;
        self.grid_absolute(&pos, &cell_size, &dims, chars)
    }
    /// Draws a box onto the canvas starting at `pos` with grid dimensions `dims`, cell size
    /// `cell_size`, and using box chars `chars` 
    ///
    /// See `DrawResultMethods::draw_inside` to draw on the inside of the grid
    ///
    /// # Errors
    ///
    /// - If there isn't enough space
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(9, 7));
    /// canvas.grid_absolute(&(1, 1), &(2, 1), &(2, 2), &box_chars::LIGHT)?;
    ///
    /// // .........
    /// // .┌──┬──┐.
    /// // .│..│..│.
    /// // .├──┼──┤.
    /// // .│..│..│.
    /// // .└──┴──┘.
    /// // .........
    /// assert_eq!(canvas.get(&(1, 1))?.text, '┌');
    /// assert_eq!(canvas.get(&(2, 1))?.text, '─');
    /// assert_eq!(canvas.get(&(1, 3))?.text, '├');
    /// assert_eq!(canvas.get(&(4, 3))?.text, '┼');
    /// # Ok(()) }
    /// ```
    fn grid_absolute(
        &mut self,
        pos: &impl Pos,
        cell_size: &impl Size,
        dims: &impl Size,
        chars: &'static box_chars::Chars
    ) -> DrawResult<Self::Output, Grid> {
        let canvas = self.base_canvas()?;

        let pos = Vec2::from_pos(pos);
        let cell_size = Vec2::from_size(cell_size);
        let dims = Vec2::from_size(dims);
        let full_size = full_grid_size(cell_size, dims);
        canvas.catch(check_bounds(pos, full_size, canvas, "grid"))?;

        let top = 0;
        let bottom = full_size.height() - 1;
        let left = 0;
        let right = full_size.width() - 1;

        // outer rectangle
        canvas.rect_absolute(&pos, &full_size, chars)?;

        // middle horizontal lines
        for horizontal in 1..dims.y {
            let y = horizontal * (cell_size.y + 1);
            canvas.set(&(pos + (left, y)), chars[0b1101])?;
            canvas.set(&(pos + (right, y)), chars[0b1110])?;
            for x in (left + 1)..right {
                canvas.set(&(pos + (x, y)), chars.horizontal())?;
            }
        }

        // middle vertical lines
        for vertical in 1..dims.x {
            let x = vertical * (cell_size.x + 1);
            canvas.set(&(pos + (x, top)), chars[0b0111])?;
            canvas.set(&(pos + (x, bottom)), chars[0b1011])?;
            for y in (top + 1)..bottom {
                canvas.set(&(pos + (x, y)), chars.vertical())?;
            }
        }

        // intersections
        for intersection in dims - 1 {
            let pos = pos + (intersection + 1) * (cell_size + 1);
            canvas.set(&pos, chars[0b1111])?;
        }

        // the grid returned fills up the entire grid including the outlines
        // so there's some overlap
        Ok(DrawInfo::grid(canvas, pos + 1, dims, cell_size + 2, Vec2::new(-1, -1)))
    }
    /// Draws a [widget](Widget) onto the canvas using `justification`
    ///
    /// # Errors
    ///
    /// - If the widget doesn't have enough space
    fn draw<W: Widget>(&mut self, justification: &Just, widget: W) -> DrawResult<Self::Output, Rect> {
        let canvas = self.base_canvas()?;
        let size = widget.size(canvas)?;
        let pos = justification.get(canvas, &size)?;
        canvas.catch(check_bounds(pos, size, canvas, W::name()))?;
        widget.draw(&mut canvas.window_absolute(&pos, &size)?)?;
        Ok(DrawInfo::rect(canvas, pos, size))
    }
    /// Gets any errors the canvas has
    ///
    /// This only ever occurs when piping instructions on a [`DrawResult`], unless
    /// a foreign type uses it as well
    ///
    /// **Note:** This is mainly only meant to be used internally in order to propagate errors
    #[allow(clippy::missing_errors_doc)]
    fn error(&self) -> Result<(), Error>;
    /// [Throws](Canvas::throw) on an error if it exists
    ///
    /// **Note:** This is mainly only meant to be used internally, all methods already catch any
    /// errors they encounter
    #[allow(clippy::missing_errors_doc)]
    fn catch<T>(&mut self, res: Result<T, Error>) -> Result<T, Error> {
        if let Err(ref err) = res {
            self.throw(err);
        }
        res
    }
    /// Handles the throwing of an error
    ///
    /// See [`Canvas::when_error`] and [`ErrorCatcher`]
    ///
    /// **Note:** This is mainly only meant to be used internally, all methods already catch any
    /// errors they encounter
    fn throw(&mut self, err: &Error);
    /// Gets the underlying canvas past the potential result (as in a [`DrawResult`])
    ///
    /// **Note:** This is mainly only meant to be used internally, please use [`Result::unwrap`] or
    /// `?` instead
    ///
    /// # Errors
    ///
    /// - If the current canvas [has an error](Self::error)
    fn base_canvas(&mut self) -> Result<&mut Self::Output, Error>;
}

/// A basic canvas, holds the text and highlights in 2d arrays
// PERF: I don't know if it's better to have seperated 2d arrays or a 2d array of cells
pub struct Basic {
    dims: Vec2,
    text: Array2D<char>,
    foreground: Array2D<Option<Color>>,
    background: Array2D<Option<Color>>,
}

impl Basic {
    pub fn new(size: &impl Size) -> Self {
        Self::filled_with(size, ' ', None, None)
    }

    pub fn filled_with_text(size: &impl Size, chr: char) -> Self {
        Self::filled_with(size, chr, None, None)
    }

    pub fn filled_with(
        size: &impl Size,
        chr: char,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>,
    ) -> Self {
        let width = size.width_unsigned().expect("width to be valid");
        let height = size.height_unsigned().expect("height to be valid");

        Self {
            dims: Vec2::from_size(size),
            text: Array2D::filled_with(chr, width, height),
            foreground: Array2D::filled_with(foreground.into(), width, height),
            background: Array2D::filled_with(background.into(), width, height),
        }
    }
}

impl Size for Basic {
    fn width(&self) -> isize { self.dims.width() }
    fn height(&self) -> isize { self.dims.height() }
}

impl Canvas for Basic {
    type Output = Self;
    type Window<'w> = Window<'w, Self>;

    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut Self, Error> {
        let (x, y) = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        self.text.set(x, y, chr).map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        Ok(self)
    }

    fn highlight_without_catch(&mut self, pos: Vec2, foreground: Option<Color>, background: Option<Color>) -> Result<&mut Self, Error> {
        let (x, y) = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        if matches!(foreground, Some(_)) { self.foreground.set(x, y, foreground).map_err(|_| Error::OutOfBounds(pos.x, pos.y))?; }
        if matches!(background, Some(_)) { self.background.set(x, y, background).map_err(|_| Error::OutOfBounds(pos.x, pos.y))?; }
        Ok(self)
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        let pos = Vec2::from_pos(pos);
        if pos.x > self.dims.width() || pos.y > self.dims.height() {
            return Err(Error::OutOfBounds(pos.x, pos.y));
        }
        let pos = pos.try_into()?;
        
        Ok(Cell {
            text: self.text[pos],
            foreground: self.foreground[pos],
            background: self.background[pos],
        })
    }

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Window<Self>, Error> {
        Ok(Window::new(self, pos, size))
    }

    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, _err: &Error) { }
    fn base_canvas(&mut self) -> Result<&mut Self::Output, Error> { Ok(self) }
}

/// A window into another canvas
///
/// See [`Canvas::window`]
///
/// Implemented by offseting [`Canvas::set`] calls and returning a different size
pub struct Window<'a, C: Canvas> {
    canvas: &'a mut C,
    offset: Vec2,
    size: Vec2,
}

impl<'a, C: Canvas> Window<'a, C> {
    /// Creates a new window
    ///
    /// # Errors
    ///
    /// - If the size cannot fit into a Vec2
    fn new(canvas: &'a mut C, pos: &impl Pos, size: &impl Size) -> Self {
        Window {
            canvas,
            offset: Vec2::from_pos(pos),
            size: Vec2::from_size(size),
        }
    }
}

impl<'a, C: Canvas> Size for Window<'a, C> {
    fn width(&self) -> isize { self.size.width() }
    fn height(&self) -> isize { self.size.height() }
}

impl<'a, C: Canvas> Canvas for Window<'a, C> {
    type Output = Self;
    type Window<'w> = Window<'w, C> where Self: 'w;

    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut Self, Error> {
        match self.canvas.set_without_catch(pos + self.offset, chr) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    fn highlight_without_catch(
        &mut self,
        pos: Vec2,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut Self, Error> {
        match self.canvas.highlight_without_catch(pos + self.offset, foreground, background) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        self.canvas.get(&(Vec2::from_pos(pos) + self.offset))
    }

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Self::Window<'_>, Error> {
        Ok(Window::new(self.canvas, &(Vec2::from_pos(pos) + self.offset), size))
    }

    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) { self.canvas.throw(err) }
    fn base_canvas(&mut self) -> Result<&mut Self::Output, Error> { Ok(self) }
}

/// A canvas wrapped with an error catcher callback
///
/// See [`Canvas::when_error`] and
/// [`DrawResultMethods::discard_result`](crate::result::DrawResultMethods::discard_result)
pub struct ErrorCatcher<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> {
    canvas: C,
    callback: F,
}

impl<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> Size for ErrorCatcher<C, F> {
    fn width(&self) -> isize { self.canvas.width() }
    fn height(&self) -> isize { self.canvas.height() }
}

impl<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> Canvas for ErrorCatcher<C, F> {
    type Output = Self;
    type Window<'w> = Window<'w, Self> where Self: 'w;

    fn set_without_catch(&mut self, pos: Vec2, chr: char) -> Result<&mut Self::Output, Error> {
        self.canvas.set_without_catch(pos, chr)?; 
        Ok(self)
    }

    fn highlight_without_catch(
        &mut self,
        pos: Vec2,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut Self::Output, Error> {
        self.canvas.highlight_without_catch(pos, foreground, background)?;
        Ok(self)
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> { self.canvas.get(pos) }

    // the window has to specifically wrap around the ErrorCatcher
    // so the throws can be redirected here
    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Self::Window<'_>, Error> {
        Ok(Window::new(self, pos, size))
    }

    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) {
        (self.callback)(&mut self.canvas, err)
            .expect("when_error callback threw an error itself, not rerunning to prevent an infinite loop");
    }
    fn base_canvas(&mut self) -> Result<&mut Self::Output, Error> { Ok(self) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn oob_set() {
        let mut canvas = Basic::new(&(5, 5));
        assert!(canvas.set(&(10, 5), 'a')
            .is_err_and(|err| matches!(err, Error::OutOfBounds(10, 5))));
    }

    #[test]
    fn text_overflow() {
        let mut canvas = Basic::new(&(5, 3));
        // .....
        // ..hello
        // .....
        let res = canvas.text_absolute(&(2, 1), "hello");
        assert!(res.is_err());
        let err = res.err().expect("asserted");
        assert_eq!(err, Error::TextOverflow {
            starting: Vec2::new(2, 1),
            text: "hello".into(),
            ending: Vec2::new(5, 1),
            canvas: Vec2::new(5, 3)
        });
    }

    #[test]
    fn when_error_on_base_catches_window_error() -> Result<(), Error> {
        let mut canvas = Basic::new(&(5, 5))
            .when_error(|canvas, _| {
                canvas.set(&(1, 1), 'e')?; 
                Ok(())
            });

        let mut window = canvas.window_absolute(&(1, 1), &(3, 3))?;

        let res = window.set(&(10, 10), 'a'); // throws error

        assert!(res.is_err_and(|err| matches!(err, Error::OutOfBounds(..))));
        assert_eq!(canvas.get(&(1, 1))?.text, 'e'); // when_error was run

        Ok(())
    }

    #[test]
    fn error_catch_on_window() -> Result<(), Error> {
        let mut canvas = Basic::new(&(5, 5));

        let mut window = canvas.window_absolute(&(1, 1), &(3, 3))?
            .when_error(|canvas, _| {
                canvas.set(&(0, 0), 'e')?; 
                Ok(())
            });

        let res = window.set(&(10, 10), 'a'); // throws error

        // .....
        // .e--.
        // .---.
        // .---.
        // .....
        assert!(res.is_err_and(|err| matches!(err, Error::OutOfBounds(..))));
        assert_eq!(canvas.get(&(1, 1))?.text, 'e'); // when_error was run

        Ok(())
    }
}
