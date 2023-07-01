use crate::{num::{Size, SignedSize, Pos}, justification::Just};

use super::{color::Color, num::Vec2};
use array2d::Array2D;
use crate::Error;

// some kind of box module with constant box char arrays
// canvas.rect(&Just::Centered, &(5, 5), box::light)
//
// .stroke(fore, back) and .fill(fore, back)
// stroke change set, fill change shapes

#[allow(clippy::missing_const_for_fn)]
fn discard_reference<T>(_: T) {}

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
/// canvas.set(&(1, 1), 'a')
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
    type Output: Canvas;
    /// The type of the window of this canvas
    ///
    /// For most cases, this will be [`Window<Self>`]
    /// unless there is some wrapper around the canvas
    type Window<'w>: Canvas where Self: 'w;
    /// Writes `chr` onto the canvas at `pos`, without [catching](Self::catch) any errors.
    ///
    /// **Note:** This is mainly meant to be used internally, see [set](Canvas::set) instead
    ///
    /// # Errors
    ///
    /// - If the index is out of bounds
    fn set_without_catch(&mut self, pos: &impl Pos, chr: char) -> Result<&mut Self::Output, Error>;
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
        pos: &impl Pos,
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
    fn set(&mut self, pos: &impl Pos, chr: char) -> Result<&mut Self::Output, Error> {
        let res = self.set_without_catch(pos, chr);
        // this little dance is needed to make sure there isn't multiple mutable borrows between
        // the result and the throw
        if let Err(err) = res { self.throw(&err); Err(err) }
        // this unwrap is fine because the error is already checked
        else { Ok(self.unwrap_base_canvas()) }
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
    /// #
    /// let mut canvas = Basic::new(&(3, 3));
    /// canvas.highlight(&(1, 1), Some(Color::grayscale(255)), None)?;
    ///
    /// let cell = canvas.get(&(1, 1))?;
    /// assert_eq!(cell.foreground, Some(Color::grayscale(255)));
    /// assert_eq!(cell.background, None);
    /// # Ok(()) }
    /// ```
    fn highlight(
        &mut self,
        pos: &impl Pos,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut Self::Output, Error> {
        // see set
        let res = self.highlight_without_catch(pos, foreground, background);
        if let Err(err) = res { self.throw(&err); Err(err) }
        else { Ok(self.unwrap_base_canvas()) }
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
    /// #
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
    /// #
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
    /// #
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
        self.window_absolute(&justification.get(self, size)?, size)
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
    /// #
    /// let mut canvas = Basic::new(&(5, 3));
    /// canvas.text_absolute(&(0, 1), "hello")?;
    ///
    /// // .....
    /// // hello
    /// // .....
    /// assert_eq!(canvas.get(&(1, 1))?.text, 'e');
    /// # Ok(()) }
    /// ```
    fn text_absolute(&mut self, pos: &impl Pos, string: &str) -> Result<&mut Self::Output, Error> {
        self.error()?;

        let size = self.catch(Vec2::from_size(self))?;
        let pos = Vec2::from_pos(pos);
        for (charnum, chr) in (0..).zip(string.chars()) {
            let charpos = pos.add_x(charnum);
            let res = self.set_without_catch(&charpos, chr)
                // add a nice error
                .map_err(|_| Error::TextOverflow { starting: pos, text: string.to_owned(), ending: charpos, size })
                .map(discard_reference); // discard the mutable reference
            // have to do it like this because both the above and catch need a mutable reference
            self.catch(res)?;
        }

        Ok(self.unwrap_base_canvas())
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
    /// #
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
    /// Gets the underlying canvas
    ///
    /// **Note:** This is guaranteed not to panic if calling [`Self::error`] returns [`Ok`]
    ///
    /// **Note:** This is mainly only meant to be used internally, please use [`Result::unwrap`] or
    /// `?` instead
    ///
    /// # Panics
    ///
    /// - If the base canvas isn't owned anymore, such as when piping instructions
    /// on a [`Result<&mut Canvas, Error>`] results with an [`Err`]
    fn unwrap_base_canvas(&mut self) -> &mut Self::Output;
    /// Gets any errors the canvas has
    ///
    /// This only ever occurs when piping instructions on a [`Result<&mut Canvas, Error>`], unless
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
        Self::filled_with(' ', None, None, size)
    }

    pub fn filled_with_text(chr: char, size: &impl Size) -> Self {
        Self::filled_with(chr, None, None, size)
    }

    pub fn filled_with(
        chr: char,
        foreground: Option<Color>,
        background: Option<Color>,
        size: &impl Size
    ) -> Self {
        Self {
            dims: Vec2::from_size(size).map_err(|err| err.to_string())
                .expect("too big of a dimension for a canvas"),
            text: Array2D::filled_with(chr, size.width(), size.height()),
            foreground: Array2D::filled_with(foreground, size.width(), size.height()),
            background: Array2D::filled_with(background, size.width(), size.height()),
        }
    }
}

impl Size for Basic {
    fn width(&self) -> usize { self.dims.width() }
    fn height(&self) -> usize { self.dims.height() }
}

impl Canvas for Basic {
    type Output = Self;
    type Window<'w> = Window<'w, Self>;

    fn set_without_catch(&mut self, pos: &impl Pos, chr: char) -> Result<&mut Self, Error> {
        let pos = Vec2::from_pos(pos);
        let (x, y) = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        self.text.set(x, y, chr).map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        Ok(self)
    }

    fn highlight_without_catch(&mut self, pos: &impl Pos, foreground: Option<Color>, background: Option<Color>) -> Result<&mut Self, Error> {
        let pos = Vec2::from_pos(pos);
        let pos = pos.try_into().map_err(|_| Error::OutOfBounds(pos.x, pos.y))?;
        if matches!(foreground, Some(_)) { self.foreground[pos] = foreground; }
        if matches!(background, Some(_)) { self.background[pos] = background; }   
        Ok(self)
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        let pos = Vec2::from_pos(pos);
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

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Window<Self>, Error> {
        Window::new(self, pos, size)
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
    pub fn new(canvas: &'a mut C, pos: &impl Pos, size: &impl Size) -> Result<Self, Error> {
        Ok(Window {
            canvas,
            offset: Vec2::from_pos(pos),
            size: Vec2::from_size(size)?,
        })
    }
}

impl<'a, C: Canvas> Size for Window<'a, C> {
    fn width(&self) -> usize { self.size.width() }
    fn height(&self) -> usize { self.size.height() }
}

impl<'a, C: Canvas> Canvas for Window<'a, C> {
    type Output = Self;
    type Window<'w> = Window<'w, C> where Self: 'w;

    fn set_without_catch(&mut self, pos: &impl Pos, chr: char) -> Result<&mut Self, Error> {
        match self.canvas.set_without_catch(&(Vec2::from_pos(pos) + self.offset), chr) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    fn highlight_without_catch(
        &mut self,
        pos: &impl Pos,
        foreground: Option<Color>,
        background: Option<Color>
    ) -> Result<&mut Self, Error> {
        match self.canvas.highlight_without_catch(&(Vec2::from_pos(pos) + self.offset), foreground, background) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    fn get(&self, pos: &impl Pos) -> Result<Cell, Error> {
        self.canvas.get(&(Vec2::from_pos(pos) + self.offset))
    }

    fn window_absolute(&mut self, pos: &impl Pos, size: &impl Size) -> Result<Self::Window<'_>, Error> {
        Window::new(self.canvas, &(Vec2::from_pos(pos) + self.offset), size)
    }

    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self }
    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) { self.canvas.throw(err) }
}

impl<C: Canvas<Output = C>> Canvas for Result<&mut C, Error> {
    type Output = C;
    type Window<'w> = C::Window<'w> where Self: 'w;

    fn set_without_catch(&mut self, pos: &impl Pos, chr: char) -> Result<&mut C, Error> {
        match self {
            Ok(canvas) => canvas.set_without_catch(pos, chr),
            Err(err) => Err(err.clone()),
        }
    }

    fn highlight_without_catch(
        &mut self,
        pos: &impl Pos,
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

impl<C: Canvas> Size for Result<&mut C, Error> {
    fn width(&self) -> usize { self.as_ref().expect("asked for the width of an errored canvas").width() }
    fn height(&self) -> usize { self.as_ref().expect("asked for the height of an errored canvas").height() }
}

/// A canvas wrapped with an error catcher callback
///
/// See [`Canvas::when_error`] and [`CanvasResult::discard_result`]
pub struct ErrorCatcher<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> {
    canvas: C,
    callback: F,
}

impl<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> Canvas for ErrorCatcher<C, F> {
    type Output = Self;
    type Window<'w> = Window<'w, Self> where Self: 'w;

    fn set_without_catch(&mut self, pos: &impl Pos, chr: char) -> Result<&mut Self::Output, Error> {
        self.canvas.set_without_catch(pos, chr)?; 
        Ok(self)
    }

    fn highlight_without_catch(
        &mut self,
        pos: &impl Pos,
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
        Window::new(self, pos, size)
    }

    fn unwrap_base_canvas(&mut self) -> &mut Self::Output { self }
    fn error(&self) -> Result<(), Error> { Ok(()) }
    fn throw(&mut self, err: &Error) {
        (self.callback)(&mut self.canvas, err)
            .expect("when_error callback threw an error itself. Not rerunning to prevent an infinite loop");
    }
}

impl<C: Canvas, F: Fn(&mut C, &Error) -> Result<(), Error>> Size for ErrorCatcher<C, F> {
    fn width(&self) -> usize { self.canvas.width() }
    fn height(&self) -> usize { self.canvas.height() }
}

/// Extra methods that can be run on the result of a function run on a canvas
#[allow(clippy::module_name_repetitions)]
pub trait CanvasResult {
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

impl<C: Canvas> CanvasResult for Result<&mut C, Error> {}

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
            size: Vec2::new(5, 3)
        });
    }

    #[test]
    fn window_error_catch() -> Result<(), Error> {
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
}
