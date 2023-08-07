//! A set of widgets with the same style as [`widgets::basic`], but with colors determined by the theme. 
//!
//! To use the widgets, create a new [`Themed`] and use its methods. Themes can be created by implementing [`self::Theme`] or [`themes::BasicTheme`], but there are default themes in [`themes::common`]. 
//!
//! For widget hover / activation support, use [`widgets::selectable`].
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
//!     # fn rolling_selection_fg(&self) -> Color { todo!() }
//!     # fn rolling_selection_bg(&self) -> Color { todo!() }
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

    fn rolling_selection_fg(&self) -> Color;
    fn rolling_selection_bg(&self) -> Color;
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
    origin: highlighted_text in super::basic,
    return_value: super::basic::HighlightedText,
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
    origin: highlighted_text in super::basic,
    return_value: super::basic::HighlightedText,
    create: |&self, text: &'a str| (
        text,
        self.theme.button_fg(),
        self.theme.button_bg(),
    )
}

widget! {
    parent: Themed<T: Theme>,
    /// A toggleable button
    ///
    /// # Style
    ///
    /// ```text
    /// ·········
    /// ·-foo-✕-· (highlight represented by -)
    /// ·········
    /// ```
    name: toggle,
    origin: toggle in super::basic,
    return_value: super::basic::Toggle,
    create: |&self, text: &'a str, activated: bool| ( 
        text,
        activated,
        self.theme.button_fg(),
        self.theme.button_bg(),
    )
}

widget! {
    parent: Themed<T: Theme>,
    /// A `title` with rows of `text` underneath
    ///
    /// # Optionals
    ///
    /// - [`max_width: usize`](super::basic::TitledText::max_width)
    ///
    /// # Style
    ///
    /// The width adjusts to the widest line of text or `max_width` if it is hit
    ///
    /// ```text
    /// ···············
    /// ··---Theme---··
    /// ··   Latte   ··
    /// ··  Frappe   ··
    /// ·· Macchiato ··
    /// ··   Mocha   ··
    /// ···············
    /// ```
    name: titled_text,
    origin: titled_text in super::basic,
    create: |&self, title: &'a str, text: &[impl ToString]| (
        title,
        text,
        self.theme.titled_text_title_fg(),
        self.theme.titled_text_title_bg(),
        self.theme.titled_text_text_fg(),
        self.theme.titled_text_text_bg(),
    )
}

widget! {
    parent: Themed<T: Theme>,
    /// A rolling selection of values
    ///
    /// # Arguments
    ///
    /// - `text` - the text within the arrows
    /// - `width` - the max width of the widget if [`Some`], unbounded if [`None`]
    ///
    /// # Optionals
    ///
    /// - [`highlighted: Color`](RollingSelection::highlighted) (default: None)
    /// - [after `build`](WidgetSource::build),
    ///     - [`at_start: bool`](super::basic::RollingSelection::at_start) (default: false)
    ///     - [`at_end: bool`](super::basic::RollingSelection::at_end) (default: false)
    ///     - [`truncate_from_end: bool`](super::basic::RollingSelection::truncate_from_end) (default: false)
    ///
    /// # Style
    ///
    /// ```text
    /// ···········
    /// · ← foo → ·
    /// ···········
    /// ```
    name: rolling_selection,
    origin: rolling_selection in super::basic,
    args: (
        text: String [&str as to_string],
        width: Option<usize> [impl Into<Option<usize>> as into],
    ),
    optionals: (
        highlighted: Option<Color>,
    ),
    build: |self| (
        self.text,
        self.width,
        if self.highlighted.is_some() {
            self.parent.theme.highlight_fg()
        } else {
            self.parent.theme.rolling_selection_fg()
        },
        self.highlighted.unwrap_or_else(|| self.parent.theme.rolling_selection_bg()),
    )
}
