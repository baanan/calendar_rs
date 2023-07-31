//! A set of widgets with the same style as [`basic`], but with colors determined by the theme
//!
//! # Example
//!
//! ```
//! use canvas_tui::prelude::*;
//!
//! struct Frappe;
//!
//! impl Frappe {
//!     pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
//!     pub const fn base() -> Color { Color::new(48, 52, 70) }
//! }
//!
//! impl widgets::Theme for Frappe {
//!     fn title_fg(&self) -> Color { Self::base() }
//!     fn title_bg(&self) -> Color { Self::rosewater() }
//!     // other colors omitted
//!     # fn text(&self) -> Color { todo!() }
//!     # fn highlight_fg(&self) -> Color { todo!() }
//!     # fn button_fg(&self) -> Color { todo!() }
//!     # fn button_bg(&self) -> Color { todo!() }
//!     # fn titled_text_title_fg(&self) -> Color { todo!() }
//!     # fn titled_text_title_bg(&self) -> Color { todo!() }
//!     # fn titled_text_text_fg(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg(&self) -> Color { todo!() }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let widgets = widgets::Themed::new(Frappe);
//!
//!     let mut canvas = Basic::new(&(7, 3));
//!     canvas.draw(&Just::Centered, widgets.title("foo"))?;
//!
//!     // ·······
//!     // ·-foo-· (highlight represented by -)
//!     // ·······
//!     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
//!     assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
//!     Ok(())
//! }
//! ```

use crate::prelude::*;
use widgets::prelude::*;

pub trait Theme {
    fn text(&self) -> Color;

    fn highlight_fg(&self) -> Color;

    fn title_fg(&self) -> Color;
    fn title_bg(&self) -> Color;

    fn button_fg(&self) -> Color;
    fn button_bg(&self) -> Color;

    fn titled_text_title_fg(&self) -> Color;
    fn titled_text_title_bg(&self) -> Color;
    fn titled_text_text_fg(&self) -> Color;
    fn titled_text_text_bg(&self) -> Color;
}

pub struct Themed<T: Theme> {
    pub theme: T
}

impl<T: Theme> Themed<T> {
    pub const fn new(theme: T) -> Self {
        Self { theme }
    }
}

widget! {
    parent: Themed<T: Theme>,
    /// A title of something (such as a page)
    ///
    /// # Style
    ///
    /// ```text
    /// ·······
    /// ·-foo-· (highlight represented by -)
    /// ·······
    /// ```
    ///
    /// # Example
    ///
    /// See the [outer module's example](self)
    name: title,
    origin: super::basic::highlighted_text,
    create: |&self, text: &'a str| (
        text,
        self.theme.title_fg(),
        self.theme.title_bg(),
    )
}

widget! {
    parent: Themed<T: Theme>,
    /// A simple button
    ///
    /// # Style
    ///
    /// ```text
    /// ·······
    /// ·-foo-· (highlight represented by -)
    /// ·······
    /// ```
    name: button,
    origin: super::basic::highlighted_text,
    create: |&self, text: &'a str| (
        text,
        self.theme.button_fg(),
        self.theme.button_bg(),
    )
}
