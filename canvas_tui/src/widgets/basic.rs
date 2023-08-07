//! A set of basic widgets with no set theming
//!
//! For basic theming support with the same style, see [`widgets::themed`]
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

use super::{truncate, length_of};

widget! {
    /// A generic thing of highlighted text
    ///
    /// See [`title`], [`button`], and [`toggle`] for more specific implementations
    ///
    /// # Style
    ///
    /// ```text
    /// ·······
    /// ·-foo-· (highlight represented by -)
    /// ·······
    /// ```
    name: highlighted_text,
    args: (
        text: String [impl ToString as to_string],
        foreground: Option<Color> [impl Into<Option<Color>> as into],
        background: Option<Color> [impl Into<Option<Color>> as into],
    ),
    optionals: (
        width: Option<usize>,
        truncate_from_end: Option<bool>,
    ),
    size: |&self, _| {
        Ok(Vec2::new(super::width_or_length(self.width, &self.text, 2)?, 1))
    },
    draw: |self, canvas| {
        canvas
            .fill(' ').colored(self.foreground, self.background)
            .text(&Just::Centered, &truncate(&self.text, self.width, self.truncate_from_end.unwrap_or_default()))
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
    origin: highlighted_text in self,
    return_value: HighlightedText,
    create: |text: &str, foreground: impl Into<Option<Color>>, background: impl Into<Option<Color>>| ( 
        text,
        foreground,
        background,
    )
}

widget! {
    /// A simple button
    ///
    /// # Style
    ///
    /// ```text
    /// ·······
    /// ·-foo-· (highlight represented by -)
    /// ·······
    /// ```
    #[inline(always)]
    name: button,
    origin: highlighted_text in self,
    return_value: HighlightedText,
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
    /// # Panics
    ///
    /// - If `width` is less than 6
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
    args: (
        text: String [impl ToString as to_string],
        activated: bool,
        foreground: Option<Color> [impl Into<Option<Color>> as into],
        background: Option<Color> [impl Into<Option<Color>> as into],
    ),
    optionals: (
        width: Option<usize>,
        truncate_from_end: Option<bool>,
    ),
    size: |&self, _| {
        if let Some(width) = self.width { assert!(width >= 6); }
        Ok(Vec2::new(super::width_or_length(self.width, &self.text, 6)?, 1))
    },
    draw: |self, canvas| {
        if let Some(width) = self.width { assert!(width >= 6); }

        canvas.fill(' ').colored(self.foreground, self.background)?;

        // if the width is constrained and the text is too big
        if self.width.is_some() && length_of(&self.text)? > canvas.width() - 3 * 2 {
            let truncate_from_end = self.truncate_from_end.unwrap_or_default();
            let text_width = (canvas.width() - 3 - 1).try_into().expect("asserted");

            // truncate the text and draw it as far right as it can go
            let text = &truncate(&self.text, Some(text_width), truncate_from_end);
            canvas.text(&Just::OffCenterRightBy(3), text)?; 
        } else {
            // otherwise just draw it in the center
            canvas.text(&Just::Centered, &self.text)?;
        }

        canvas.text(&Just::CenterRight, if self.activated { "✓" } else { "✕" })
            .discard_info()
    },
}

widget! {
    /// A `title` with rows of `text` underneath
    ///
    /// # Optionals
    ///
    /// - [`max_width: usize`](TitledText::max_width)
    ///
    /// # Style
    ///
    /// The width adjusts to the widest line of text or `max_width` if it is hit
    ///
    /// ```text
    /// ···············
    /// ··###Theme###··
    /// ··---Latte---··
    /// ··--Frappe---··
    /// ··-Macchiato-··
    /// ··---Mocha---··
    /// ···············
    /// ```
    #[allow(clippy::similar_names)] // sorry
    name: titled_text,
    args: (
        title: String [impl ToString as to_string],
        text: Vec<String> [&[impl ToString] > .iter().map(ToString::to_string).collect()],
        title_fg: Option<Color> [impl Into<Option<Color>> as into],
        title_bg: Option<Color> [impl Into<Option<Color>> as into],
        text_fg:  Option<Color> [impl Into<Option<Color>> as into],
        text_bg:  Option<Color> [impl Into<Option<Color>> as into],
    ),
    optionals: (
        max_width: Option<usize>,
    ),
    size: |&self, _| {
        titled_text_bounds(&self.title, &self.text, self.max_width)
    },
    draw: |self, canvas| {
        let width = canvas.width();
        // give the text some padding on the sides
        let max_width = self.max_width.map(|max| max - 2);

        // empty canvas
        canvas.fill(' ')?;

        // title
        let title = truncate(&self.title, max_width, false);
        canvas.text(&(Just::CenteredOnRow(0)), &title)
            .expand_profile(width, None, GrowFrom::CenterPreferRight)
            .colored(self.title_fg, self.title_bg)?;

        // text
        for (text, line) in self.text.iter().zip(1..) {
            let text = truncate(text, max_width, false);
            canvas.text(&Just::CenteredOnRow(line), &text)
                .expand_profile(width, None, GrowFrom::Center)
                .colored(self.text_fg, self.text_bg)?;
        }

        Ok(())
    },
}

pub(super) fn titled_text_bounds(title: &String, text: &Vec<String>, max_width: Option<usize>) -> Result<Vec2, Error> {
    let mut text_width = text.iter()
        .chain(std::iter::once(title))
        .map(|string| string.chars().count())
        .max()
        .expect("the iterator has at least one element: the title");
    if let Some(max_width) = max_width {
        text_width = text_width.min(max_width - 2);
    }
    let text_width: isize = text_width.try_into()
        .map_err(|_| Error::TooLarge("text length", text_width))?;

    let lines = text.len();
    let lines: isize = lines.try_into()
        .map_err(|_| Error::TooLarge("lines of titled text", lines))?;

    Ok(Vec2::new(text_width + 2, lines + 1))
}

widget! {
    /// A rolling selection of values
    ///
    /// # Optionals
    ///
    /// - [`at_start: bool`](RollingSelection::at_start) (default: false)
    /// - [`at_end: bool`](RollingSelection::at_end) (default: false)
    /// - [`truncate_from_end: bool`](RollingSelection::truncate_from_end) (default: false)
    ///
    /// # Style
    ///
    /// ```text
    /// ···········
    /// · ← foo → ·
    /// ···········
    /// ```
    name: rolling_selection,
    args: (
        text: String [impl ToString as to_string],
        // the width is used much more often than an optional would require
        width: Option<usize> [impl Into<Option<usize>> as into],
        foreground: Option<Color> [impl Into<Option<Color>> as into],
        background: Option<Color> [impl Into<Option<Color>> as into],
    ),
    optionals: (
        at_start: Option<bool>,
        at_end: Option<bool>,
        truncate_from_end: Option<bool>,
    ),
    size: |&self, _| {
        let width = self.width.unwrap_or_else(|| self.text.chars().count() + 6);
        let width: isize = width.try_into()
            .map_err(|_| Error::TooLarge("text length", width))?;
        Ok(Vec2::new(width, 1))
    },
    draw: |self, canvas| {
        assert!(!self.width.is_some_and(|width| width < 6), "rolling selection width must be at least 6");

        let text = truncate(&self.text, self.width.map(|val| val - 6), self.truncate_from_end.unwrap_or_default());
        canvas
            .fill(' ').colored(self.foreground, self.background)
            .text(&Just::Centered, &text)?;

        if !self.at_start.unwrap_or_default() {
            canvas.text(&Just::CenterLeft, "←")?;
        }

        if !self.at_end.unwrap_or_default() {
            canvas.text(&Just::CenterRight, "→")?;
        }

        Ok(())
    },
}
