use justification::Just;
use num::Vec2;
use thiserror::Error;
use yansi::Paint;

pub mod box_chars;
pub mod canvas;
pub mod color;
pub mod justification;
pub mod num;
pub mod prelude;
pub mod result;
pub mod shapes;
pub mod themes;
pub mod widgets;

#[doc(hidden)]
pub use paste::paste;
#[doc(hidden)]
pub use color_hex::color_from_hex;

/// A generic error for the crate
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("tried to access out of bounds position ({0}, {1})")]
    OutOfBounds(isize, isize),
    #[error("given {0} {1} is too large to fit in an isize ({}..={})", isize::MIN, isize::MAX)]
    TooLarge(&'static str, usize),
    #[error("{name} {value} is negative, expected positive")]
    NegativeValue { value: isize, name: &'static str },
    #[error("justification {justification} could not fit object of size {object} in canvas of size {canvas}")]
    JustificationOutOfBounds { canvas: Vec2, object: Vec2, justification: Just },
    #[error("text '{text}' overflew at {ending}. It started at {starting}, but the size of the canvas was only {canvas}")]
    TextOverflow { starting: Vec2, text: String, ending: Vec2, canvas: Vec2 },
    #[error("Object `{name}` didn't have enough space. It started at {pos} with dimensions {size}, but the canvas was only {canvas}")]
    ItemTooBig { pos: Vec2, size: Vec2, canvas: Vec2, name: &'static str },
}

impl From<array2d::Error> for Error {
    fn from(value: array2d::Error) -> Self {
        match value {
            array2d::Error::IndicesOutOfBounds(x, y) => Self::OutOfBounds(
                x.try_into().expect("x value too large to fit into an isize"),
                y.try_into().expect("y value too large to fit into an isize"),
            ),
            _ => unimplemented!(),
        }
    }
}

/// Initializes the library
pub fn init() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }
}
