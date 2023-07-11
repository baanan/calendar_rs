use crate::{prelude::*, canvas, result::{DrawResult, DrawInfo}};
use super::num::{Size, Vec2};

pub trait DrawnShape: Sized {
    type Grown: DrawnShape;
    type Drawer<C: Canvas<Output = C>>;
    /// Grows the shape by `size`
    fn grow(&self, size: &impl Size) -> Self::Grown;
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
    /// For [`Single`] and [`Rect`], the drawer takes in a window. 
    /// For [`Grid`], the drawer is run on each cell and as such takes in a cell position and its window.
    /// All drawers return a `Result<(), Error>`, which gets propagated out
    ///
    /// # Errors
    ///
    /// - If the shape does not fit on the canvas
    ///     - If a window cannot be made
    /// - If one of the drawers returns an error
    fn draw<C: Canvas<Output = C>>(self, canvas: &mut C, drawer: Self::Drawer<C>) -> DrawResult<C, Self>;
}

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

#[derive(Debug)]
pub struct Grid {
    pub pos: Vec2,
    pub dims: Vec2,
    pub cell_size: Vec2,
    pub spacing: Vec2,
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

    fn fill<C: Canvas<Output = C>>(self, canvas: &mut C, chr: char) -> DrawResult<C, Self> {
        let full_spacing = self.cell_size + self.spacing;

        canvas.catch(canvas::check_bounds(self.pos, (self.dims - 1) * full_spacing + self.spacing, canvas, "grid"))?;

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

        canvas.catch(canvas::check_bounds(self.pos, (self.dims - 1) * full_spacing + self.spacing, canvas, "grid"))?;

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
