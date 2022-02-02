//! The module defines the [`Pattern`] enum. It is used by [`crate::giftwrap::GiftWrap`].
use std::fmt::*;

/// The `Color` enum represents a named color or a custom RGB value color to be used within the
/// `Pattern` enum, allowing `Pattern` to be customized.
///
/// !todo() Constrain the u8
/// !todo() Consider making this sRGB compliant?
#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8)
}

/// The `Pattern` enum represents various different Patterns that could be used by
/// [`crate::giftwrap::GiftWrap`]. Though it is used by the `GiftWrap`, the `Pattern` enum can be
/// used anywhere in Rust.
///
/// todo!() Make Patterns generic so that Patterns::Other(U) can be defined as needed.
#[derive(Debug, PartialEq)]
pub enum Pattern {
    Striped { first_stripe: Color, second_stripe: Color },
    Paisley { background: Color, foreground: Color },
    Polkadots { background: Color, foreground: Color },
    Sparkles { background: Color, foreground: Color },
    Cloth { color: Color },
    KraftPaper { color: Color },
    NewsPaper,
}

#[cfg(test)]
mod test {}
