//! This module defines the [`GiftBox`] enum which is the base generic type used in this crate. It
//! represents a real-life gift box which can hold any gift that can fit within the box. The enum
//! has two variants:
//!
//! * Gifts(T) - Which represents a gift box with whatever `Gifts` contained as `T`.
//! * Empty - Which represents an empty gift box.
//!
//! # Examples
//!
//! ```
//! use giftbox::giftbox::GiftBox;
//! use giftbox::gifttag::GiftTag;
//! use giftbox::giftwrap::GiftWrap;
//! use giftbox::pattern::{ Pattern, Color };
//! let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
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
//! assert_eq!(
//!      wrapped_box,
//!      {
//!          GiftWrap {
//!              contents:{
//!                  GiftBox::Gifts(["Toys", "Candy", "Money"])
//!              },
//!              pattern: Pattern::Polkadots { background: Color::White, foreground: Color::Black },
//!              has_bow: true,
//!              tag: Some(
//!                  GiftTag {
//!                      recipient: "Bob".to_string(),
//!                      sender: "Sally".to_string(),
//!                      message: "Happy Cake Day!".to_string()
//!                  }
//!              )
//!          }
//!      }
//! )
//! ```

use crate::giftbox::GiftBox::*;
use crate::gifttag::GiftTag;
use crate::giftwrap::GiftWrap;
use crate::pattern::Pattern;
use std::fmt::*;

/// A `GiftBox` type for Rust that could contain any type of gift that can be represented as a Rust
/// type.
///
/// For example, the `GiftBox` can contain:
///
/// **A `u8` gift represented as:**
/// ```
/// use giftbox::giftbox::GiftBox;
/// let gift_of_42 = GiftBox::Gifts(42);
/// ```
/// **A nice message in a `String` as:**
/// ```
/// use giftbox::giftbox::GiftBox;
/// let gift_of_string = GiftBox::Gifts(String::from("If anything is worth doing, do it with all
/// your heart. <3"));
/// ```
///
/// # `GiftBox` States
/// `GiftBox` has two possible states:
/// * The `GiftBox` contains `Gifts(T)` where the value of `T` is the gift.
/// * The `GiftBox` is `Empty`.
///
/// # Methods
///
/// ## fill()
/// Fill a `GiftBox` with the [`GiftBox::fill()`] method. Example:
/// ```
/// use giftbox::giftbox::GiftBox;
/// let filled_gift_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
/// ```
/// This will create an instance of a filled gift box called `filled_gift_box`. The
/// `filled_gift_box` is the same as:
/// ```text
/// GiftBox::Gifts(["Toys", "Candy", "Money"])
/// ```
///
/// ## fill_keeping_some()
/// You can fill a box and keep some for yourself and makes the contents of `GiftBox` contain
/// keep the [`std::option::Option`] too, with the [`GiftBox::fill_keeping_some()`] method:
/// ```
/// use giftbox::giftbox::GiftBox;
/// let some_filled_box = GiftBox::fill_keeping_some(Some("Chocolate"));
/// assert_eq!(some_filled_box.open(), Some("Chocolate"));
/// ```
///
/// ## open()
/// You can open a `GiftBox` with [`GiftBox::open()`] to get the contents of the `GiftBox`. Example:
/// ```
/// use giftbox::giftbox::GiftBox;
/// let filled_gift_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
/// let gifts = filled_gift_box.open();
/// assert_eq!(gifts, ["Toys", "Candy", "Money"]);
/// ```
///
/// ### Opening an `Empty` box causes a panic!
/// If you open an empty box, represented as ```GiftBox::Empty``` then the compiler will panic.
/// Example:
/// ```should_panic
/// use giftbox::giftbox::GiftBox;
/// let empty_box = GiftBox::Empty;
/// empty_box.open()
/// // ^^^This will cause a panic at runtime.^^^
/// ```
///
/// You can also create an empty box by filling a box with a `None` from [`std::option::Option`].
/// ```should_panic
/// use giftbox::giftbox::GiftBox;
/// let another_empty_box = GiftBox::fill(None);
/// another_empty_box.open()
/// // ^^^This will also cause a panic at compile time.^^^
/// ```
///
/// # About the Contents of a `GiftBox` and Memory Allocation
/// While a `GiftBox` could contain any type `T` within `GiftBox::Gifts(T)`, it should be remembered
/// that when creating a `GiftBox` Rust will associate another type (type `T` in this case) with the
/// `GiftBox` type to create a combined type. In this case you have to choose one specific `T` value
/// each time you declare a variable that stores a `GiftBox`.
///
/// The moment you create a `GiftBox`, you have to decide what it is a `GiftBox` of, so that the
/// memory for it can be laid out. At the moment that `GiftBox` is created it needs to decide what
/// it is a `GiftBox` "of" the moment it is put into memory. You cannot create an empty box, which
/// you can decide what you want it to be a box of later, as the compiler would not know how to lay
/// out the memory.
///
/// ## Declaring What Type `T` is in `GiftBox::Gifts(T)`
/// The compiler must always know what type `T` is for every variable. If the value is a concrete
/// instance of `GiftBox` then it must know the `T`. Even if an empty `GiftBox` is declared the
/// compiler has to know what type `T` could be in order to correctly assign memory. When declaring
/// a GiftBox that contains something, the compiler is able to automatically infer what type `T` is
/// so you do not have to declare exactly what type `T` is. In the following example, the compiler
/// knows how much memory to allocate for this GiftBox because the type of `T` can be inferred from
/// the integer `42`:
/// ```
/// use giftbox::giftbox::GiftBox;
/// let integer_gift = GiftBox::Gifts(42);
/// ```
/// However, if you are declaring an empty `GiftBox`, you must associate the empty type with the
/// `GiftBox` by adding the type annotation to the empty box. For example, you could create an empty
/// `GiftBox` with a pointer type, such as:
/// ```
/// use giftbox::giftbox::GiftBox;
/// let empty_box: GiftBox<()> = GiftBox::Empty;
/// ```
/// That tells the compiler "ah, so for the empty_box variable, the `T` associated with `GiftBox` in
/// that instance is the empty type". If you wanted to assign a gift value to that variable, you
/// would then only be allowed to put in an empty type instance `empty_box = GiftBox::Gifts(())`.
/// You could not change the type for `empty_box`. For example, the following would not compile:
/// ```compile_fail
/// use giftbox::giftbox::GiftBox;
/// let mut empty_box: GiftBox<()> = GiftBox::Empty;
/// empty_box = GiftBox::Gifts(true);
/// // ^^^This would not compile!^^^
///
/// let mut int_box = GiftBox::Gifts(42);
/// int_box = GiftBox::Gifts("Words");
/// // ^^^This would not compile as well!^^^
/// ```
///
/// The compiler must always know what `T` is for every variable. If the value is a concrete
/// instance of `GiftBox` then it must know the type for `T`.
///
/// # Generic Functions and `GiftBox`
/// If you would like to accept a GiftBox, but this particular function does not care what kind of
/// gift is in the gift box, then you can make the function itself generic. Example:
/// ```
/// use giftbox::giftbox::GiftBox;
/// fn open_gift_or_panic<T>(gift: GiftBox<T>) -> T {
///      return match gift {
///         GiftBox::Gifts(the_gift) => the_gift,
///         GiftBox::Empty => panic!("I thought I was going to get a gift! D: "),
///     }
/// }
/// ```
/// Here, the function works for any `T` because the code works regardless of what the type in the
/// gift is, it only needs to return the type which is whatever type was in the gift box. In the
/// case of the generic function, it passes the responsibility of figuring out what the `T` is on to
/// the caller of the function.
#[derive(Copy, Debug, PartialEq, PartialOrd)]
pub enum GiftBox<T> {
    Gifts(T),
    Empty,
}

