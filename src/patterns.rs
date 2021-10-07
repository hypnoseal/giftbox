//! The module defines the [`Patterns`] enum. It is used by [`crate::giftwrap::GiftWrap`].
use std::fmt::*;

/// The `Patterns` enum represents various different Patterns that could be used by
/// [`crate::giftwrap::GiftWrap`]. Though it is used by the `GiftWrap`, the `Patterns` enum can be
/// used anywhere in Rust.
///
/// todo!() Make Patterns generic so that Patterns::Other(U) can be defined as needed.
#[derive(Debug, PartialEq)]
pub enum Patterns {
    Red,
    Blue,
    Green,
    Paisley,
    Polkadots,
    Sparkles,
    KraftPaper,
    NewsPaper,
    Cloth,
}

#[cfg(test)]
mod test {}
