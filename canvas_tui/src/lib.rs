use justification::Just;
use num::Vec2;
use thiserror::Error;

pub mod num;
pub mod canvas;
pub mod color;
pub mod justification;
pub mod prelude;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("tried to access out of bounds position ({0}, {1})")]
    OutOfBounds(isize, isize),
    #[error("given {0} {1} is too large to fit in an isize ({}..={})", isize::MIN, isize::MAX)]
    TooLarge(&'static str, usize),
    #[error("index {0} is negative, expected positive")]
    NegativeIndex(isize),
    #[error("justification {justification} could not fit object of size {object} in canvas of size {canvas}")]
    JustificationOutOfBounds { canvas: Vec2, object: Vec2, justification: Just },
    #[error("text '{text}' overflew at {ending}. It started at {starting}, but the size of the canvas was only {size}")]
    TextOverflow { starting: Vec2, text: String, ending: Vec2, size: Vec2 },
    #[error("Box was too big. It started at {pos} with dimensions {size}, but the canvas was only {canvas}")]
    BoxTooBig { pos: Vec2, size: Vec2, canvas: Vec2 },
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