impl<T> GiftBox<T> {
    /// The `fill(t: Option<T>)` method accepts an [`std::option::Option`] and returns a `GiftBox`
    /// with either `GiftBox::Gifts(t)` if `Some(t)` was provided or a `GiftBox::Empty` if
    /// `None` is provided.
    ///
    /// # Arguments
    /// * `t: Option<T>` - `t` accepts an [`std::option::Option`]
    ///
    /// # Returns
    /// Returns a `GiftBox<T>`.
    ///
    /// # Example
    /// Filling a `GiftBox` with `Some(T)`:
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// assert_eq!(filled_box, GiftBox::Gifts(["Toys", "Candy", "Money"]));
    /// ```
    /// Filling a `GiftBox` with `None`:
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// let empty_box: GiftBox<()> = GiftBox::fill(None);
    ///  assert_eq!(empty_box, GiftBox::Empty);
    /// ```
    pub fn fill(t: Option<T>) -> GiftBox<T> {
        GiftBox::from(t)
    }

    /// The `fill_keeping_some(t: Option<T>)` method accepts an
    /// [`std::option::Option`] and returns a `GiftBox` with either `GiftBox::Gifts(Some(t))` if
    /// `Some(t)` was provided or a `GiftBox::Empty` if `None` is provided.
    ///
    /// # Arguments
    /// * `t: Option<T>` - `t` accepts an [`std::option::Option`]
    ///
    /// # Returns
    /// Returns a `GiftBox<Option<T>>`.
    ///
    /// # Example
    /// Filling a `GiftBox` with `Some(T)`:
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// let filled_box = GiftBox::fill_keeping_some(Some(["Toys", "Candy", "Money"]));
    /// assert_eq!(filled_box, GiftBox::Gifts(Some(["Toys", "Candy", "Money"])));
    /// ```
    /// Filling a `GiftBox` with `None`:
    /// ```<T>
    /// use giftbox::giftbox::GiftBox;
    /// use std::option::Option;
    /// let empty_box: GiftBox<Option<T>> = GiftBox::fill_keeping_some(None);
    /// assert_eq!(empty_box, GiftBox::Empty);
    /// ```
    pub fn fill_keeping_some(t: Option<T>) -> GiftBox<Option<T>> {
        match t {
            Some(t) => GiftBox::Gifts(Some(t)),
            None => GiftBox::Empty,
        }
    }

