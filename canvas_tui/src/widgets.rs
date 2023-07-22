use crate::{prelude::*, num::Size};

/// Constructs a [`Widget`] using the specified parameters
///
/// # Examples
///
/// ## Creating a new widget
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::widget;
///
/// widget! {
///     // you can also specify a struct to be a parent, in which the function becomes a method
///     // within the parent, and a reference to the parent is stored within the widget as `parent`
///     // parent: Parent,
///     name: title,
///     args: ( 
///         text: String [&str as to_string], // converters can be used like this
///         foreground: Color,
///         background: Color,
///     ),
///     size: |&self, _| { 
///         let len = self.text.chars().count();
///         let len: isize = len.try_into()
///             .map_err(|_| Error::TooLarge("text length", len))?;
///         Ok(Vec2::new(len + 2, 1))
///     },
///     draw: |self, canvas| { 
///         canvas.text(&Just::Centered, &self.text)
///             .grow_profile(&(1, 0))
///             .colored(self.foreground, self.background)
///             .discard_info()
///     },
/// }
///
/// fn main() -> Result<(), Error> {
///     let mut canvas = Basic::new(&(7, 3));
///     canvas.draw(&Just::Centered, title("foo", Color::BLACK, Color::WHITE))?;
///
///     // ·······
///     // ·-foo-· highlight represented by -
///     // ·······
///     assert_eq!(canvas.get(&(0, 1))?.foreground, None);
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Color::BLACK));
///     assert_eq!(canvas.get(&(2, 1))?.foreground, Some(Color::BLACK));
///     Ok(())
/// }
/// ```
///
/// ## Changing an already existing widget
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::widget;
///
/// widget! {
///     name: title,
///     origin: widgets::basic::title,
///     create: |text: &str| (
///         text,
///         Color::WHITE,
///         Color::BLACK,
///     )
/// }
///
/// fn main() -> Result<(), Error> {
///     let mut canvas = Basic::new(&(7, 3));
///     canvas.draw(&Just::Centered, title("foo"))?;
///
///     // ·······
///     // ·-foo-· highlight represented by -
///     // ·······
///     assert_eq!(canvas.get(&(0, 1))?.foreground, None);
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Color::WHITE));
///     assert_eq!(canvas.get(&(2, 1))?.foreground, Some(Color::WHITE));
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! widget {
    (
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty as $converter:ty])?),* $(,)? ), 
        // returns the size of the widget
        size: |&$sizeself:ident, $canvas_size:tt| $size:expr,
        // draws the widget onto `canvas`
        draw: |$drawself:ident, $canvas:ident| $draw:expr $(,)?
    ) => {
        paste::paste! {
            #[doc(hidden)]
            pub struct [<$name:camel>] { $($arg: $type),* }

            use $crate::num::Size;
            impl Widget for [<$name:camel>] {
                fn size(&$sizeself, $canvas_size: &impl Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            #[must_use]
            #[allow(clippy::redundant_field_names)]
            $(#[$($attrs)*])*
            pub fn [<$name:lower>]($($arg: $crate::rightmost!(($type) $(($from))?)),*) -> impl Widget + 'static {
                [<$name:camel>] { $($arg: $arg$(.$converter())?),* }
            }
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(<$($($lt:lifetime),+ , )? $($generic_name:ident: $generic_value:ty),*>)?,
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty as $converter:ty])?),* $(,)? ),
        // returns the size of the widget
        size: |&$sizeself:ident, $canvas_size:tt| $size:expr,
        // draws the widget onto `canvas`
        draw: |$drawself:ident, $canvas:ident| $draw:expr $(,)?
    ) => {
        paste::paste! {
            #[doc(hidden)]
            pub struct [<$name:camel>]<'a $(, $($lt),+)? $(, $($generic_name: $generic_value),*)?> {
                parent: &'a $parent$(<$($generic_name),*>)?, 
                $($arg: $type),*
            }

            use $crate::num::Size;
            impl<'a $(, $($lt),+)? $(, $($generic_name: $generic_value),*)?> Widget for [<$name:camel>]<'a $(, $($lt),+)? $(, $($generic_name),*)?> {
                fn size(&$sizeself, $canvas_size: &impl Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            impl$(<$($($lt),+ ,)? $($generic_name: $generic_value),*>)? $parent$(<$($($lt),+ ,)? $($generic_name),*>)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>](&self, $($arg: $crate::rightmost!(($type) $(($from))?)),*) -> impl Widget + '_ {
                    [<$name:camel>] { parent: self, $($arg: $arg$(.$converter())?),* }
                }
            }

        }       
    };
    (
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the created and original widget
        name: $name:ident,
        // the path of the original widget's function (can't be a method currently)
        origin: $origin:path,
        // the new widget's signature + the arguments passed into the original widget
        // note: all the arguments have to have types
        create: |$($param:ident: $type:ty),*| ($($arg:expr),* $(,)?) $(,)?
    ) => {
        paste::paste! {
            #[must_use]
            #[allow(clippy::redundant_field_names)]
            $(#[$($attrs)*])*
            pub fn [<$name:lower>]($($param: $type),*) -> impl Widget + '_ {
                $origin($($arg),*)
            }
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(<$($($lt:lifetime),+ , )? $($generic_name:ident: $generic_value:ty),*>)?,
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the created and original widget
        name: $name:ident,
        // the path of the original widget's function (can't be a method currently)
        origin: $origin:path,
        // the new widget's signature + the arguments passed into the original widget
        // the first argument is &self, referring to the parent
        // note: all the arguments have to have types
        create: |&$create_self:ident, $($param:ident: $type:ty),*| ($($arg:expr),* $(,)?) $(,)? 
    ) => {
        paste::paste! {
            impl$(<$($($lt),+ ,)? $($generic_name: $generic_value),*>)? $parent$(<$($($lt),+ ,)? $($generic_name),*>)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>]<'a>(&'a $create_self, $($param: $type),*) -> impl Widget + '_ {
                    $origin($($arg),*)
                }
            }
        }
    };
}

