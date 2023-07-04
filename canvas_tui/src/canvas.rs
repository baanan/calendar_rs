use std::ops::{Deref, DerefMut};

use crate::{num::{Size, Pos}, justification::Just, prelude::box_chars};

use super::{color::Color, num::Vec2};
use array2d::Array2D;
use itertools::iproduct;
use crate::Error;

// some kind of box module with constant box char arrays
// canvas.rect(&Just::Centered, &(5, 5), box::light)
//
// .stroke(fore, back) and .fill(fore, back)
// stroke change set, fill change shapes
//
// crazy idea
// canvas.text(..).color(..)
// some kind of CanvasResult struct that holds some info
// about the previous draw, but derefs to a result

#[allow(clippy::missing_const_for_fn)]
fn discard_reference<T>(_: T) {}

macro_rules! catch {
    ($s:ident.$($tt:tt)*) => {
        let res = $s.$($tt)*.map(discard_reference);
        $s.catch(res)?;
    };
}

fn check_bounds(pos: Vec2, size: Vec2, canvas: &impl Size, name: &'static str) -> Result<(), Error> {
    let canvas = Vec2::from_size(canvas);
    let outer = pos + size;
    if outer.x > canvas.x || outer.y > canvas.y {
        return Err(Error::ItemTooBig { pos, size, canvas, name })
    }
    Ok(())
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
    fn set(&mut self, pos: &impl Pos, chr: char) -> DrawResult<Self::Output> {
        let pos = Vec2::from_pos(pos);
        let res = self.set_without_catch(pos, chr);
        // this little dance is needed to make sure there isn't multiple mutable borrows between
        // the result and the throw
        if let Err(err) = res { self.throw(&err); Err(err) }
        // this unwrap is fine because the error is already checked
        else { Ok(DrawInfo::single(self.unwrap_base_canvas(), pos)) }
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
    ) -> DrawResult<Self::Output> {
        // see set
        let pos = Vec2::from_pos(pos);
        let res = self.highlight_without_catch(pos, foreground.into(), background.into());
        if let Err(err) = res { self.throw(&err); Err(err) }
        else { Ok(DrawInfo::single(self.unwrap_base_canvas(), pos)) }
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
    fn fill(&mut self, chr: char) -> DrawResult<Self::Output> {
        self.error()?;
        let size = Vec2::from_size(self);
        for pos in iproduct!(0..size.width(), 0..size.height()) {
            catch!(self.set(&pos, chr));
        }
        Ok(DrawInfo::new(self.unwrap_base_canvas(), Vec2::ZERO, size))
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
    ) -> DrawResult<Self::Output> {
        self.error()?;

        let pos = Vec2::from_pos(pos);
        let size = Vec2::from_size(size);
        self.catch(check_bounds(pos, size, self, "highlight"))?;
        
        let foreground = foreground.into();
        let background = background.into();

        for offset in iproduct!(0..size.width(), 0..size.height()) {
            let coord = pos + Vec2::from(offset);
            self.highlight(&coord, foreground, background)?;
        }

        Ok(DrawInfo::new(self.unwrap_base_canvas(), pos, size))
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
    fn text(&mut self, justification: &Just, string: &str) -> DrawResult<Self::Output> {
        self.error()?;
        let size = (string.len(), 1);
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
    fn text_absolute(&mut self, pos: &impl Pos, string: &str) -> DrawResult<Self::Output> {
        self.error()?;

        let canvas = Vec2::from_size(self);
        let pos = Vec2::from_pos(pos);
        for (charnum, chr) in (0..).zip(string.chars()) {
            let charpos = pos.add_x(charnum);
            catch!(self.set_without_catch(charpos, chr)
                // add a nice error
                .map_err(|_| Error::TextOverflow { starting: pos, text: string.to_owned(), ending: charpos, canvas })
            );
        }

        let textsize = self.catch((string.len(), 1).try_into())?;
        Ok(DrawInfo::new(self.unwrap_base_canvas(), pos, textsize))
    }
    /// Draws a box onto the canvas using `justification` with size `size`
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
    fn rect(&mut self, justification: &Just, size: &impl Size, chars: &'static box_chars::Chars) -> DrawResult<Self::Output> {
        self.error()?;
        let pos = self.catch(justification.get(self, size))?;
        self.rect_absolute(&pos, size, chars)
    }
    /// Draws a box onto the canvas at `pos` with size `size`
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
    fn rect_absolute(&mut self, pos: &impl Pos, size: &impl Size, chars: &'static box_chars::Chars) -> DrawResult<Self::Output> {
        self.error()?;

        let size = Vec2::from_size(size);
        let pos = Vec2::from_pos(pos);
        self.catch(check_bounds(pos, size, self, "rect"))?;

        let top = 0;
        let bottom = size.height() - 1;
        let left = 0;
        let right = size.width() - 1;

        for x in (left + 1)..right {
            catch!(self.set(&(pos + (x, top)), chars.horizontal()));
            catch!(self.set(&(pos + (x, bottom)), chars.horizontal()));
        }

        for y in (top + 1)..bottom {
            catch!(self.set(&(pos + (left, y)), chars.vertical()));
            catch!(self.set(&(pos + (right, y)), chars.vertical()));
        }

        catch!(self.set(&(pos + (left, top)),     chars[0b0101]));
        catch!(self.set(&(pos + (right, top)),    chars[0b0110]));
        catch!(self.set(&(pos + (left, bottom)),  chars[0b1001]));
        catch!(self.set(&(pos + (right, bottom)), chars[0b1010]));

        Ok(DrawInfo::new(self.unwrap_base_canvas(), pos, size))
    }
    /// Gets the underlying canvas
    ///
    /// **Note:** This is guaranteed not to panic if calling [`Self::error`] returns [`Ok`]
    ///
    /// **Note:** This is mainly only meant to be used internally, please use [`Result::unwrap`] or
    /// `?` instead
    ///
    /// # Panics
    ///
    /// - If the base canvas isn't owned anymore (such as when piping instructions
    /// on a [`DrawResult`] results with an [`Err`])
    fn unwrap_base_canvas(&mut self) -> &mut Self::Output;
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

    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self }
    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, _err: &Error) { }
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

    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self }
    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) { self.canvas.throw(err) }
}

