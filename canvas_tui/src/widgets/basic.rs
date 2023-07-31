//! A set of basic widgets with no set theming
//!
//! For basic theming support with the same style, see [`themed`]
//!
//! # Example
//!
//! ```
//! use canvas_tui::prelude::*;
//! use widgets::basic;
//!
//! struct Frappe;
//!
//! impl Frappe {
//!     pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
//!     pub const fn base() -> Color { Color::new(48, 52, 70) }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut canvas = Basic::new(&(7, 3));
//!     canvas.draw(&Just::Centered, basic::title("foo", Frappe::base(), Frappe::rosewater()))?;
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

widget! {
    #[doc(hidden)]
    // a lot of widgets extend this in some way
    // so there's a generic name for errors
    name: highlighted_text,
    args: (
        text: String [impl Into<String> as into],
        foreground: Option<Color> [impl Into<Option<Color>> as into],
        background: Option<Color> [impl Into<Option<Color>> as into],
    ),
    size: |&self, _| {
        let len = self.text.chars().count();
        let len: isize = len.try_into()
            .map_err(|_| Error::TooLarge("text length", len))?;
        Ok(Vec2::new(len + 2, 1))
    },
    draw: |self, canvas| {
        canvas.text(&Just::Centered, &self.text)
            .grow_profile(&(1, 0))
            .colored(self.foreground, self.background)
            .discard_info()
    },
}

widget! {
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
    #[inline(always)]
    name: title,
    origin: self::highlighted_text,
    create: |text: &str, foreground: impl Into<Option<Color>>, background: impl Into<Option<Color>>| ( 
        text,
        foreground,
        background,
    )
}

widget! {
    /// A toggleable button
    ///
    /// # Style
    ///
    /// ```text
    /// ·········
    /// ·-foo-✕-· (highlight represented by -)
    /// ·········
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use canvas_tui::prelude::*;
    /// use themes::catppuccin::Frappe;
    /// use widgets::basic;
    /// # fn main() -> Result<(), Error> {
    /// let mut canvas = Basic::new(&(9, 3));
    /// canvas.draw(&Just::Centered, basic::toggle("foo", false, Frappe::base(), Frappe::rosewater()))?;
    ///
    /// // ·········
    /// // ·-foo-✕-· (highlight represented by -)
    /// // ·········
    /// assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
    /// assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
    /// assert_eq!(canvas.get(&(6, 1))?.text, '✕');
    /// Ok(())
    /// # }
    /// ```
    name: toggle,
    origin: self::highlighted_text,
    create: |text: &str, activated: bool, foreground: impl Into<Option<Color>>, background: impl Into<Option<Color>>| ( 
        format!("{text} {}", if activated { '✓' } else { '✕' }),
        foreground,
        background,
    )
}
