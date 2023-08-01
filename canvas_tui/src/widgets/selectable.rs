//! A selectable set of widgets, showing different colors whether they are hovered over or activated. 
//!
//! To use the widgets, create a new [`Selectable`] and use its methods. Themes can be created by implementing [`self::SelectableTheme`] or [`themes::BasicTheme`], but there are default themes in [`themes::common`].  
//!
//! If you don't need selectable widgets, use [`widgets::themed`].
//!
//! # Example
//!
//! ```
//! use canvas_tui::prelude::*;
//! use widgets::{Theme, SelectableTheme};
//!
//! struct Frappe;
//!
//! impl Frappe {
//!     pub const fn special_text() -> Color { Color::WHITE }
//!     pub const fn text() -> Color { Color::new(198, 208, 245) }
//!     pub const fn subtext() -> Color { Color::new(165, 173, 206) }
//!     pub const fn surface() -> Color { Color::new(65, 69, 89) }
//! }
//!
//! impl Theme for Frappe {
//!     fn button_fg(&self) -> Color { Self::subtext() }
//!     fn button_bg(&self) -> Color { Self::surface() }
//!     // other colors omitted
//!     # fn text(&self) -> Color { todo!() }
//!     # fn highlight_fg(&self) -> Color { todo!() }
//!     # fn title_fg(&self) -> Color { todo!() }
//!     # fn title_bg(&self) -> Color { todo!() }
//!     # fn titled_text_title_fg(&self) -> Color { todo!() }
//!     # fn titled_text_title_bg(&self) -> Color { todo!() }
//!     # fn titled_text_text_fg(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg(&self) -> Color { todo!() }
//! }
//!
//! impl SelectableTheme for Frappe {
//!     fn button_bg_hover(&self) -> Color { Self::surface() }
//!     fn button_bg_activated(&self) -> Color { Self::surface() }
//!     fn button_fg_hover(&self) -> Color { Self::text() }
//!     fn button_fg_activated(&self) -> Color { Self::special_text() }
//!     // other colors omitted
//!     # fn titled_text_text_fg_hover(&self) -> Color { todo!() }
//!     # fn titled_text_text_fg_activated(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg_hover(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg_activated(&self) -> Color { todo!() }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     // set the current selection in the constructor
//!     let widgets = widgets::Selectable::num(Frappe, 1, false);
//!
//!     let mut canvas = Basic::new(&(7, 3));
//!     // each different widget has a different selector, which gets tested against
//!     canvas
//!         .draw(&Just::CenteredOnRow(1), widgets.button(1, "foo"))
//!         .draw(&Just::CenteredOnRow(2), widgets.button(2, "bar"))?;
//!
//!     // ·······
//!     // ·-foo-· (highlight represented by -)
//!     // ·-bar-·
//!     // ·······
//!     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe.button_fg_hover()));
//!     assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe.button_bg()));
//!     assert_eq!(canvas.get(&(1, 2))?.foreground, Some(Frappe.button_fg()));
//!     assert_eq!(canvas.get(&(1, 2))?.background, Some(Frappe.button_bg()));
//!     Ok(())
//! }
//! ```

use crate::prelude::*;
use widgets::prelude::*;
use widgets::themed::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Deselected,
    Selected,
    Activated,
}

// creates the necessary methods in the trait as well as a select_ method
macro_rules! selectable {
    ($id:ident) => {
        paste::paste! {
            fn [<$id _hover>](&self) -> Color;
            fn [<$id _activated>](&self) -> Color;
            fn [<select_ $id>](&self, selected: Selection) -> Color {
                match selected {
                    Selection::Deselected => self.$id(),
                    Selection::Selected => self.[<$id _hover>](),
                    Selection::Activated => self.[<$id _activated>](),
                }
            }
        }
    };
}

// creates a method in the struct that gets the color based on the selected item
macro_rules! get_color {
    ($id:ident) => {
        paste::paste! {
            fn $id(&self, selection: &V) -> Color {
                self.theme.[<select_ $id>](self.selected(selection))
            }
        }
    };
}

// both theme and this are reexported in themed and often used together
#[allow(clippy::module_name_repetitions)] 
pub trait SelectableTheme: Theme {
    selectable!(button_fg);
    selectable!(button_bg);
    selectable!(titled_text_text_fg);
    selectable!(titled_text_text_bg);
}

