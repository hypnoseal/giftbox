//! This module defines the [`GiftTag`]. It is meant to be used by the [`crate::giftwrap::GiftWrap`]
//! module but can be used anywhere in Rust. The [`GiftTag`] can be imagined as a tag that is tied
//! to a wrapped gift box. It contains the following information in Strings:
//! * Recipient
//! * Sender
//! * Message
//!
//! # Examples
//! The [`GiftTag`] can be used on it's own in the following way:
//! ```
//! use giftbox::gifttag::GiftTag;
//! let tag = GiftTag::write(
//!      "Bob".to_string(),
//!      "Sally".to_string(),
//!      "Happy Cake Day!".to_string()
//! );
//! ```
//!
//! The [`GiftTag`] can be used with [`crate::giftwrap::GiftWrap`] and in the following way:
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

use std::fmt::*;

/// The `GiftTag` struct represents a gift tag that could be included with a gift's wrapping. It is
/// used by the `GiftWrap` struct to include a recipient, a sender, and a message. Though it is
/// utilized by `GiftWrap`, this struct can be used anywhere in Rust.
///
/// `GiftTag` has the following parameters:
/// * `recipient` which represents the recipient of the gift as a String.
/// * `sender` which represents the sender of the gift as a String.
/// * `message` which can be a message to be included with the gift as a String.
///
/// # Methods
/// ## write(recipient, sender, message)
/// You can write a new `GiftTag` with the [`GiftTag::write()`] method. Example:
/// ```
/// use giftbox::gifttag::GiftTag;
/// let tag = GiftTag::write(
///      "Bob".to_string(),
///      "Sally".to_string(),
///      "Happy Cake Day!".to_string()
/// );
/// ```
///
/// ## read()
/// You can read a `GiftTag` with the [`GiftTag::read()`] method. Example:
/// ```
/// use giftbox::gifttag::GiftTag;
/// let tag = GiftTag::write(
///      "Bob".to_string(),
///      "Sally".to_string(),
///      "Happy Cake Day!".to_string()
/// );
/// assert_eq!(tag.read(),
/// "To: Bob,\nFrom: Sally,\nMessage: Happy Cake Day!"
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct GiftTag {
    pub recipient: String,
    pub sender: String,
    pub message: String,
}

impl GiftTag {
    /// The `write` method accepts three arguments as Strings (a recipient, a sender, and a message)
    /// and returns a `GiftTag`.
    ///
    /// # Arguments
    /// * `recipient` - Accepts a String that represents a gift's recipient (the person receiving the
    /// gift).
    /// * `sender` - Accepts a String that represents a gift's sender (the person who sent the gift).
    /// * `message` - Accepts a string that represents a message to the recipient from the sender to
    /// be included with the gift.
    ///
    /// # Returns
    /// Returns a `GiftTag`.
    ///
    /// # Example
    /// ```
    /// use giftbox::gifttag::GiftTag;
    /// let tag = GiftTag::write(
    ///      "Bob".to_string(),
    ///      "Sally".to_string(),
    ///      "Happy Cake Day!".to_string()
    /// );
    /// ```
    pub fn write(recipient: String, sender: String, message: String) -> GiftTag {
        GiftTag {
            recipient,
            sender,
            message,
        }
    }

    /// The `read()` method takes a `GiftTag` as `self` and returns a formatted String representing
    /// the contents of the `GiftTag`.
    ///
    /// # Arguments
    /// * `self` only.
    ///
    /// # Returns
    /// Returns a pre-formatted String.
    ///
    /// # Example
    ///
    /// ```
    /// use giftbox::gifttag::GiftTag;
    /// let tag = GiftTag::write(
    ///      "Bob".to_string(),
    ///      "Sally".to_string(),
    ///      "Happy Cake Day!".to_string()
    /// );
    /// assert_eq!(tag.read(),
    /// "To: Bob,\nFrom: Sally,\nMessage: Happy Cake Day!"
    /// );
    /// ```
    ///
    pub fn read(self) -> String {
        format!(
            "To: {},\nFrom: {},\nMessage: {}",
            self.recipient, self.sender, self.message
        )
    }
}

#[cfg(test)]
mod test {}
