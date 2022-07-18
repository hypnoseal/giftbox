//! This module defines the [`GiftWrap`] struct. This can be used by [`crate::giftbox::GiftBox`] to
//! wrap itself. It is meant to represent wrapping paper used to wrap a gift box. The struct has
//! four fields:
//! * **contents** - which is a generic data type that can hold any Rust definable type.
//! * **pattern** - the [`crate::patterns::Patterns`] enum which represents the type of wrapping
//! paper.
//! * **has_box** - a boolean which represents whether or not the `GiftWrap` has a bow.
//! * **tag** - an `Option<GiftTag>`, where the `GiftTag` is a struct representing a gift tag the
//! `GiftWrap` that contains the recipient, sender, and a message.
//!
//! # Examples
//! Wrap and unwrap a [`crate::giftbox::GiftBox`] with a tag:
//! ```
//! use giftbox::giftbox::GiftBox;
//! use giftbox::gifttag::GiftTag;
//! use giftbox::pattern::{ Pattern, Color };
//! let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
//! let tag = GiftTag::write(
//!     "Bob".to_string(),
//!     "Sally".to_string(),
//!     "Happy Cake Day!".to_string()
//! );
//! let wrapped_box = filled_box.wrap(
//!     Pattern::Polkadots { background: Color::White, foreground: Color::Black },
//!     true,
//!     Some(tag)
//! );
//! let unwrapped_box = wrapped_box.unwrap();
//! assert_eq!(unwrapped_box, filled_box);
//! ```
//!
//! todo!() Turn GiftWrap into a trait.

use crate::gifttag::GiftTag;
use crate::pattern::Pattern;
use std::fmt::*;

/// A `GiftWrap` type for Rust which represents gift wrap that can be wrapped around any other
/// type that can be represented as a Rust type.
///
/// `GiftWrap` is a Rust struct which has the following parameters:
/// * `contents` which can be any type `T`.
/// * `pattern` which represents the pattern of the `GiftWrap` defined in the `Patterns` enum.
/// * `has_bow` which is a boolean which represents whether or not the `GiftWrap` has a bow.
/// * `tag` which is an `Option<GiftTag>`, where the `GiftTag` is a struct representing a gift tag
/// on the `GiftWrap` that contains the recipient, sender, and a message.
///
/// # Methods
/// ## unwrap()
/// Unwrap `GiftWrap` to get its contents with the [`GiftWrap::unwrap()`] method. Example:
/// ```
/// use giftbox::giftbox::GiftBox;
/// use giftbox::giftwrap::GiftWrap;
/// use giftbox::pattern::{ Pattern, Color };
/// let wrapped_string_gift = GiftWrap {
///     contents: GiftBox::Gifts("String of words".to_string()),
///     pattern: Pattern::Sparkles { background: Color::White, foreground: Color::Green },
///     has_bow: true,
///     tag: None
/// };
/// let unwrapped_string_gift = wrapped_string_gift.unwrap();
/// assert_eq!(unwrapped_string_gift, GiftBox::Gifts("String of words".to_string()));
/// ```
///
/// ## read_tag()
/// "Read" the `GiftTag` of `GiftWrap`. More specifically, get a String of the `GiftTags` contents
/// with the [`GiftWrap::read_tag()`] Example:
/// ```
/// use giftbox::giftbox::GiftBox;
/// use giftbox::gifttag::GiftTag;
/// use giftbox::pattern::{ Pattern, Color };
/// let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
/// let tag = GiftTag::write(
///     "Bob".to_string(),
///     "Sally".to_string(),
///     "Happy Cake Day!".to_string()
/// );
/// let wrapped_box = filled_box.wrap(
///     Pattern::Polkadots { background: Color::White, foreground: Color::Black },
///     true,
///     Some(tag)
/// );
/// assert_eq!(
///     wrapped_box.read_tag(),
///     "You read the nice Gift Tag and it says:\nTo: Bob,\nFrom: Sally,\nMessage: Happy Cake Day!"
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct GiftWrap<T> {
    pub contents: T,
    pub pattern: Pattern,
    pub has_bow: bool,
    pub tag: Option<GiftTag>,
}

