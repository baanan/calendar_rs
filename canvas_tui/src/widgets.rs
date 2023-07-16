use crate::{prelude::*, num::Size};

/// Constructs a [`Widget`] using the specified parameters
///
/// # Example
///
/// ```
/// use canvas_tui::prelude::*;
/// use canvas_tui::widgets::widget;
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
///     // gets the size of the widget
///     size: |self, _| { 
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
#[macro_export]
macro_rules! widget {
    (
        name: $name:ident,
        args: ( $($arg:ident: $type:ty $([$from:ty as $converter:ty])?),* $(,)? ), 
        size: |$sizeself:ident, $canvas_size:tt| $size:expr,
        draw: |$drawself:ident, $canvas:ident| $draw:expr,
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
        parent: $parent:ident$(<$($generic_name:tt: $generic_value:tt),*>)?,
        name: $name:ident,
        args: ( $($arg:ident: $type:ty $([$from:ty as $converter:ty])?),* $(,)? ),
        size: |$sizeself:ident, $canvas_size:tt| $size:expr,
        draw: |$drawself:ident, $canvas:ident| $draw:expr,
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

pub mod basic {
    use crate::prelude::*;

    widget! {
        name: title,
        args: (
            text: String [&str as to_string],
            foreground: Option<Color> [impl Into<Option<Color>> as into],
            background: Option<Color> [impl Into<Option<Color>> as into],
        ),
        size: |self, _| {
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

pub mod themed {
    use crate::prelude::*;

    pub trait Theme {
        fn title_fg(&self) -> Color;
        fn title_bg(&self) -> Color;
    }

    pub struct Themed<T: Theme> {
        pub theme: T
    }

    widget! {
        parent: Themed<T: Theme>,
        name: title,
        args: (
            text: String [&str as to_string]
        ),
        size: |self, _| {
            let len = self.text.len();
            let len: isize = len.try_into()
                .map_err(|_| Error::TooLarge("text length", len))?;
            Ok(Vec2::new(len + 2, 1))
        },
        draw: |self, canvas| {
            canvas.text(&Just::Centered, &self.text)
                .grow_profile(&(1, 0))
                .colored(self.parent.theme.title_fg(), self.parent.theme.title_bg())
                .discard_info()
        },
    }
}
