//! Drawable objects that are too complex to be included in [`Canvas`]
//!
//! Examples include buttons, text boxes, or page titles. To draw one, use [`Canvas::draw`].
//!
//! Use [`basic`], [`themed`], or [`selectable`] for built-in widgets, or create new ones using
//! [`widget!`]

use crate::{prelude::*, num::Size};

/// Constructs a [`Widget`] using the specified parameters
///
/// # Examples
///
/// ## Creating a new widget
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::prelude::*;
///
/// widget! {
///     // you can also specify a struct to be a parent, in which the function becomes a method
///     // within the parent, and a reference to the parent is stored within the widget as `parent`
///     // parent: Parent,
///     name: title,
///     args: ( 
///         text: String [&str as to_string], // converters can be used either like this
///      // text: String [&str > .to_string()], // or like this (to chain extra methods)
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
/// use widgets::prelude::*;
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
///
/// ## Optional Arguments
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::prelude::*;
///
/// widget! {
///     name: title,
///     args: ( 
///         text: String [&str as to_string],
///     ),
///     optionals: (
///         // each argument has to be in an option
///         foreground: Option<Color>,
///         background: Option<Color>, 
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
///     canvas.draw(&Just::Centered, title("foo").foreground(Color::BLACK))?;
///
///     // ·······
///     // ·-foo-· highlight represented by -
///     // ·······
///     assert_eq!(canvas.get(&(0, 1))?.foreground, None);
///     assert_eq!(canvas.get(&(1, 1))?.foreground, Some(Color::BLACK));
///     assert_eq!(canvas.get(&(1, 1))?.background, None);
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
        args: ( $($arg:ident: $type:ty $([$from:ty $(as $method:ident)? $(> $($rest:tt)*)?])?),* $(,)? ),
        // any optional arguments
        // each is None by default, and can be set using methods with the same name
        $(optionals: ( $($optional_name:ident: Option<$optional_type:ty>),* $(,)? ),)?
        // returns the size of the widget
        size: |&$sizeself:ident, $canvas_size:tt| $size:expr,
        // draws the widget onto `canvas`
        draw: |$drawself:ident, $canvas:ident| $draw:expr $(,)?
    ) => {
        paste::paste! {
            $crate::optional_attr!(
                !($($($optional_name)*)?)
                (#[doc(hidden)])
                pub struct [<$name:camel>] {
                    $($arg: $type),*
                    $(,$($optional_name: Option<$optional_type>),*)?
                }
            );

            use $crate::num::Size;
            impl Widget for [<$name:camel>] {
                fn size(&$sizeself, $canvas_size: &impl Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            #[must_use]
            #[allow(clippy::redundant_field_names)]
            #[cfg(not(doc))]
            $(#[$($attrs)*])*
            pub fn [<$name:lower>]($($arg: $crate::rightmost!(($type) $(($from))?)),*) -> [<$name:camel>] {
                [<$name:camel>] {
                    $($arg: $crate::rightmost!(
                        ($arg$($(.$method())?)?)
                        $($(($arg$($rest)*))?)?
                    )),*
                    $(,$($optional_name: None),*)?
                }
            }

            // use the full name only if there are optionals
            // otherwise, just use impl Widget
            $crate::select!(($($($optional_name)*)?)
                (#[cfg(doc)] $(#[$($attrs)*])* pub fn [<$name:lower>]($($arg: $crate::rightmost!(($type) $(($from))?)),*) 
                    -> impl Widget { })
                (#[cfg(doc)] $(#[$($attrs)*])* pub fn [<$name:lower>]($($arg: $crate::rightmost!(($type) $(($from))?)),*) 
                    -> [<$name:camel>] { })
            );

            impl [<$name:camel>] {
                $($(
                    #[must_use]
                    #[allow(clippy::missing_const_for_fn)] // clippy wrong yet again
                    pub fn $optional_name(self, $optional_name: $optional_type) -> Self {
                        Self { $optional_name: Some($optional_name), ..self }
                    }
                )*)?
            }
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty $(as $method:ident)? $(> $($rest:tt)*)?])?),* $(,)? ),
        // any optional arguments
        // each is None by default, and can be set using methods with the same name
        $(optionals: ( $($optional_name:ident: Option<$optional_type:ty>),* $(,)? ),)?
        // returns the size of the widget
        size: |&$sizeself:ident, $canvas_size:tt| $size:expr,
        // draws the widget onto `canvas`
        draw: |$drawself:ident, $canvas:ident| $draw:expr $(,)?
    ) => {
        paste::paste! {
            $crate::optional_attr!(
                !($($($optional_name)*)?)
                (#[doc(hidden)])
                pub struct [<$name:camel>]<'a $(, $($generic_name: $generic_value),*)?> {
                    parent: &'a $parent$(<$($generic_name),*>)?, 
                    $($arg: $type),*
                    $(,$($optional_name: Option<$optional_type>),*)?
                }
            );

            use $crate::num::Size;
            impl<'a $(, $($generic_name: $generic_value),*)?> Widget for [<$name:camel>]<'a $(, $($generic_name),*)?> {
                fn size(&$sizeself, $canvas_size: &impl Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            impl$(< $($generic_name: $generic_value),* >)? $parent$(< $($generic_name),* >)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                #[cfg(not(doc))]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>](&self, $($arg: $crate::rightmost!(($type) $(($from))?)),*) 
                    -> [<$name:camel>]<'_ $(, $($generic_name),*)?> 
                {
                    [<$name:camel>] { parent: self, 
                        $($arg: $crate::rightmost!(
                            ($arg$($(.$method())?)?)
                            $($(($arg$($rest)*))?)?
                        )),*
                        $(,$($optional_name: None),*)?
                    }
                }

                // use the full name only if there are optionals
                // otherwise, just use impl Widget
                $crate::select!(($($($optional_name)*)?)
                    (#[cfg(doc)] $(#[$($attrs)*])* pub fn [<$name:lower>](&self, $($arg: $crate::rightmost!(($type) $(($from))?)),*) 
                        -> impl Widget + '_ { })
                    (#[cfg(doc)] $(#[$($attrs)*])* pub fn [<$name:lower>](&self, $($arg: $crate::rightmost!(($type) $(($from))?)),*) 
                        -> [<$name:camel>]<'_ $(, $($generic_name),*)?> { })
                );
            }

            impl<'a $(, $($generic_name: $generic_value),*)?> [<$name:camel>]<'a $(, $($generic_name),*)?> {
                $($(
                    #[must_use]
                    #[allow(clippy::missing_const_for_fn)] // clippy wrong yet again
                    pub fn $optional_name(self, $optional_name: $optional_type) -> Self {
                        Self { $optional_name: Some($optional_name), ..self }
                    }
                )*)?
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
        parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,
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
            impl$(< $($generic_name: $generic_value),* >)? $parent$(< $($generic_name),* >)? {
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
    };
    // (...)* (...) -- never used in normal practice
    (($($left:tt)*) $($rest:tt)* ) => {
        $crate::rightmost($($rest)*)
    }
}

// just used in the above macro
// if the first () is present, then the right () is used
// otherwise, the left () is used
// ! flips this behavior
#[doc(hidden)]
#[macro_export]
macro_rules! select {
    (() ($($left:tt)*) ($($right:tt)*)) => { $($left)* };
    (($($cond:tt)*) ($($left:tt)*) ($($right:tt)*)) => { $($right)* };
    (!() ($($left:tt)*) ($($right:tt)*)) => { $($right)* };
    (!($($cond:tt)*) ($($left:tt)*) ($($right:tt)*)) => { $($left)* };
}

// just used in the above macro
// if the first () is present, then the attr is used
// otherwise, it isn't
// ! flips this behavior
#[doc(hidden)]
#[macro_export]
macro_rules! optional_attr {
    (() ($($attr:tt)*) $item:item) => { $item };
    (($($cond:tt)*) ($($attr:tt)*) $item:item) => { $($attr)* $item };
    (!() ($($attr:tt)*) $item:item) => { $($attr)* $item };
    (!($($cond:tt)*) ($($attr:tt)*) $item:item) => { $item };
}

pub use widget;
/// Useful imports for developing widgets
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

pub mod basic;
pub mod themed;
pub mod selectable;
pub use themed::*;
pub use selectable::*;
