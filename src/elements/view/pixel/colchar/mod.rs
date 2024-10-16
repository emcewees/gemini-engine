use std::fmt::{self, Debug, Display};
mod colour;
mod modifier;
pub use colour::Colour;
pub use modifier::Modifier;
use std::fmt::Write; // Import the Write trait from std::fmt

/// We use `ColChar` to say exactly what each pixel should look like and what colour it should be. That is, the [`View`](super::super::View)'s canvas is just a vector of `ColChar`s under the hood. `ColChar` has the [`text_char`](ColChar::text_char) and [`modifier`](ColChar::modifier) properties. [`text_char`](ColChar::text_char) is the single ascii character used as the "pixel" when the [`View`](super::super::View) is rendered, whereas [`modifier`](ColChar::modifier) can give that pixel a colour or make it bold/italic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColChar {
    /// The actual character that will dictate the appearance of the pixel
    pub text_char: char,
    /// The modifier that will be applied to the text character
    pub modifier: Modifier,
}

impl ColChar {
    /// A solid █ character with no [`Modifier`].
    ///
    /// Using a sequence like this will create a red █ `ColChar`
    /// ```rs
    /// ColChar::SOLID.with_rgb(255,0,0)
    /// ```
    pub const SOLID: Self = Self {
        text_char: '*',
        // text_char: '█',
        modifier: Modifier::None,
    };
    /// A less solid ░ character with no [`Modifier`]
    pub const BACKGROUND: Self = Self {
        text_char: '.',
        // text_char: '░',
        modifier: Modifier::None,
    };
    /// A whitespace character with no [`Modifier`]
    pub const EMPTY: Self = Self {
        text_char: ' ',
        modifier: Modifier::None,
    };
    /// For use with the [`Sprite`](crate::elements::Sprite) and [`Text`](crate::elements::Text) elements, which consider a regular whitespace a transparent character
    pub const VOID: Self = Self {
        text_char: '-',
        // text_char: '\u{2008}',
        modifier: Modifier::None,
    };

    /// Create a new `ColChar` with a text character and a [`Modifier`]
    #[must_use]
    pub const fn new(text_char: char, modifier: Modifier) -> Self {
        Self {
            text_char,
            modifier,
        }
    }

    /// Return a `ColChar` with the same `modifier` and new `text_char`
    #[must_use]
    pub const fn with_char(self, text_char: char) -> Self {
        Self {
            text_char,
            modifier: self.modifier,
        }
    }

    /// Return a `ColChar` with the same `text_char` and new `modifier`
    #[must_use]
    pub const fn with_mod(self, modifier: Modifier) -> Self {
        Self {
            text_char: self.text_char,
            modifier,
        }
    }

    /// Return a `ColChar` with the same `text_char` and new `modifier` of the `Modifier::Colour` enum variant from RGB values
    #[must_use]
    pub const fn with_rgb(self, r: u8, g: u8, b: u8) -> Self {
        Self {
            text_char: self.text_char,
            modifier: Modifier::from_rgb(r, g, b),
        }
    }

    /// Return a `ColChar` with the same `text_char` and new `modifier` of the `Modifier::Colour` enum variant from HSV values
    #[must_use]
    pub fn with_hsv(self, h: u8, s: u8, v: u8) -> Self {
        Self {
            text_char: self.text_char,
            modifier: Modifier::from_hsv(h, s, v),
        }
    }

    /// Return a `ColChar` with the same `text_char` and new `modifier` of the `Modifier::Colour` enum variant from an HSV value
    #[must_use]
    pub const fn with_colour(self, colour: Colour) -> Self {
        Self {
            text_char: self.text_char,
            // modifier: Modifier::None,
            modifier: Modifier::Colour(colour),
        }
    }

    /// Return the displayed `ColChar`, omitting the `Modifier`s where necessary
    pub(crate) fn display_with_prev_and_next(
        self,
        f: &mut fmt::Formatter,
        prev_mod: Option<Modifier>,
        next_mod: Option<Modifier>,
        index: usize
    ) -> fmt::Result {
        let modifier = if prev_mod == Some(self.modifier) {
            Modifier::None
        } else {
            self.modifier
        };
        let end = if next_mod == Some(self.modifier) {
            Modifier::None
        } else {
            Modifier::END
        };

        // write!(f, "{}{}{}", modifier, index, end)
        write!(f, "{}{}{}", modifier, self.text_char, end)
    }
    
    /// Writes the displayed `ColChar`, omitting the `Modifier`s where necessary
    pub(crate) fn write_with_prev_and_next(
        self,
        o: &mut std::string::String,
        prev_mod: Option<Modifier>,
        next_mod: Option<Modifier>,
        index:usize 
    ) -> fmt::Result {
        let modifier = if prev_mod == Some(self.modifier) {
            Modifier::None
        } else {
            // Modifier::None
            self.modifier
        };
        let end = if next_mod == Some(self.modifier) {
            Modifier::None
        } else {
            Modifier::END
        };

        write!(o, "{}{}{}", modifier, self.text_char, end)
        // write!(o, "{}{}{}", modifier, index , end)
    }
}

impl Default for ColChar {
    fn default() -> Self {
        Self::SOLID
    }
}

impl Display for ColChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.modifier {
            Modifier::None => write!(f, "{}", self.text_char),
            _ => write!(f, "{}{}{}", self.modifier, self.text_char, Modifier::END),
        }
    }
}
