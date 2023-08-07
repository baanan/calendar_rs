//! Some default themes
//!
//! # List
//!
//! - [`Catppuccin`](catppuccin)
//!     - [`Latte`](catppuccin::Latte)
//!     - [`Frappe`](catppuccin::Frappe)
//!     - [`Macchiato`](catppuccin::Macchiato)
//!     - [`Mocha`](catppuccin::Mocha)
//! - [`OneDark`]

use crate::prelude::*;

use super::BasicTheme;

macro_rules! colors {
    ($($name:ident: ($r:expr, $g:expr, $b:expr)),* $(,)?) => {
        $(#[must_use] pub const fn $name() -> Color { Color::new($r, $g, $b) })*
    };
}

macro_rules! highlights {
    ($($name:ident),* $(,)?) => {
        const HIGHLIGHTS: &'static [Color] = &[$(Self::$name()),*];
    };
}

/// The One Dark color theme from Atom
///
/// Colors obtained from [here](https://github.com/joshdick/onedark.vim/blob/main/colors/onedark.vim)
pub struct OneDark;
impl OneDark {
    colors! {
        black: (40, 44, 52),
        mantle: (30, 33, 39),
        crust: (22, 24, 28),
        surface: (50, 56, 66),
        surface1: (64, 71, 84),
        surface2: (81, 90, 107),
        white: (171, 178, 191),
        light_red: (224, 108, 117),
        dark_red: (190, 80, 70),
        green: (152, 195, 121),
        light_yellow: (229, 192, 123),
        dark_yellow: (209, 154, 102),
        blue: (97, 175, 239),
        magenta: (198, 120, 221),
        cyan: (86, 182, 194),
        gutter_grey: (76, 82, 99),
        comment_grey: (92, 99, 112),
    }
    highlights![light_red, dark_red, green, light_yellow, dark_yellow, blue, magenta, cyan];
}

impl BasicTheme for OneDark {
    fn base() -> Color { Self::black() }
    fn mantle() -> Color { Self::mantle() }
    fn crust() -> Color { Self::crust() }

    fn surface() -> Color { Self::surface() }
    fn surface1() -> Color { Self::surface1() }
    fn surface2() -> Color { Self::surface2() }

    fn text() -> Color { Self::white() }
    fn subtext() -> Color { Self::comment_grey() }
    fn special_text() -> Color { Color::WHITE }

    fn success() -> Color { Self::green() }
    fn warning() -> Color { Self::light_yellow() }
    fn error() -> Color { Self::light_red() }
    fn link() -> Color { Self::cyan() }

    fn highlights() -> &'static [Color] {
        Self::HIGHLIGHTS
    }
}


/// The Catppuccin color theme
///
/// Colors obtained from [here](https://github.com/catppuccin/catppuccin)
pub mod catppuccin {
    use crate::{prelude::*, themes::BasicTheme};

