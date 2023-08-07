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
//!     # fn rolling_selection_fg(&self) -> Color { todo!() }
//!     # fn rolling_selection_bg(&self) -> Color { todo!() }
//! }
//!
//! impl SelectableTheme for Frappe {
//!     fn button_bg_hover(&self) -> Color { Self::surface() }
//!     fn button_bg_activated(&self) -> Color { Self::surface() }
//!     fn button_fg_hover(&self) -> Color { Self::text() }
//!     fn button_fg_activated(&self) -> Color { Self::special_text() }
//!     // other colors omitted
//!     # fn highlight_fg_hover(&self) -> Color { todo!() }
//!     # fn highlight_fg_activated(&self) -> Color { todo!() }
//!     # fn titled_text_text_fg_hover(&self) -> Color { todo!() }
//!     # fn titled_text_text_fg_activated(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg_hover(&self) -> Color { todo!() }
//!     # fn titled_text_text_bg_activated(&self) -> Color { todo!() }
//!     # fn rolling_selection_fg_hover(&self) -> Color { todo!() }
//!     # fn rolling_selection_fg_activated(&self) -> Color { todo!() }
//!     # fn rolling_selection_bg_hover(&self) -> Color { todo!() }
//!     # fn rolling_selection_bg_activated(&self) -> Color { todo!() }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     // set the current selection in the constructor
//!     let widgets = widgets::Selectable::num(Frappe, 1, false);
//!
//!     let mut canvas = Basic::new(&(7, 3));
//!     // each different widget has a different selector, which gets tested against
//!     canvas
//!         .draw(&Just::CenteredOnRow(1), widgets.button(&1, "foo"))
//!         .draw(&Just::CenteredOnRow(2), widgets.button(&2, "bar"))?;
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

/// creates the necessary methods in the trait as well as a select_ method
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

/// creates a method in the struct that gets the color based on the selected item
macro_rules! private_get_color {
    ($id:ident) => {
        paste::paste! {
            fn $id(&self, selection: &V) -> Color {
                self.theme.[<select_ $id>](self.selected(selection))
            }
        }
    };
}

#[allow(clippy::module_name_repetitions)] // both theme and this are reexported in themed and often used together
pub trait SelectableTheme: Theme {
    selectable!(highlight_fg);

    selectable!(button_fg);
    selectable!(button_bg);
    selectable!(titled_text_text_fg);
    selectable!(titled_text_text_bg);
    selectable!(rolling_selection_fg);
    selectable!(rolling_selection_bg);
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

    private_get_color!(button_fg);
    private_get_color!(button_bg);
    private_get_color!(titled_text_text_fg);
    private_get_color!(titled_text_text_bg);
    private_get_color!(rolling_selection_fg);
    private_get_color!(rolling_selection_bg);
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
    origin: highlighted_text in super::basic,
    return_value: super::basic::HighlightedText,
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
    origin: highlighted_text in super::basic,
    return_value: super::basic::HighlightedText,
    create: |&self, selection: &V, text: &'a str| (
        text,
        self.button_fg(selection),
        self.button_bg(selection),
    )
}

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
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
    return_value: super::basic::HighlightedText,
    create: |&self, selection: &V, text: &'a str, activated: bool| (
        text,
        activated,
        self.button_fg(selection),
        self.button_bg(selection),
    )
}

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
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
    /// ··---Theme---··
    /// ··   Latte   ··
    /// ··  Frappe   ··
    /// ·· Macchiato ··
    /// ··   Mocha   ··
    /// ···············
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use canvas_tui::prelude::*;
    /// use themes::catppuccin::Frappe;
    /// use widgets::{Theme, SelectableTheme};
    ///
    /// fn main() -> Result<(), Error> {
    ///     // 2 is selected (but not activated)
    ///     let widgets = widgets::Selectable::num(Frappe, 2, false);
    ///
    ///     let mut canvas = Basic::new(&(15, 7));
    ///     canvas
    ///         .draw(&Just::CenteredOnRow(1), widgets.titled_text(
    ///             1.., // the selections can be defined by an iterator 
    ///             "Theme", &[
    ///                 "Latte", // so the first is 1
    ///                 "Frappe", // second is 2
    ///                 "Macchiato", // and so on
    ///                 "Mocha",
    ///             ]
    ///         ));
    ///
    ///     // ···············
    ///     // ··---Theme---··
    ///     // ··   Latte   ··
    ///     // ··  Frappe   ·· selected!
    ///     // ·· Macchiato ··
    ///     // ··   Mocha   ··
    ///     // ···············
    ///     assert_eq!(canvas.get(&(5, 1))?.text, 'T');
    ///     assert_eq!(canvas.get(&(5, 1))?.foreground, Some(Frappe.titled_text_title_fg()));
    ///     assert_eq!(canvas.get(&(5, 2))?.background, Some(Frappe.titled_text_text_bg()));
    ///     assert_eq!(canvas.get(&(5, 3))?.background, Some(Frappe.titled_text_text_bg_hover()));
    ///     assert_ne!(Frappe.titled_text_text_bg(), Frappe.titled_text_text_bg_hover());
    ///     Ok(())
    /// }
    /// ```
    name: titled_text,
    args: (
        selections: Vec<V> [impl IntoIterator<Item = V> > .into_iter().take(text.len()).collect()],
        title: String [impl ToString as to_string],
        text: Vec<String> [&[impl ToString] > .iter().map(ToString::to_string).collect()],
    ),
    optionals: (
        max_width: Option<usize>,
    ),
    size: |&self, _| {
        basic::titled_text_bounds(&self.title, &self.text, self.max_width)
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

widget! {
    parent: Selectable<V: PartialEq, T: SelectableTheme>,
    /// A rolling selection of values
    ///
    /// # Arguments
    ///
    /// - `selection` - the selection id of the widget
    /// - `text` - the text within the arrows
    /// - `width` - the max width of the widget if [`Some`], unbounded if [`None`]
    ///
    /// # Optionals
    ///
    /// - [`highlighted: Color`](RollingSelection::highlighted) (default: None)
    /// - [after `build`](WidgetSource::build),
    ///     - [`at_start: bool`](super::basic::RollingSelection::at_start) (default: false)
    ///     - [`at_end: bool`](super::basic::RollingSelection::at_start) (default: false)
    ///     - [`truncate_from_end: bool`](super::basic::RollingSelection::truncate_from_end)
    ///
    /// *Note:
    /// [`RollingSelection::truncate_from_end`](super::basic::RollingSelection::truncate_from_end)
    /// is overwritten by this extension*
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
        selection: V,
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
            self.parent.rolling_selection_fg(&self.selection)
        },
        self.highlighted.unwrap_or_else(|| self.parent.rolling_selection_bg(&self.selection))
    ).truncate_from_end(self.parent.activated(&self.selection))
}