impl<T> GiftWrap<T> {
    /// The `unwrap()` method takes the `GiftWrap` and unwraps it to reveal its contents.
    ///
    /// # Arguments
    /// * `self` only.
    ///
    /// # Returns
    /// Returns `T` where `T` is the contents of `Giftwrap.contents`.
    ///
    /// # Example
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// use giftbox::gifttag::GiftTag;
    /// use giftbox::pattern::{ Pattern, Color };
    /// let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Pattern::Polkadots { background: Color::White, foreground: Color::Black },
    ///     true,
    ///     Some(tag)
    /// );
    /// let unwrapped_box = wrapped_box.unwrap();
    /// assert_eq!(unwrapped_box, filled_box);
    /// ```
    pub fn unwrap(self) -> T {
        self.contents
    }

    /// The `read_tag()` method takes a `GiftWrap` and returns the contents of a `GiftTag` as a
    /// String. If there is no `GiftTag` (`self.tag` is `None`) then a default String is returned.
    ///
    /// # Arguments
    /// * `self` only.
    ///
    /// # Returns
    /// Returns a String returned from [`GiftTag::read()`] if there is `Some(tag)`. Otherwise, if
    /// there is `None` it returns a default String.
    ///
    /// # Example
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// use giftbox::gifttag::GiftTag;
    /// use giftbox::pattern::{ Pattern, Color };
    /// let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Pattern::Polkadots { background: Color::White, foreground: Color::Black },
    ///     true,
    ///     Some(tag)
    /// );
    /// assert_eq!(
    ///     wrapped_box.read_tag(),
    ///     "You read the nice Gift Tag and it says:\nTo: Bob,\nFrom: Sally,\nMessage: Happy Cake Day!"
    /// );
    /// ```
    pub fn read_tag(self) -> String {
        match self.tag {
            Some(tag) => tag.read(),
            None => "There was no tag to read.".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::giftbox::GiftBox;
    use crate::pattern::Color;

    #[test]
    fn wrap_gift_box_with_tag() {
        let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string(),
        );
        let pattern = Pattern::Polkadots { background: Color::Red, foreground: Color::Blue };
        let wrapped_box = filled_box.wrap(pattern, true, Some(tag));
        assert_eq!(wrapped_box, {
            GiftWrap {
                contents: { GiftBox::Gifts(["Toys", "Candy", "Money"]) },
                pattern: Pattern::Polkadots { background: Color::Red, foreground: Color::Blue },
                has_bow: true,
                tag: Some(GiftTag {
                    recipient: "Bob".to_string(),
                    sender: "Sally".to_string(),
                    message: "Happy Cake Day!".to_string(),
                }),
            }
        })
    }

    #[test]
    fn wrap_and_unwrap_gift_box() {
        let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string(),
        );
        let pattern = Pattern::Striped { first_stripe: Color::Green, second_stripe: Color::Blue };
        let wrapped_box = filled_box.wrap(pattern, true, Some(tag));
        let unwrapped_box = wrapped_box.unwrap();
        assert_eq!(unwrapped_box, filled_box);
    }

    #[test]
    fn wrap_and_open_gift_box_with_tag() {
        let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string(),
        );
        let pattern = Pattern::Cloth { color: Color::Blue };
        let wrapped_box = filled_box.wrap(pattern, true, Some(tag));
        assert_eq!(wrapped_box.unwrap().open(), ["Toys", "Candy", "Money"]);
    }

    #[test]
    fn write_and_read_tag() {
        let filled_box = GiftBox::from(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string(),
        );
        let pattern = Pattern::NewsPaper;
        let wrapped_box = filled_box.wrap(pattern, true, Some(tag));
        assert_eq!(
            wrapped_box.read_tag(),
            "You read the nice Gift Tag and it says:\nTo: Bob,\nFrom: Sally,\nMessage: Happy Cake Day!"
        );
    }

    #[test]
    fn attempt_to_read_none_tag() {
        let wrapped_nothing = GiftWrap {
            contents: (),
            pattern: Pattern::KraftPaper { color: Color::Red },
            has_bow: false,
            tag: None,
        };
        assert_eq!(wrapped_nothing.read_tag(), "There was no tag to read.");
    }
}