// just used in the above macro
#[doc(hidden)]
#[macro_export]
macro_rules! rightmost {
    // (...)
    (($($single:tt)*)) => {
        $($single)*
    };
    // (...) (...)
    (($($left:tt)*) ($($right:tt)*)) => {
        $($right)*
    }
}

pub use widget;

/// Some things that are useful for developing widgets
pub mod prelude {
    pub use super::widget;
    pub use crate::num::*;
    pub use super::*;
}

/// Some common drawable object that's too complex to be included in [`Canvas`]
pub trait Widget {
    /// Gets the size of the widget to be drawn while potentially using the `canvas_size`
    ///
    /// # Errors
    ///
    /// - If there is some error into getting the size, such as when some text's length is too long
    /// to fit into an [`isize`]
    fn size(&self, canvas_size: &impl Size) -> Result<Vec2, Error>;
    /// Draws the widget onto the canvas
    ///
    /// The input `canvas` must be the same size as given by [`Self::size`]
    ///
    /// # Errors
    ///
    /// - If the drawing of the widget has an error
    fn draw<C: Canvas>(self, canvas: &mut C) -> Result<(), Error>;
    /// The name of the widget to be used in error messages
    fn name() -> &'static str;
}

/// A set of basic widgets with no set theming
///
/// For basic theming support with the same style, see [`themed`]
///
/// # Example
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::basic;
///
/// struct Frappe;
///
/// impl Frappe {
///     pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
///     pub const fn base() -> Color { Color::new(48, 52, 70) }
/// }
///
/// fn main() -> Result<(), Error> {
///     let mut canvas = Basic::new(&(7, 3));
///     canvas.draw(&Just::Centered, basic::title("foo", Frappe::base(), Frappe::rosewater()))?;
///
///     // ·······
///     // ·-foo-· (highlight represented by -)
///     // ·······
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
///     assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
///     Ok(())
/// }
/// ```
pub mod basic {
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

//     widget! {
//         name: titled_text,
//         args: (
//             title: String [impl Into<String> as into],
//             title_fg: Option<Color> [impl Into<Option<Color>> as into],
//             title_bg: Option<Color> [impl Into<Option<Color>> as into],
//             text_fg: Option<Color> [impl Into<Option<Color>> as into],
//             text_bg: Option<Color> [impl Into<Option<Color>> as into],
//         )
//     }
}

/// A set of widgets with the same style as [`basic`], but with colors determined by the theme
///
/// # Example
///
/// ```
/// use canvas_tui::prelude::*;
///
/// struct Frappe;
///
/// impl Frappe {
///     pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
///     pub const fn base() -> Color { Color::new(48, 52, 70) }
/// }
///
/// impl widgets::Theme for Frappe {
///     fn title_fg(&self) -> Color { Self::base() }
///     fn title_bg(&self) -> Color { Self::rosewater() }
///
/// # fn button_bg(&self) -> Color { Color::BLACK }
/// # fn button_fg_normal(&self) -> Color { Color::BLACK }
/// # fn button_fg_hover(&self) -> Color { Color::BLACK }
/// }
///
/// fn main() -> Result<(), Error> {
///     let widgets = widgets::Themed::new(Frappe);
///
///     let mut canvas = Basic::new(&(7, 3));
///     canvas.draw(&Just::Centered, widgets.title("foo"))?;
///
///     // ·······
///     // ·-foo-· (highlight represented by -)
///     // ·······
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe::base()));
///     assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe::rosewater()));
///     Ok(())
/// }
/// ```
pub mod themed {
    use crate::prelude::*;
    use widgets::prelude::*;

