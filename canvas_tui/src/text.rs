pub mod box_chars {
    use std::ops::{Index, IndexMut};

    /// A list of box chars
    ///
    /// They are organized in a binary number in order of up, down, left, right. For example:
    /// `0b1100` represents a vertical line (│) because both up and down are present, but not
    /// left and right.
    pub struct Chars {
        chars: [char; 16],
    }

    impl Chars {
        const fn new(chars: [char; 16]) -> Self {
            Self { chars }
        }

        #[must_use]
        pub fn vertical(&self) -> char { self[0b1100] }
        #[must_use]
        pub fn horizontal(&self) -> char { self[0b0011] }
    }

    impl Index<usize> for Chars {
        type Output = char;
        fn index(&self, index: usize) -> &Self::Output {
            &self.chars[index]
        }
    }

    impl IndexMut<usize> for Chars {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.chars[index]
        }
    }

    const EMPTY: [char; 16] = [' '; 16];

    /// Light box characters [as defined by unicode](https://en.wikipedia.org/wiki/Box-drawing_character)
    pub const LIGHT: Chars = {
        let mut chars = EMPTY;
        chars[0b0000] = ' ';
        chars[0b0001] = '╶';
        chars[0b0010] = '╴';
        chars[0b0011] = '─'; // horizontal!
        chars[0b0100] = '╷';
        chars[0b0101] = '┌';
        chars[0b0110] = '┐';
        chars[0b0111] = '┬';
        chars[0b1000] = '╵';
        chars[0b1001] = '└';
        chars[0b1010] = '┘';
        chars[0b1011] = '┴';
        chars[0b1100] = '│'; // vertical!
        chars[0b1101] = '├';
        chars[0b1110] = '┤';
        chars[0b1111] = '┼';
        Chars::new(chars)
    };

    /// Heavy box characters [as defined by unicode](https://en.wikipedia.org/wiki/Box-drawing_character)
    pub const HEAVY: Chars = {
        let mut chars = EMPTY;
        chars[0b0000] = ' ';
        chars[0b0001] = '╺';
        chars[0b0010] = '╸';
        chars[0b0011] = '━'; // horizontal!
        chars[0b0100] = '╻';
        chars[0b0101] = '┏';
        chars[0b0110] = '┓';
        chars[0b0111] = '┳';
        chars[0b1000] = '╹';
        chars[0b1001] = '┗';
        chars[0b1010] = '┛';
        chars[0b1011] = '┻';
        chars[0b1100] = '┃'; // vertical!
        chars[0b1101] = '┣';
        chars[0b1110] = '┫';
        chars[0b1111] = '╋';
        Chars::new(chars)
    };
}
