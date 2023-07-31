use crate::{prelude::*, widgets::{Theme, SelectableTheme}};

pub mod common;
pub use common::*;

/// A basic theme
///
/// This automatically implements [`widgets::Theme`] and [`widgets::SelectableTheme`], so it's a simpler way to
/// create a theme. However, the two traits are also available to be implemented for full control
///
/// # Background order
///
/// - Surface (above)
/// - Base (basic background)
/// - Mantle (below)
/// - Crust
pub trait BasicTheme {
    /// The general background
    #[must_use] fn base() -> Color;
    #[must_use] fn mantle() -> Color;
    #[must_use] fn crust() -> Color;
    #[must_use] fn surface() -> Color;
    #[must_use] fn surface1() -> Color;
    #[must_use] fn surface2() -> Color;

    /// The generic text color
    #[must_use] fn text() -> Color; 
    #[must_use] fn subtext() -> Color; 
    #[must_use] fn special_text() -> Color; 

    /// A color to contrast against highlight colors
    #[must_use] fn highlight_fg() -> Color { Self::base() }

    #[must_use] fn button_fg() -> Color { Self::subtext() }
    #[must_use] fn button_bg() -> Color { Self::surface() }

    /// A generic background for text boxes
    #[must_use] fn text_bg() -> Color { Self::surface() }
    #[must_use] fn hover_fg() -> Color { Self::special_text() }
    #[must_use] fn hover_bg() -> Color { Self::surface1() }

    #[must_use] fn success() -> Color;
    #[must_use] fn warning() -> Color;
    #[must_use] fn error() -> Color;

    #[must_use] fn link() -> Color;

    #[must_use] 
    fn highlighted(self, highlight: Color) -> WithHighlight<Self> where Self: Sized {
        WithHighlight { theme: self, highlight }
    }
}

impl<T: BasicTheme> Theme for T {
    fn text(&self) -> Color { Self::text() }

    fn highlight_fg(&self) -> Color { Self::highlight_fg() }

    fn title_fg(&self) -> Color { Self::text() }
    fn title_bg(&self) -> Color { Self::surface() }

    fn button_bg(&self) -> Color { Self::button_bg() }
    fn button_fg(&self) -> Color { Self::button_fg() }

    fn titled_text_title_fg(&self) -> Color { Self::text() }
    fn titled_text_title_bg(&self) -> Color { Self::surface2() }

    fn titled_text_text_fg(&self) -> Color { Self::text() }
    fn titled_text_text_bg(&self) -> Color { Self::text_bg() }
}

impl<T: BasicTheme> SelectableTheme for T {
    fn button_fg_hover(&self) -> Color { Self::hover_fg() }
    fn button_fg_activated(&self) -> Color { self.button_fg_hover() }

    fn button_bg_hover(&self) -> Color { Self::button_bg() }
    fn button_bg_activated(&self) -> Color { self.button_bg_hover() }

    fn titled_text_text_fg_hover(&self) -> Color { Self::text() }
    fn titled_text_text_fg_activated(&self) -> Color { Self::hover_fg() }

    fn titled_text_text_bg_hover(&self) -> Color { Self::hover_bg() }
    fn titled_text_text_bg_activated(&self) -> Color { self.titled_text_text_bg_hover() }
}

pub struct WithHighlight<T: Theme + SelectableTheme> {
    theme: T,
    highlight: Color,
}

impl<T: Theme + SelectableTheme> Theme for WithHighlight<T> {
    fn text(&self) -> Color { self.theme.text() }

    fn highlight_fg(&self) -> Color { self.theme.highlight_fg() }

    fn title_fg(&self) -> Color { self.highlight_fg() }
    fn title_bg(&self) -> Color { self.highlight }

    fn button_bg(&self) -> Color { self.theme.button_bg() }
    fn button_fg(&self) -> Color { self.theme.button_fg() }

    fn titled_text_title_fg(&self) -> Color { self.highlight_fg() }
    fn titled_text_title_bg(&self) -> Color { self.highlight }

    fn titled_text_text_fg(&self) -> Color { self.theme.titled_text_text_fg() }
    fn titled_text_text_bg(&self) -> Color { self.theme.titled_text_text_bg() }
}

impl<T: Theme + SelectableTheme> SelectableTheme for WithHighlight<T> {
    fn button_fg_hover(&self) -> Color { self.theme.button_fg_hover() }
    fn button_fg_activated(&self) -> Color { self.theme.button_fg_activated() }

    fn button_bg_hover(&self) -> Color { self.theme.button_bg_hover() }
    fn button_bg_activated(&self) -> Color { self.theme.button_bg_activated() }

    fn titled_text_text_fg_hover(&self) -> Color { self.theme.titled_text_text_fg_hover() }
    fn titled_text_text_fg_activated(&self) -> Color { self.highlight }

    fn titled_text_text_bg_hover(&self) -> Color { self.theme.titled_text_text_bg_hover() }
    fn titled_text_text_bg_activated(&self) -> Color { self.theme.titled_text_text_bg_activated() }
}