    pub trait Theme {
        fn text(&self) -> Color;

        fn title_fg(&self) -> Color;
        fn title_bg(&self) -> Color;

        fn button_bg(&self) -> Color;
        fn button_fg(&self) -> Color;

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
}

/// A selectable set of widgets
///
/// # Example
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::Theme;
///
/// struct Frappe;
///
/// impl Frappe {
/// #   pub const fn rosewater() -> Color { Color::new(242, 213, 207) }
/// #   pub const fn base() -> Color { Color::new(48, 52, 70) }
///     pub const fn text() -> Color { Color::new(198, 208, 245) }
///     pub const fn subtext0() -> Color { Color::new(165, 173, 206) }
///     pub const fn surface0() -> Color { Color::new(65, 69, 89) }
/// }
///
/// impl widgets::Theme for Frappe {
/// # fn title_fg(&self) -> Color { Self::base() }
/// # fn title_bg(&self) -> Color { Self::rosewater() }
///
///     fn button_bg(&self) -> Color { Self::surface0() }
///     fn button_fg_normal(&self) -> Color { Self::subtext0() }
///     fn button_fg_hover(&self) -> Color { Self::text() }
/// }
///
/// fn main() -> Result<(), Error> {
///     // set the current selection in the constructor
///     let widgets = widgets::Selectable::num(Frappe, 1);
///
///     let mut canvas = Basic::new(&(7, 3));
///     // each different widget has a different selector, which gets tested against
///     canvas
///         .draw(&Just::CenteredOnRow(1), widgets.button(1, "foo"))
///         .draw(&Just::CenteredOnRow(2), widgets.button(2, "bar"))?;
///
///     // ·······
///     // ·-foo-· (highlight represented by -)
///     // ·-bar-·
///     // ·······
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Frappe.button_fg_hover()));
///     assert_eq!(canvas.get(&(1, 1))?.background, Some(Frappe.button_bg()));
///     assert_eq!(canvas.get(&(1, 2))?.foreground, Some(Frappe.button_fg_normal()));
///     assert_eq!(canvas.get(&(1, 2))?.background, Some(Frappe.button_bg()));
///     Ok(())
/// }
/// ```
pub mod selectable {
    use crate::prelude::*;
    use crate::shapes::DrawnShape;
    use widgets::prelude::*;
    use widgets::themed::Theme;

    pub trait SelectableTheme: Theme {
        fn button_fg_hover(&self) -> Color;
        fn titled_text_text_fg_hover(&self) -> Color;

        fn select_button_fg(&self, selected: bool) -> Color 
            { if selected { self.button_fg_hover() } else { self.button_fg() } }
        fn select_titled_text_text_fg_hover(&self, selected: bool) -> Color
            { if selected { self.titled_text_text_fg_hover() } else { self.titled_text_text_fg() } }
    }

    pub struct Selectable<V: PartialEq, T: SelectableTheme> {
        pub theme: T,
        pub selection: V,
    }

    impl<T: SelectableTheme> Selectable<usize, T> {
        pub const fn num(theme: T, val: usize) -> Self {
            Self { theme, selection: val }
        }
    }

    impl<V: PartialEq, T: SelectableTheme> Selectable<V, T> {
        pub const fn new(theme: T, selection: V) -> Self {
            Self { theme, selection }
        }
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
            self.theme.select_button_fg(selection == self.selection),
            self.theme.button_bg()
        )
    }

    pub struct TitledText<'p, V: PartialEq, T: SelectableTheme> {
        parent: &'p Selectable<V, T>,
        title: String,
        text: Vec<String>,
        selection: V,
    }

    impl<'p, V: PartialEq, T: SelectableTheme> Widget for TitledText<'p, V, T> {
        fn size(&self, _: &impl Size) -> Result<Vec2, Error> {
            let text_width = self.text.iter()
                .chain(std::iter::once(&self.title))
                .map(|string| string.chars().count())
                .max()
                .expect("the iterator has at least one element: the title");
            let text_width: isize = text_width.try_into()
                .map_err(|_| Error::TooLarge("text length", text_width))?;
            let lines = self.text.len();
            let lines: isize = lines.try_into()
                .map_err(|_| Error::TooLarge("lines of titled text", lines))?;
            Ok(Vec2::new(text_width + 2, lines + 1))
        }

        fn draw<C: Canvas>(self, canvas: &mut C) -> Result<(), Error> {
            let theme = &self.parent.theme;
            let width = canvas.width();
            canvas
                .text(&(Just::CenteredOnRow(0)), &self.title)
                .highlight_box(&(0, 0), &(width, 1), theme.titled_text_title_fg(), theme.titled_text_title_bg())?;
            Ok(())
        }

        fn name() -> &'static str { "titled text" }
    }
}

pub use themed::*;
pub use selectable::*;