    /// The `open()` method takes a GiftBox and returns the contents of that GiftBox.
    ///
    /// # Arguments
    /// * `self` only.
    ///
    /// # Returns
    /// Returns `T` where `T` is the contents of a `GiftBox:Gifts(<T>)`.
    ///
    /// # Panics!
    /// The `open()` method will panic if used on a GiftBox that is empty (`GiftBox::Empty`).
    ///
    /// # Example
    /// ```
    ///  use giftbox::giftbox::GiftBox;
    /// let filled_box = GiftBox::fill(Some(vec![1, 3, 5, 7]));
    ///  assert_eq!(filled_box.open(), vec![1, 3, 5, 7]);
    /// ```
    pub fn open(self) -> T {
        match self {
            Gifts(gift) => gift,
            Empty => panic!("Opened an empty gift box!"),
        }
    }

    /// The `wrap` method takes a `GiftBox` and some `GiftWrap` parameters to return a `GiftBox`
    /// contained within `GiftWrap`.
    ///
    /// # Arguments
    /// * `self`
    /// * `pattern` - Accepts a [`Patterns`] enum type representing the type of pattern of the
    /// `GiftWrap`.
    /// * `has_bow` - Accepts a boolean representing whether or not the `GiftWrap` has a bow.
    /// * `tag` - Accepts an Option containing a `Tag` type struct or `None` representing a
    /// `GiftTag` that may or may not be included.
    ///
    /// # Returns
    /// Returns `GiftWrap<GiftBox<T>>`
    ///
    /// # Example
    /// ```
    /// use giftbox::giftbox::GiftBox;
    /// use giftbox::gifttag::GiftTag;
    /// use giftbox::giftwrap::GiftWrap;
    /// use giftbox::pattern::{ Pattern, Color };
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Pattern::Striped { first_stripe: Color::Blue, second_stripe: Color::Red },
    ///     true,
    ///     Some(tag)
    /// );
    /// assert_eq!(
    ///      wrapped_box,
    ///      {
    ///          GiftWrap {
    ///              contents:{
    ///                  GiftBox::Gifts(["Toys", "Candy", "Money"])
    ///              },
    ///              pattern: Pattern::Striped { first_stripe: Color::Blue, second_stripe: Color::Red },
    ///              has_bow: true,
    ///              tag: Some(
    ///                  GiftTag {
    ///                      recipient: "Bob".to_string(),
    ///                      sender: "Sally".to_string(),
    ///                      message: "Happy Cake Day!".to_string()
    ///                  }
    ///              )
    ///          }
    ///      }
    /// )
    /// ```
    pub fn wrap(
        self,
        pattern: Pattern,
        has_bow: bool,
        tag: Option<GiftTag>,
    ) -> GiftWrap<GiftBox<T>> {
        GiftWrap {
            contents: self,
            pattern,
            has_bow,
            tag,
        }
    }
}

/// Implementation of From trait for GiftBox
impl<T> From<Option<T>> for GiftBox<T> {
    fn from(gift: Option<T>) -> Self {
        match gift {
            Some(t) => GiftBox::Gifts(t),
            None => GiftBox::Empty,
        }
    }
}

/// This `Clone` allows for GiftBox<T> to utilize `#[derive(Copy)]`.
impl<T: Clone> Clone for GiftBox<T> {
    fn clone(&self) -> Self {
        match self {
            Gifts(x) => Gifts(x.clone()),
            Empty => Empty,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Gifts(to), Gifts(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn open_empty_box_panic() {
        let empty_box: GiftBox<()> = GiftBox::Empty;
        empty_box.open();
    }

    #[test]
    fn filling_box() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        assert_eq!(filled_box, GiftBox::Gifts(["Toys", "Candy", "Money"]));
    }

    #[test]
    fn open_filled_box() {
        let filled_box = GiftBox::fill(Some(vec![1, 3, 5, 7]));
        assert_eq!(filled_box.open(), vec![1, 3, 5, 7]);
    }

    #[test]
    fn filling_with_some() {
        let some_filled_box = GiftBox::fill_keeping_some(Some("Chocolate"));
        assert_eq!(some_filled_box.open(), Some("Chocolate"));
    }

    #[test]
    fn filling_with_none() {
        let empty_box: GiftBox<()> = GiftBox::fill(None);
        assert_eq!(empty_box, GiftBox::Empty);
    }
}