pub struct Selectable<V: PartialEq, T: SelectableTheme> {
    pub theme: T,
    pub selection: V,
    pub activated: bool,
}

impl<T: SelectableTheme> Selectable<usize, T> {
    pub const fn num(theme: T, val: usize, activated: bool) -> Self {
        Self { theme, selection: val, activated }
    }
}

impl<V: PartialEq, T: SelectableTheme> Selectable<V, T> {
    pub const fn new(theme: T, selection: V, activated: bool) -> Self {
        Self { theme, selection, activated }
    }

    pub fn selected(&self, val: &V) -> Selection {
        match (self.selection.eq(val), self.activated) {
            (false, _) => Selection::Deselected,
            (true, false) => Selection::Selected,
            (true, true) => Selection::Activated,
        }
    }

    pub fn activated(&self, val: &V) -> bool {
        self.selected(val) == Selection::Activated
    }

    get_color!(button_fg);
    get_color!(button_bg);
    get_color!(titled_text_text_fg);
    get_color!(titled_text_text_bg);
}

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
    /// A title of something such as a page
    ///
    /// # Style
    ///
    /// ```text
    /// ·······
    /// ·-foo-· (highlight represented by -)
    /// ·······
    /// ```
    name: title,
    origin: super::basic::highlighted_text,
    create: |&self, text: &'a str| (
        text,
        self.theme.title_fg(),
        self.theme.title_bg(),
    )
}

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
    /// A selectable button
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
    name: button,
    origin: super::basic::highlighted_text,
    create: |&self, selection: V, text: &'a str| (
        text,
        self.button_fg(&selection),
        self.button_bg(&selection),
    )
}

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
    /// A `title` with rows of `text` underneath
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
    name: titled_text,
    args: (
        selections: Vec<V> [impl IntoIterator<Item = V> > .into_iter().take(text.len()).collect()],
        title: String [&str as to_string],
        text: Vec<String> [&[impl ToString] > .iter().map(ToString::to_string).collect()],
    ),
    optionals: (
        max_width: Option<usize>,
    ),
    size: |&self, _| {
        let mut text_width = self.text.iter()
            .chain(std::iter::once(&self.title))
            .map(|string| string.chars().count())
            .max()
            .expect("the iterator has at least one element: the title");
        if let Some(max_width) = self.max_width {
            text_width = text_width.min(max_width - 2);
        }
        let text_width: isize = text_width.try_into()
            .map_err(|_| Error::TooLarge("text length", text_width))?;

        let lines = self.text.len();
        let lines: isize = lines.try_into()
            .map_err(|_| Error::TooLarge("lines of titled text", lines))?;

        Ok(Vec2::new(text_width + 2, lines + 1))
    },
    draw: |self, canvas| {
        let theme = &self.parent.theme;
        let width = canvas.width();
        // give the text some padding on the sides
        let max_width = self.max_width.map(|max| max - 2);

        // empty canvas
        canvas.fill(' ')?;

        // title
        let title = truncate(&self.title, max_width, false);
        canvas.text(&(Just::CenteredOnRow(0)), &title)
            .expand_profile(width, None, GrowFrom::CenterPreferRight)
            .colored(
                theme.titled_text_title_fg(), 
                theme.titled_text_title_bg()
            )?;

        // text
        for ((text, line), selection) in self.text.iter().zip(1..).zip(self.selections) {
            let text = truncate(text, max_width, self.parent.activated(&selection));
            canvas.text(&Just::CenteredOnRow(line), &text)
                .expand_profile(width, None, GrowFrom::Center)
                .colored(
                    self.parent.titled_text_text_fg(&selection),
                    self.parent.titled_text_text_bg(&selection),
                )?;
        }

        Ok(())
    },
}

/// Truncate `string` to `max_width` from the start if `!activated` or from the end if `activated`
fn truncate(string: &str, max_width: Option<usize>, activated: bool) -> String {
    if let Some(max_width) = max_width {
        if string.len() > max_width {
            return truncate_unchecked(string, max_width, activated);
        }
    }
    string.to_string()
}

/// Truncate `string` to `max_width` from the start if `!activated` or from the end if `activated`
///
/// # Panics
///
/// - If the `string`'s length is smaller than `max_width`
fn truncate_unchecked(string: &str, max_width: usize, activated: bool) -> String {
    if activated {
        string[(string.len() - max_width)..].to_string()
    } else {
        string[..max_width].to_string()
    }
}