    macro_rules! catppuccin {
        ($name:ident) => {
            impl BasicTheme for $name {
                fn base() -> Color { Self::base() }
                fn mantle() -> Color { Self::mantle() }
                fn crust() -> Color { Self::crust() }
                fn surface() -> Color { Self::surface0() }
                fn surface1() -> Color { Self::surface1() }
                fn surface2() -> Color { Self::surface2() }
                fn text() -> Color { Self::text() }
                fn subtext() -> Color { Self::subtext0() }

                fn special_text() -> Color { Color::WHITE }

                fn success() -> Color { Self::green() }
                fn warning() -> Color { Self::yellow() }
                fn error() -> Color { Self::red() }

                fn link() -> Color { Self::blue() }
                fn highlights() -> &'static [Color] {
                    Self::HIGHLIGHTS
                }
            }

            impl $name {
                highlights![rosewater, flamingo, pink, mauve, red, maroon, peach, yellow, green, teal, sky, sapphire, blue, lavender];
            }
        };
    }

    pub struct Latte;
    catppuccin!(Latte);
    impl Latte {
        colors! {
            rosewater: (220, 138, 120),
            flamingo: (221, 120, 120),
            pink: (234, 118, 203),
            mauve: (136, 57, 239),
            red: (210, 15, 57),
            maroon: (230, 69, 83),
            peach: (254, 100, 11),
            yellow: (223, 142, 29),
            green: (64, 160, 43),
            teal: (23, 146, 153),
            sky: (4, 165, 229),
            sapphire: (32, 159, 181),
            blue: (30, 102, 245),
            lavender: (114, 135, 253),
            text: (76, 79, 105),
            subtext1: (92, 95, 119),
            subtext0: (108, 111, 133),
            overlay2: (124, 127, 147),
            overlay1: (140, 143, 161),
            overlay0: (156, 160, 176),
            surface2: (172, 176, 190),
            surface1: (188, 192, 204),
            surface0: (204, 208, 218),
            base: (239, 241, 245),
            mantle: (230, 233, 239),
            crust: (220, 224, 232),
        }
    }

    pub struct Frappe;
    catppuccin!(Frappe);
    impl Frappe {
        colors! {
            rosewater: (242, 213, 207),
            flamingo: (238, 190, 190),
            pink: (244, 184, 228),
            mauve: (202, 158, 230),
            red: (231, 130, 132),
            maroon: (234, 153, 156),
            peach: (239, 159, 118),
            yellow: (229, 200, 144),
            green: (166, 209, 137),
            teal: (129, 200, 190),
            sky: (153, 209, 219),
            sapphire: (133, 193, 220),
            blue: (140, 170, 238),
            lavender: (186, 187, 241),
            text: (198, 208, 245),
            subtext1: (181, 191, 226),
            subtext0: (165, 173, 206),
            overlay2: (148, 156, 187),
            overlay1: (131, 139, 167),
            overlay0: (115, 121, 148),
            surface2: (98, 104, 128),
            surface1: (81, 87, 109),
            surface0: (65, 69, 89),
            base: (48, 52, 70),
            mantle: (41, 44, 60),
            crust: (35, 38, 52),
        }
    }

    pub struct Macchiato;
    catppuccin!(Macchiato);
    impl Macchiato {
        colors! {
            rosewater: (245, 224, 220),
            flamingo: (242, 205, 205),
            pink: (245, 194, 231),
            mauve: (203, 166, 247),
            red: (243, 139, 168),
            maroon: (235, 160, 172),
            peach: (250, 179, 135),
            yellow: (249, 226, 175),
            green: (166, 227, 161),
            teal: (148, 226, 213),
            sky: (137, 220, 235),
            sapphire: (116, 199, 236),
            blue: (137, 180, 250),
            lavender: (180, 190, 254),
            text: (205, 214, 244),
            subtext1: (186, 194, 222),
            subtext0: (166, 173, 200),
            overlay2: (147, 153, 178),
            overlay1: (127, 132, 156),
            overlay0: (108, 112, 134),
            surface2: (88, 91, 112),
            surface1: (69, 71, 90),
            surface0: (49, 50, 68),
            base: (30, 30, 46),
            mantle: (24, 24, 37),
            crust: (17, 17, 27),
        }
    }

    pub struct Mocha;
    catppuccin!(Mocha);
    impl Mocha {
        colors! {
            rosewater: (245, 224, 220),
            flamingo: (242, 205, 205),
            pink: (245, 194, 231),
            mauve: (203, 166, 247),
            red: (243, 139, 168),
            maroon: (235, 160, 172),
            peach: (250, 179, 135),
            yellow: (249, 226, 175),
            green: (166, 227, 161),
            teal: (148, 226, 213),
            sky: (137, 220, 235),
            sapphire: (116, 199, 236),
            blue: (137, 180, 250),
            lavender: (180, 190, 254),
            text: (205, 214, 244),
            subtext1: (186, 194, 222),
            subtext0: (166, 173, 200),
            overlay2: (147, 153, 178),
            overlay1: (127, 132, 156),
            overlay0: (108, 112, 134),
            surface2: (88, 91, 112),
            surface1: (69, 71, 90),
            surface0: (49, 50, 68),
            base: (30, 30, 46),
            mantle: (24, 24, 37),
            crust: (17, 17, 27),
        }
    }
}