/// A canvas wrapped with an error catcher callback
///
/// See [`Canvas::when_error`] and [`DrawResultMethods::discard_result`]
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

    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self }
    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) {
        (self.callback)(&mut self.canvas, err)
            .expect("when_error callback threw an error itself, not rerunning to prevent an infinite loop");
    }
}

/// Holds the current canvas, as well as extra info from the last drawn object
///
/// Also can [dereference](Deref) into the inner canvas
pub struct DrawInfo<'c, C: Canvas<Output = C>> {
    output: &'c mut C,
    pos: Vec2,
    size: Vec2,
}

impl<'c, C: Canvas<Output = C>> DrawInfo<'c, C> {
    fn single(output: &'c mut C, pos: Vec2) -> Self {
        Self { output, pos, size: Vec2::from(1) }
    }

    fn new(output: &'c mut C, pos: Vec2, size: Vec2) -> Self {
        Self { output, pos, size }
    }
}

impl<'c, C: Canvas<Output = C>> Deref for DrawInfo<'c, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target { self.output }
}

impl<'c, C: Canvas<Output = C>> DerefMut for DrawInfo<'c, C> {
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
pub type DrawResult<'c, C> = Result<DrawInfo<'c, C>, Error>;

/// Extra methods that can be run on a [`DrawResult`]
pub trait DrawResultMethods<C: Canvas<Output = C>> {
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
    /// - If there is not enough room for the color (after [`Self::grow_bounds`])
    fn colored(&mut self, foreground: impl Into<Option<Color>>, background: impl Into<Option<Color>>) -> DrawResult<C>;
    /// Colors the last drawn object with `foreground`
    ///
    /// See [`Self::colored`]
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (when after [`Self::grow_bounds`])
    fn foreground(&mut self, foreground: impl Into<Option<Color>>) -> DrawResult<C> {
        self.colored(foreground, None)
    }
    /// Colors the last drawn object with `background`
    ///
    /// See [`Self::colored`]
    ///
    /// # Errors
    ///
    /// - If the result is an error
    /// - If there is not enough room for the color (when after [`Self::grow_bounds`])
    fn background(&mut self, background: impl Into<Option<Color>>) -> DrawResult<C> {
        self.colored(None, background)
    }
    /// Expands the stored bounds of the last drawn object, not changing the canvas
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(7, 3));
    /// canvas.text(&Just::Centered, "foo").grow_bounds(&(1, 0)).foreground(Color::WHITE)?;
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
    #[must_use]
    fn grow_bounds(self, size: &impl Size) -> Self;
    /// Ignore the result, especially for when the canvas is using
    /// [`when_error`](Canvas::when_error)
    ///
    /// # Example
    ///
    /// ```
    /// # use canvas_tui::prelude::*;
    /// # fn main() -> Result<(), Error> {
    /// #
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
}

impl<'c, C: Canvas<Output = C>> DrawResultMethods<C> for DrawResult<'c, C> {
    fn colored(&mut self, foreground: impl Into<Option<Color>>, background: impl Into<Option<Color>>) -> DrawResult<C> {
        match self {
            Ok(info) => info.output.highlight_box(&info.pos, &info.size, foreground, background),
            Err(err) => Err(err.clone()),
        }
    }

    fn grow_bounds(self, size: &impl Size) -> Self {
        match self {
            Ok(mut info) => {
                let size = Vec2::from_size(size);
                info.pos -= size;
                info.size += size * 2;
                Ok(info)
            },
            Err(err) => Err(err),
        }
    }
}

impl<'c, C: Canvas<Output = C>> Size for DrawResult<'c, C> {
    fn width(&self) -> isize { self.as_ref().expect("asked for the width of an errored canvas").width() }
    fn height(&self) -> isize { self.as_ref().expect("asked for the height of an errored canvas").height() }
}

impl<'c, C: Canvas<Output = C>> Canvas for DrawResult<'c, C> { 
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
