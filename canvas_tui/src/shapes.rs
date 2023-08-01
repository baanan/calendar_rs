//! Various shapes that can represent items drawn to the canvas
//!
//! These are used inside [`DrawInfo`] to store the last drawn item for [`DrawResultMethods`]

use crate::{prelude::*, canvas, result::{DrawResult, DrawInfo}};
use super::num::{Size, Vec2};

/// A shape that was just drawn to the canvas
pub trait DrawnShape: Sized {
    /// A grown version of this shape
    type Grown: DrawnShape;
    /// A type of boxed function used for drawing to the canvas, see [`draw`](Self::draw)
    type Drawer<C: Canvas<Output = C>>;
    /// Grows the shape by `size`
    fn grow(&self, size: &impl Size) -> Self::Grown;
    /// Expands the shape to `x` and `y` (or the closest it can get to it, if it is a grid), growing
    /// from `from`
    fn expand_to(&self, x: Option<isize>, y: Option<isize>, from: GrowFrom) -> Self::Grown;
    /// Colors a `canvas` using this shape
    ///
    /// # Errors
    ///
    /// - If the shape does not fit on the canvas
    /// - If the coloring has an error, see [`Canvas::highlight`] or [`Canvas::highlight_box`]
    fn color<C: Canvas<Output = C>>(
        self,
        canvas: &mut C,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<C, Self>;
    /// Fills a `canvas` with `chr` in this shape
    ///
    /// # Errors
    ///
    /// - If the shape does not fit on the canvas
    /// - If the filling has an error, see [`Canvas::set`] or [`Canvas::fill_box`]
    fn fill<C: Canvas<Output = C>>(self, canvas: &mut C, chr: char) -> DrawResult<C, Self>;
    /// Uses `drawer` to draw onto the `canvas` within this shape
    ///
    /// For [`Single`] and [`Rect`], the drawer is just given a window into the profile. 
    /// For [`Grid`], the drawer is run on each cell and as such takes in a cell position and the window.
    /// All drawers return a `Result<(), Error>`, which gets propagated out
    ///
    /// # Errors
    ///
    /// - If the shape does not fit on the canvas
    ///     - If a window cannot be made
    /// - If one of the drawers returns an error
    fn draw<C: Canvas<Output = C>>(self, canvas: &mut C, drawer: Self::Drawer<C>) -> DrawResult<C, Self>;
}

/// Determines how a shape should be grown to expand to a certain width or height in
/// [`DrawnShape::expand_to`]
pub enum GrowFrom {
    Center,
    CenterPreferRight,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl GrowFrom {
    /// Grows a rectangle of size `current` at `pos` to the size of `goal`
    ///
    /// Returns the new position
    #[must_use]
    pub fn grow(self, pos: Vec2, current: Vec2, goal: Vec2) -> Vec2 {
        #[allow(clippy::use_self)]
        match self {
            GrowFrom::Center => pos - (goal - current) / 2,
            GrowFrom::CenterPreferRight => pos - (goal - current + 1) / 2,
            GrowFrom::TopLeft => pos,
            GrowFrom::TopRight => pos.sub_x(goal.x - current.x),
            GrowFrom::BottomLeft => pos.sub_y(goal.y - current.y),
            GrowFrom::BottomRight => pos - (goal - current),
        }
    }
}

/// A single position
///
/// Used in [`Canvas::set`] or [`Canvas::highlight`]
#[derive(Debug)]
pub struct Single {
    pub pos: Vec2
}

impl DrawnShape for Single {
    type Grown = Rect;
    type Drawer<C: Canvas<Output = C>> = Box<dyn FnOnce(C::Window<'_>) -> Result<(), Error>>;

    fn grow(&self, by: &impl Size) -> Self::Grown {
        let by = Vec2::from_size(by);
        Rect { pos: self.pos - by, size: by * 2 }
    }

    fn expand_to(&self, x: Option<isize>, y: Option<isize>, from: GrowFrom) -> Self::Grown {
        let size = Vec2::new(x.unwrap_or(1), y.unwrap_or(1));
        Rect { pos: from.grow(self.pos, Vec2::ONE, size), size }
    }
    
    fn color<C: Canvas<Output = C>>(
        self,
        canvas: &mut C,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<C, Self> {
        canvas.highlight(&self.pos, foreground, background)
    }

    fn fill<C: Canvas<Output = C>>(self, canvas: &mut C, chr: char) -> DrawResult<C, Self> {
        canvas.set(&self.pos, chr)
    }

    fn draw<C: Canvas<Output = C>>(self, canvas: &mut C, drawer: Self::Drawer<C>) -> DrawResult<C, Self> {
        let window = canvas.window_absolute(&self.pos, &(1, 1));
        window.and_then(drawer).map(|_| DrawInfo::new(canvas, self))
    }
}

/// A rectangle
///
/// The shape for most items drawn to the canvas including [`text`](Canvas::text), [`rect`](Canvas::rect), and [widgets](Canvas::draw)
#[derive(Debug)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2
}

impl DrawnShape for Rect {
    type Grown = Self;
    type Drawer<C: Canvas<Output = C>> = Box<dyn FnOnce(C::Window<'_>) -> Result<(), Error>>;
    
    fn grow(&self, by: &impl Size) -> Self::Grown {
        let by = Vec2::from_size(by);
        Self { pos: self.pos - by, size: self.size + by * 2 }
    }

    fn expand_to(&self, x: Option<isize>, y: Option<isize>, from: GrowFrom) -> Self::Grown {
        let current = self.size;
        let goal = Vec2::new(x.unwrap_or(current.x), y.unwrap_or(current.y));
        Self { pos: from.grow(self.pos, current, goal), size: goal }
    }
    
    fn color<C: Canvas<Output = C>>(
        self,
        canvas: &mut C,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<C, Self> {
        canvas.highlight_box(&self.pos, &self.size, foreground, background)
    }

    fn fill<C: Canvas<Output = C>>(self, canvas: &mut C, chr: char) -> DrawResult<C, Self> {
        canvas.fill_box(&self.pos, &self.size, chr)
    }

    fn draw<C: Canvas<Output = C>>(self, canvas: &mut C, drawer: Self::Drawer<C>) -> DrawResult<C, Self> {
        let window = canvas.window_absolute(&self.pos, &self.size);
        window.and_then(drawer).map(|_| DrawInfo::new(canvas, self))
    }
}

/// A grid including its dimensions, a spacing between each cell, and the size of each cell
///
/// Only used in [`Canvas::grid`]
#[derive(Debug)]
pub struct Grid {
    pub pos: Vec2,
    pub dims: Vec2,
    pub cell_size: Vec2,
    pub spacing: Vec2,
}

impl Grid {
    /// The full size of the grid from edge to edge
    #[must_use]
    pub fn full_size(&self) -> Vec2 {
        let Self { dims, cell_size, spacing, .. } = *self;
        dims * (cell_size + spacing) + spacing
    }

    #[must_use]
    pub fn cell_size_from_full_size(&self, goal: Vec2) -> Vec2 {
        let Self { dims, spacing, .. } = *self;
        // goal = dims * (cell_size + spacing) + spacing
        // goal - spacing = dims * (cell_size + spacing)
        // (goal - spacing) / dims = cell_size + spacing
        // (goal - spacing) / dims - spacing = cell_size
        (goal - spacing) / dims - spacing
    }
}

impl DrawnShape for Grid {
    type Grown = Self;
    type Drawer<C: Canvas<Output = C>> = Box<dyn Fn(C::Window<'_>, Vec2) -> Result<(), Error>>;

    fn grow(&self, size: &impl Size) -> Self::Grown {
        let size = Vec2::from_size(size);
        Self {
            spacing: self.spacing - size * 2,
            pos: self.pos + size,
            cell_size: self.cell_size + size * 2,
            ..*self
        }
    }

    // keeps the spacing and dims
    // only changes the cell size and position
    // a grid with different dimension for each cell would be helpful for this,
    // but there isn't one
    fn expand_to(&self, x: Option<isize>, y: Option<isize>, from: GrowFrom) -> Self::Grown {
        let current = self.full_size();
        let goal = Vec2::new(x.unwrap_or(current.x), y.unwrap_or(current.y));
        // compute a goal cell size from the goal full size
        let cell_size = self.cell_size_from_full_size(goal);
        // recompute the goal because the new cell size might be less than what is needed (because integers)
        let goal = Self { cell_size, ..*self }.full_size();

        Self {
            pos: from.grow(self.pos, current, goal),
            cell_size,
            ..*self
        }
    }

    fn fill<C: Canvas<Output = C>>(self, canvas: &mut C, chr: char) -> DrawResult<C, Self> {
        let full_spacing = self.cell_size + self.spacing;

        canvas.catch(canvas::check_bounds(self.pos, self.full_size(), canvas, "grid"))?;

        for cell in self.dims {
            let pos = self.pos + cell * full_spacing + self.spacing;
            canvas.fill_box(&pos, &self.cell_size, chr)?;
        }

        Ok(DrawInfo::new(canvas, self))
    }

    fn color<C: Canvas<Output = C>>(
        self,
        canvas: &mut C,
        foreground: impl Into<Option<Color>>,
        background: impl Into<Option<Color>>
    ) -> DrawResult<C, Self> {
        let full_spacing = self.cell_size + self.spacing;

        canvas.catch(canvas::check_bounds(self.pos, self.full_size(), canvas, "grid"))?;

        let foreground = foreground.into();
        let background = background.into();

        for cell in self.dims {
            let pos = self.pos + cell * full_spacing + self.spacing;
            canvas.highlight_box(&pos, &self.cell_size, foreground, background)?;
        }

        Ok(DrawInfo::new(canvas, self))
    }

    fn draw<C: Canvas<Output = C>>(self, canvas: &mut C, drawer: Self::Drawer<C>) -> DrawResult<C, Self> {
        let full_spacing = self.cell_size + self.spacing;
        for cell in self.dims {
            let pos = self.pos + cell * full_spacing + self.spacing;
            let window = canvas.window_absolute(&pos, &self.cell_size);
            window.and_then(|window| drawer(window, cell))?;
        }
        Ok(DrawInfo::new(canvas, self))
    }
}
