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
///     // the name of the widget and the function that creates it
///     name: title,
///     // the arguments for the creation function
///     args: ( 
///         text: String [&str as to_string], // converters can be used like this
///         foreground: Color,
///         background: Color,
///     ),
///     // returns the size of the widget
///     size: |&self, _| { 
///         let len = self.text.len();
///         let len: isize = len.try_into()
///             .map_err(|_| Error::TooLarge("text length", len))?;
///         Ok(Vec2::new(len + 2, 1))
///     },
///     // draws the widget onto `canvas`
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
///     origin: widgets::basic,
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
            pub fn [<$name:lower>]($($arg: $crate::rightmost!(($type) $(($from))?)),*) -> impl Widget + '_ {
                [<$name:camel>] { $($arg: $arg$(.$converter())?),* }
            }
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(<$($generic_name:tt: $generic_value:tt),*>)?,
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
            pub struct [<$name:camel>]<'a $(, $($generic_name: $generic_value),*)?> {
                parent: &'a $parent$(<$($generic_name),*>)?, 
                $($arg: $type),*
            }

            use $crate::num::Size;
            impl<'a $(, $($generic_name: $generic_value),*)?> Widget for [<$name:camel>]<'a $(, $($generic_name),*)?> {
                fn size(&$sizeself, $canvas_size: &impl Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            impl$(<$($generic_name: $generic_value),*>)? $parent$(<$($generic_name),*>)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                pub fn [<$name:lower>](&self, $($arg: $crate::rightmost!(($type) $(($from))?)),*) -> impl Widget + '_ {
                    [<$name:camel>] { parent: self, $($arg: $arg$(.$converter())?),* }
                }
            }

        }       
    };
    (
        // the name of the created and original widget
        name: $name:ident,
        // the path of the original widget's module (can't be a struct currently)
        origin: $origin:path,
        // the new widget's signature + the arguments passed into the original widget
        create: |$($param:ident: $type:ty),*| ($($arg:expr),* $(,)?) $(,)?
    ) => {
        paste::paste! {
            #[must_use]
            #[allow(clippy::redundant_field_names)]
            pub fn [<$name:lower>]($($param: $type),*) -> impl Widget + '_ {
                $origin::$name($($arg),*)
            }
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(<$($generic_name:tt: $generic_value:tt),*>)?,
        // the name of the created and original widget
        name: $name:ident,
        // the path of the original widget's module (can't be a struct currently)
        origin: $origin:path,
        // the new widget's signature + the arguments passed into the original widget
        // the first argument is &self, referring to the parent
        create: |&$create_self:ident, $($param:ident: $type:ty),*| ($($arg:expr),* $(,)?) $(,)? 
    ) => {
        paste::paste! {
            #[allow(clippy::redundant_field_names)]
            impl$(<$($generic_name: $generic_value),*>)? $parent$(<$($generic_name),*>)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                pub fn [<$name:lower>]<'a>(&'a $create_self, $($param: $type),*) -> impl Widget + '_ {
                    $origin::$name($($arg),*)
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

/// Some common drawn object that's too complex to be included in [`Canvas`]
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
        name: title,
        args: (
            text: String [&str as to_string],
            foreground: Option<Color> [impl Into<Option<Color>> as into],
            background: Option<Color> [impl Into<Option<Color>> as into],
        ),
        size: |&self, _| {
            let len = self.text.len();
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

    pub trait Theme {
        fn title_fg(&self) -> Color;
        fn title_bg(&self) -> Color;
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
        name: title,
        origin: super::basic,
        create: |&self, text: &'a str| (
            text,
            self.theme.title_fg(),
            self.theme.title_bg(),
        )
    }
}

pub use themed::*;
