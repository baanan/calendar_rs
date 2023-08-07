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
/// ## Widget Extensions
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::prelude::*;
///
/// widget! {
///     name: title,
///     origin: title in widgets::basic,
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
///
/// ## Extension Optionals
///
/// ```
/// use canvas_tui::prelude::*;
/// use widgets::prelude::*;
///
/// widget! {
///     name: title,
///     args: ( 
///         text: String [&str as to_string],
///         foreground: Option<Color>,
///         background: Option<Color>, 
///     ),
///     size: |&self, _| { 
///         // omitted //
/// #       let len = self.text.chars().count();
/// #       let len: isize = len.try_into()
/// #           .map_err(|_| Error::TooLarge("text length", len))?;
/// #       Ok(Vec2::new(len + 2, 1))
///     },
///     draw: |self, canvas| { 
///         // omitted //
/// #       canvas.text(&Just::Centered, &self.text)
/// #           .grow_profile(&(1, 0))
/// #           .colored(self.foreground, self.background)
/// #           .discard_info()
///     },
/// }
///
/// widget! {
///     name: optional_title,
///     origin: title in self,
///     args: (
///         text: String [&str as to_string],
///     ),
///     optionals: (
///         foreground: Option<Color>,
///         background: Option<Color>,
///     ),
///     build: |self| (
///         &self.text,
///         self.foreground,
///         self.background,
///     )
/// }
///
/// fn main() -> Result<(), Error> {
///     let mut canvas = Basic::new(&(7, 3));
///     canvas.draw(&Just::Centered, optional_title("foo").foreground(Color::BLACK))?;
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
        $crate::paste! {
            $crate::optional_attr!(
                !($($($optional_name)*)?)
                (#[doc(hidden)])
                #[doc = "See [`" $name "`]"]
                pub struct [<$name:camel>] {
                    $($arg: $type),*
                    $(,$($optional_name: Option<$optional_type>),*)?
                }
            );

            impl Widget for [<$name:camel>] {
                fn size(&$sizeself, $canvas_size: &impl $crate::num::Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            #[must_use]
            #[allow(clippy::redundant_field_names)]
            #[cfg(not(doc))]
            $(#[$($attrs)*])*
            pub fn [<$name:lower>]($($arg: $crate::first!($(($from))? ($type))),*) -> [<$name:camel>] {
                [<$name:camel>] {
                    $($arg: $crate::first!(
                        $($(($arg$($rest)*))?)?
                        ($arg$($(.$method())?)?)
                    )),*
                    $(,$($optional_name: None),*)?
                }
            }

            // use the full name only if there are optionals
            // otherwise, just use impl Widget
            $crate::select_return_value!(select
                ($($($optional_name)*)?) 
                ([<$name:camel>])
                (impl Widget)
                #[cfg(doc)] $(#[$($attrs)*])* 
                pub fn [<$name:lower>]($($arg: $crate::first!($(($from))? ($type))),*) -> _ {  }
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
        $crate::paste! {
            $crate::optional_attr!(
                !($($($optional_name)*)?)
                (#[doc(hidden)])
                #[doc = "See [`" $parent "::" $name "`]"]
                pub struct [<$name:camel>]<'a $(, $($generic_name: $generic_value),*)?> {
                    parent: &'a $parent$(<$($generic_name),*>)?, 
                    $($arg: $type),*
                    $(,$($optional_name: Option<$optional_type>),*)?
                }
            );

            impl<'a $(, $($generic_name: $generic_value),*)?> Widget for [<$name:camel>]<'a $(, $($generic_name),*)?> {
                fn size(&$sizeself, $canvas_size: &impl $crate::num::Size) -> Result<Vec2, Error> { $size }
                fn draw<C: Canvas>($drawself, $canvas: &mut C) -> Result<(), Error> { $draw }
                fn name() -> &'static str { stringify!($name) }
            }

            impl$(< $($generic_name: $generic_value),* >)? $parent$(< $($generic_name),* >)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                #[cfg(not(doc))]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>](&self, $($arg: $crate::first!($(($from))? ($type))),*) 
                    -> [<$name:camel>]<'_ $(, $($generic_name),*)?> 
                {
                    [<$name:camel>] { parent: self, 
                        $($arg: $crate::first!(
                            $($(($arg$($rest)*))?)?
                            ($arg$($(.$method())?)?)
                        )),*
                        $(,$($optional_name: None),*)?
                    }
                }

                // use the full name only if there are optionals
                // otherwise, just use impl Widget
                $crate::select_return_value!(select
                    ($($($optional_name)*)?) 
                    ([<$name:camel>]<'_ $(, $($generic_name),*)?>)
                    (impl Widget + '_)
                    #[cfg(doc)] $(#[$($attrs)*])* 
                    pub fn [<$name:lower>](&self, $($arg: $crate::first!($(($from))? ($type))),*) -> _ {  }
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
    // widgets that are based on other widgets,
    // just changing around the arguments
    (
        // the parent struct that the widget becomes a method of
        $(parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,)?
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the created and original widget
        name: $name:ident,
        // the path of the original widget's function (can't be a method currently)
        origin: $origin:ident in $path:path,
        // the return value of the function, default: `impl Widget` 
        $(return_value: $return:ty,)?
        // the new widget's signature + the arguments passed into the original widget
        // if the widget has a parent, the first argument must be &self, referring to it
        // note: all the arguments have to have types
        create: |$(&$create_self:ident,)? $($param:ident: $type:ty),*| 
            ($($arg:expr),* $(,)?)
            $(.$option:ident($val:expr))* $(,)? 
    ) => {
        $crate::paste!{ 
            $crate::widget!(
                $(parent: $parent$(< $($generic_name: $generic_value),* >)?,)?
                $(#[$($attrs)*])*
                name: $name,
                $(return_value: $return,)?
                create: |$(&$create_self,)? $($param: $type),*| { $path::$origin($($arg),*)$(.$option($val))* }
            );
        }
    };
    // widgets that are based on other widgets,
    // but that need to do some extra operations on the input arguments
    // so they just call the origin function themselves inside a block
    (
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the created and original widget
        name: $name:ident,
        // the return value of the function
        // useful when the origin function has optional values
        $(return_value: $return_value:ty,)?
        // the new widget's signature + the arguments passed into the original widget
        // note: all the arguments have to have types
        create: |$($param:ident: $type:ty),*| { $($body:tt)* } $(,)?
    ) => {
        $crate::paste! {
            $crate::select_return_value!(first
                ($($return_value)?)
                (impl Widget + '_)
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>]($($param: $type),*) -> _ { $($body)* }
            );
        }
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the created and original widget
        name: $name:ident,
        // the return value of the function
        // useful when the origin function has optional values
        $(return_value: $return_value:ty,)?
        // the new widget's signature + the arguments passed into the original widget
        // the first argument is &self, referring to the parent
        // note: all the arguments have to have types
        create: |&$create_self:ident, $($param:ident: $type:ty),*| { $($body:tt)* } $(,)? 
    ) => {
        $crate::paste! {
            impl$(< $($generic_name: $generic_value),* >)? $parent$(< $($generic_name),* >)? {
                $crate::select_return_value!(first
                    ($($return_value)?)
                    (impl Widget + '_)
                    #[must_use]
                    #[allow(clippy::redundant_field_names)]
                    $(#[$($attrs)*])*
                    pub fn [<$name:lower>]<'a>(&'a $create_self, $($param: $type),*) -> _ { $($body)* }
                );
            }
        }
    };
    // widget extensions with optionals
    // behind the hood, the macro creates a builder
    // that implements WidgetSource with `build`
    (
        // the parent struct that the widget becomes a method of
        $(parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,)?
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the origin of the widget
        origin: $func:ident in $path:path,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty $(as $method:ident)? $(> $($rest:tt)*)?])?),* $(,)? ),
        // any optional arguments
        // each is None by default, and can be set using methods with the same name
        optionals: ( $($optional_name:ident: Option<$optional_type:ty>),* $(,)? ),
        // a function to build the origin widget from this widget
        build: |$self:ident| 
            ($($buildarg:expr),* $(,)?) 
            $(.$option:ident($val:expr))* $(,)?
    ) => {
        $crate::widget!(
            $(parent: $parent$(< $($generic_name: $generic_value),* >)?,)?
            $(#[$($attrs)*])*
            name: $name,
            origin: $func in $path,
            args: ( $($arg: $type $([$from $(as $method)? $(> $($rest)*)?])?),* ),
            optionals: ( $($optional_name: Option<$optional_type>),* ),
            build: |$self| { $path::$func($($buildarg),*)$(.$option($val))* }
        );
    };
    (
        // the parent struct that the widget becomes a method of
        parent: $parent:ident$(< $($generic_name:ident: $generic_value:ty),* >)?,
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the origin of the widget
        origin: $func:ident in $path:path,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty $(as $method:ident)? $(> $($rest:tt)*)?])?),* $(,)? ),
        // any optional arguments
        // each is None by default, and can be set using methods with the same name
        optionals: ( $($optional_name:ident: Option<$optional_type:ty>),* $(,)? ),
        // a function to build the origin widget from this widget
        build: |$self:ident| { $($body:tt)* } $(,)?
    ) => {
        $crate::paste! {
            #[doc = "See [`" $parent "::" $name "`]"]
            pub struct [<$name:camel>]<'a $(, $($generic_name: $generic_value),*)?> {
                parent: &'a $parent$(<$($generic_name),*>)?, 
                $($arg: $type),*,
                $($optional_name: Option<$optional_type>),*
            }

            impl<'a $(, $($generic_name: $generic_value),*)?> WidgetSource 
                for [<$name:camel>]<'a $(, $($generic_name),*)?> 
            {
                type Output = $path::[<$func:camel>];
                fn build($self) -> Self::Output { $($body)* }
            }

            impl$(< $($generic_name: $generic_value),* >)? $parent$(< $($generic_name),* >)? {
                #[must_use]
                #[allow(clippy::redundant_field_names)]
                $(#[$($attrs)*])*
                pub fn [<$name:lower>](&self, $($arg: $crate::first!($(($from))? ($type))),*) 
                    -> [<$name:camel>]<'_ $(, $($generic_name),*)?> 
                {
                    [<$name:camel>] { parent: self, 
                        $($arg: $crate::first!(
                            $($(($arg$($rest)*))?)?
                            ($arg$($(.$method())?)?)
                        )),*,
                        $($optional_name: None),*
                    }
                }
            }

            impl<'a $(, $($generic_name: $generic_value),*)?> [<$name:camel>]<'a $(, $($generic_name),*)?> {
                $(
                    #[must_use]
                    #[allow(clippy::missing_const_for_fn)] // clippy wrong yet again
                    pub fn $optional_name(self, $optional_name: $optional_type) -> Self {
                        Self { $optional_name: Some($optional_name), ..self }
                    }
                )*
            }
        }       
    };
    (
        // optional doc comments
        $(#[$($attrs:tt)*])*
        // the name of the widget and the function that creates it
        name: $name:ident,
        // the origin of the widget
        origin: $func:ident in $path:path,
        // the arguments for the creation function
        args: ( $($arg:ident: $type:ty $([$from:ty $(as $method:ident)? $(> $($rest:tt)*)?])?),* $(,)? ),
        // any optional arguments
        // each is None by default, and can be set using methods with the same name
        optionals: ( $($optional_name:ident: Option<$optional_type:ty>),* $(,)? ),
        // a function to build the origin widget from this widget
        build: |$self:ident| { $($body:tt)* } $(,)?
    ) => {
        $crate::paste! {
            #[doc = "See [`" $name "`]"]
            pub struct [<$name:camel>] {
                $($arg: $type),*,
                $($optional_name: Option<$optional_type>),*
            }

            impl WidgetSource for [<$name:camel>] {
                type Output = $path::[<$func:camel>];
                fn build($self) -> Self::Output { $($body)* }
            }

            #[must_use]
            #[allow(clippy::redundant_field_names)]
            $(#[$($attrs)*])*
            pub fn [<$name:lower>]($($arg: $crate::first!($(($from))? ($type))),*) -> [<$name:camel>] {
                [<$name:camel>] {
                    $($arg: $crate::first!(
                        $($(($arg$($rest)*))?)?
                        ($arg$($(.$method())?)?)
                    )),*,
                    $($optional_name: None),*
                }
            }

            impl [<$name:camel>] {
                $(
                    #[must_use]
                    #[allow(clippy::missing_const_for_fn)] // clippy wrong yet again
                    pub fn $optional_name(self, $optional_name: $optional_type) -> Self {
                        Self { $optional_name: Some($optional_name), ..self }
                    }
                )*
            }
        }
    }
}

// just used in the above macro
#[doc(hidden)]
#[macro_export]
macro_rules! first {
    // (...)* (...)
    (($($left:tt)*) $($rest:tt)* ) => {
        $($left)*
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! select_return_value {
    // first if it exists
    (first () ($($right:tt)*) 
        $(#[$attr:meta])* $vis:vis fn $name:ident$(<$($lifetimes:lifetime),*>)?($($args:tt)*) -> _ { $($body:tt)* }
    ) => {
        $(#[$attr])* $vis fn $name$(<$($lifetimes),*>)?($($args)*) -> $($right)* { $($body)* }
    };
    (first ($($left:tt)*) ($($right:tt)*) 
        $(#[$attr:meta])* $vis:vis fn $name:ident$(<$($lifetimes:lifetime),*>)?($($args:tt)*) -> _ { $($body:tt)* }
    ) => {
        $(#[$attr])* $vis fn $name$(<$($lifetimes),*>)?($($args)*) -> $($left)* { $($body)* }
    };
    // left if there's nothing, otherwise right
    (select () ($($left:tt)*) ($($right:tt)*) 
        $(#[$attr:meta])* $vis:vis fn $name:ident$(<$($lifetimes:lifetime),*>)?($($args:tt)*) -> _ { $($body:tt)* }
    ) => {
        $(#[$attr])* $vis fn $name$(<$($lifetimes),*>)?($($args)*) -> $($left)* { $($body)* }
    };
    (select ($($cond:tt)*) ($($left:tt)*) ($($right:tt)*) 
        $(#[$attr:meta])* $vis:vis fn $name:ident$(<$($lifetimes:lifetime),*>)?($($args:tt)*) -> _ { $($body:tt)* }
    ) => {
        $(#[$attr])* $vis fn $name$(<$($lifetimes),*>)?($($args)*) -> $($right)* { $($body)* }
    };
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

// returns the result if there are tokens inside the first () and they aren't false
#[doc(hidden)]
#[macro_export]
macro_rules! and {
    (() $($tt:tt)*) => { };
    ((false) $($tt:tt)*) => { };
    (($($cond:tt)*) $($tt:tt)*) => { $($tt)* };
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

/// A source of a [widget](Widget)
///
/// This can be a [widget](Widget) itself or a builder of a widget (such as when optionals are
/// provided on a [widget extension](widget#extension-optionals))
pub trait WidgetSource {
    type Output: Widget;
    /// Builds the source into a widget
    fn build(self) -> Self::Output;
}

impl<W: Widget> WidgetSource for W {
    type Output = Self;
    fn build(self) -> Self::Output { self }
}

/// Truncate `string` to `max_width` optionally from the end if specified
fn truncate(string: &str, max_width: Option<usize>, from_end: bool) -> String {
    if let Some(max_width) = max_width {
        if string.len() > max_width {
            return truncate_unchecked(string, max_width, from_end);
        }
    }
    string.to_string()
}

/// Truncate `string` to `max_width` optionally from the end if specified
///
/// # Panics
///
/// - If the `string`'s length is smaller than `max_width`
fn truncate_unchecked(string: &str, max_width: usize, from_end: bool) -> String {
    if from_end {
        string[(string.len() - max_width)..].to_string()
    } else {
        string[..max_width].to_string()
    }
}

pub mod basic;
pub mod themed;
pub mod selectable;
pub use themed::{Themed, Theme};
pub use selectable::{Selectable, SelectableTheme};
