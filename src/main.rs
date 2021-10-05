//! ./src/main.rs
//! A Rust gift box generics example. Let's `wrap` our heads around Rust Generics!
//!
//!               .__.      .==========.
//!             .(\\//).  .-[  GiftBox ]
//!            .(\\()//)./  '=========='
//!        .----(\)\/(/)----.
//!        |     ///\\\     |
//!        |    ///||\\\    |
//!        |   //`||||`\\   |
//!        |      ||||      |
//!        |      ||||      |
//!        '------====------'
//!

// Import std::fmt for derive macro.
use std::fmt::*;
use crate::GiftBox::*;

/// A `GiftBox` type for Rust that could contain any type of gift that can be represented as a Rust
/// type.
///
/// For example, the `GiftBox` can contain:
///
/// * A `u8` gift represented as
/// ```
/// GiftBox::Gifts(42)
/// ```
/// * A nice message in a `String` as
/// ```
/// GiftBox::(String::From("If anything is worth doing, do it
/// with all your heart. <3"))
/// ```
///
/// #`GiftBox` States
/// `GiftBox` has two possible states:
/// * The `GiftBox` contains `Gifts(T)` where the value of `T` is the gift.
/// * The `GiftBox` is `Empty`.
///
/// #Methods
///
/// ##fill()
/// Fill a `GiftBox` with the [`Gifts::fill()`] method. Example:
/// ```
/// let filled_gift_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
/// ```
/// This will create an instance of a filled gift box called `filled_gift_box`. The
/// `filled_gift_box` is the same as:
/// ```
/// GiftBox::Gifts(["Toys", "Candy", "Money"])
/// ```
///
/// ##fill_keeping_some()
/// You can fill a box and keep some for yourself and makes the contents of `GiftBox` contain
/// keep the [`std::option::Option`] too,
/// with the [`Gifts::fill_keeping_some()`] method:
/// ```
/// let gift_box = Gift::fill_with_some(Some("Chocolate"));
/// assert_eq!(some_filled_box.open(), Some("Chocolate"));
/// ```
///
/// ##open()
/// You can open a `GiftBox` with [`GiftBox::open()`] to get the contents of the `GiftBox`. Example:
/// ```
/// let filled_gift_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
/// let gifts = filled_gift_box.open();
/// assert_eq!(gifts, ["Toys", "Candy", "Money"]);
/// ```
///
/// ###Opening an `Empty` box causes a panic!
/// If you open an empty box, represented as ```GiftBox::Empty``` then the compiler will panic.
/// Example:
///
/// ```
/// let empty_box = GiftBox::Empty;
/// empty_box.open()
/// // ^^^This will cause a panic at compile time.^^^
/// ```
///
/// You can also create an empty box by filling a box with a `None` from [`std::option::Option`].
/// ```
/// let another_empty_box = GiftBox::fill(None);
/// empty_box.open()
/// // ^^^This will also cause a panic at compile time.^^^
/// ```
///
/// #About the Contents of a `GiftBox` and Memory Allocation
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
/// ##Declaring What Type `T` is in `GiftBox::Gifts(T)`
/// The compiler must always know what type `T` is for every variable. If the value is a concrete
/// instance of `GiftBox` then it must know the `T`. Even if an empty `GiftBox` is declared the
/// compiler has to know what type `T` could be in order to correctly assign memory. When declaring
/// a GiftBox that contains something, the compiler is able to automatically infer what type `T` is
/// so you do not have to declare exactly what type `T` is. In the following example, the compiler
/// knows how much memory to allocate for this GiftBox because the type of `T` can be inferred from
/// the integer `42`:
/// ```
/// let integer_gift = GiftBox::Gifts(42)
/// ```
/// However, if you are declaring an empty `GiftBox`, you must associate the empty type with the
/// `GiftBox` by adding the type annotation to the empty box. For example, you could create an empty
/// `GiftBox` with a pointer type, such as:
/// ```
/// let empty_box: GiftBox<()> = GiftBox::Empty;
/// ```
/// That tells the compiler "ah, so for the empty_box variable, the `T` associated with `GiftBox` in
/// that instance is the empty type". If you wanted to assign a gift value to that variable, you
/// would then only be allowed to put in an empty type instance `empty_box = GiftBox::Gifts(())`.
/// You could not change the type for `empty_box`. For example, the following would not compile:
/// ```
/// let mut empty_box: GiftBox<()> = GiftBox::Empty;
/// empty_box = GiftBox::Gifts(true);
/// // ^^^This would not compile!^^^
///
/// let mut int_box = GiftBox::Gifts(42);
/// num_box = GiftBox::Gifts("Words");
/// // ^^^This would not compile as well!^^^
/// ```
/// The compiler must always know what `T` is for every variable. If the value is a concrete
/// instance of `GiftBox` then it must know the type for `T`.
///
/// #Generic Functions and `GiftBox`
/// If you would like to accept a GiftBox, but this particular function does not care what kind of
/// gift is in the gift box, then you can make the function itself generic. Example:
/// ```
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
///
#[derive(Copy, Debug, PartialEq, PartialOrd)]
pub enum GiftBox<T> {
    Gifts(T),
    Empty
}

impl<T> GiftBox<T> {
    /// The `fill(t: Option<T>)` method accepts an [`std::option::Option`] and returns a `GiftBox`
    /// with either `GiftBox::Gifts(t)` if `Some(t)` was provided or a `GiftBox::Empty` if
    /// `None` is provided.
    ///
    /// #Arguments
    /// * `t: Option<T>` - `t` accepts an [`std::option::Option`]
    ///
    /// #Returns
    /// Returns a `GiftBox<T>`.
    ///
    /// #Example
    /// Filling a `GiftBox` with `Some(T)`:
    /// ```
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// assert_eq!(filled_box, GiftBox::Gifts(["Toys", "Candy", "Money"]));
    /// ```
    /// Filling a `GiftBox` with `None`:
    /// ```
    /// let empty_box: GiftBox<()> = GiftBox::fill(None);
    ///  assert_eq!(empty_box, GiftBox::Empty);
    /// ```
    pub fn fill(t: Option<T>) -> GiftBox<T> {
        match t {
            Some(t) => GiftBox::Gifts(t),
            None => GiftBox::Empty,
        }
    }

    /// The `fill_keeping_some(t: Option<T>)` method accepts an
    /// [`std::option::Option`] and returns a `GiftBox` with either `GiftBox::Gifts(Some(t))` if
    /// `Some(t)` was provided or a `GiftBox::Empty` if `None` is provided.
    ///
    /// #Arguments
    /// * `t: Option<T>` - `t` accepts an [`std::option::Option`]
    ///
    /// #Returns
    /// Returns a `GiftBox<Option<T>>`.
    ///
    /// #Example
    /// Filling a `GiftBox` with `Some(T)`:
    /// ```
    /// let filled_box = GiftBox::fill_keeping_some(Some(["Toys", "Candy", "Money"]));
    /// assert_eq!(filled_box, GiftBox::Gifts(Some(["Toys", "Candy", "Money"])));
    /// ```
    /// Filling a `GiftBox` with `None`:
    /// ```
    /// let empty_box: GiftBox<()> = GiftBox::fill_keeping_some(None);
    ///  assert_eq!(empty_box, GiftBox::Empty);
    /// ```
    pub fn fill_keeping_some(t: Option<T>) -> GiftBox<Option<T>>{
        match t {
            Some(t) => GiftBox::Gifts(Some(t)),
            None => GiftBox::Empty,
        }
    }

    /// The `open()` method takes a GiftBox and returns the contents of that GiftBox.
    ///
    /// #Arguments
    /// * `self` only.
    ///
    /// #Returns
    /// Returns `T` where `T` is the contents of a `GiftBox:Gifts(<T>)`.
    ///
    /// #Panics!
    /// The `open()` method will panic if used on a GiftBox that is empty (`GiftBox::Empty`).
    ///
    /// #Example
    /// ```
    ///  let filled_box = GiftBox::fill(Some(vec![1, 3, 5, 7]));
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
    /// #Arguments
    /// * `self`
    /// * `pattern` - Accepts a [`Patterns`] enum type representing the type of pattern of the
    /// `GiftWrap`.
    /// * `has_bow` - Accepts a boolean representing whether or not the `GiftWrap` has a bow.
    /// * `tag` - Accepts an Option containing a `Tag` type struct or `None` representing a
    /// `GiftTag` that may or may not be included.
    ///
    /// #Returns
    /// Returns `GiftWrap<GiftBox<T>>`
    ///
    /// #Example
    /// ```
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Patterns::Polkadots,
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
    ///              pattern: Patterns::Polkadots,
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
    pub fn wrap(self, pattern: Patterns, has_bow: bool, tag: Option<GiftTag>) ->
        GiftWrap<GiftBox<T>>
    {
        GiftWrap {
            contents: self,
            pattern,
            has_bow,
            tag
        }
    }
}

/// This `Clone` allows for GiftBox<T> to utilize #[derive(Copy)].
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
/// #Methods
/// ##unwrap()
/// Unwrap `GiftWrap` to get its contents with the [`GiftWrap::unwrap()`] method. Example:
/// ```
/// let wrapped_string_gift = GiftWrap {
///     contents: GiftBox::Gifts("String of words".to_string()),
///     pattern: Patterns::Sparkles,
///     has_bow: true,
///     tag: None
/// }
/// let unwrapped_string_gift = wrapped_string_gift.unwrap();
/// assert_eq!(unwrapped_string_gift, GiftBox::Gifts("String of words".to_string()));
/// ```
///
/// ##read_tag()
/// "Read" the `GiftTag` of `GiftWrap`. More specifically, get a String of the `GiftTags` contents
/// with the [`GiftWrap::read_tag()`] Example:
/// ```
/// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
/// let tag = GiftTag::write(
///     "Bob".to_string(),
///     "Sally".to_string(),
///     "Happy Cake Day!".to_string()
/// );
/// let wrapped_box = filled_box.wrap(
///     Patterns::Polkadots,
///     true,
///     Some(tag)
/// );
/// assert_eq!(
///     wrapped_box.read_tag(),
///     "
///     To: Bob,\
///     From: Sally,\
///     Message: Happy Cake Day!
///     "
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct GiftWrap<T> {
    contents: T,
    pattern: Patterns,
    has_bow: bool,
    tag: Option<GiftTag>
}

impl<T> GiftWrap<T> {
    /// The `unwrap()` method takes the `GiftWrap` and unwraps it to reveal its contents.
    ///
    /// #Arguments
    /// * `self` only.
    ///
    /// #Returns
    /// Returns `T` where `T` is the contents of `Giftwrap.contents`.
    ///
    /// #Example
    /// ```
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Patterns::Polkadots,
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
    /// #Arguments
    /// * `self` only.
    ///
    /// #Returns
    /// Returns a String returned from [`GiftTag::read()`] if there is `Some(tag)`. Otherwise, if
    /// there is `None` it returns a default String.
    ///
    /// #Example
    /// ```
    /// let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
    /// let tag = GiftTag::write(
    ///     "Bob".to_string(),
    ///     "Sally".to_string(),
    ///     "Happy Cake Day!".to_string()
    /// );
    /// let wrapped_box = filled_box.wrap(
    ///     Patterns::Polkadots,
    ///     true,
    ///     Some(tag)
    /// );
    /// assert_eq!(
    ///     wrapped_box.read_tag(),
    ///     "
    ///     To: Bob,\
    ///     From: Sally,\
    ///     Message: Happy Cake Day!
    ///     "
    /// );
    /// ```
    pub fn read_tag(self) -> String {
        match self.tag {
            Some(tag) => tag.read(),
            None => "There was no tag to read.".to_string(),
        }
    }
}

/// The `Patterns` enum represents various different Patterns that could be used by [`GiftWrap`].
/// Though it is used by the `GiftWrap`, the `Patterns` enum can be used anywhere in Rust.
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
    Cloth
}

/// The `GiftTag` struct represents a gift tag that could be included with a gift's wrapping. It is
/// used by the `GiftWrap` struct to include a recipient, a sender, and a message. Though it is
/// utilized by `GiftWrap`, this struct can be used anywhere in Rust.
///
/// `GiftTag` has the following parameters:
/// * `recipient` which represents the recipient of the gift as a String.
/// * `sender` which represents the sender of the gift as a String.
/// * `message` which can be a message to be included with the gift as a String.
///
/// #Methods
/// ##write(recipient, sender, message)
/// You can write a new `GiftTag` with the [`GiftTag::write()`] method. Example:
/// ```
/// let tag = GiftTag::write(
///      "Bob".to_string(),
///      "Sally".to_string(),
///      "Happy Cake Day!".to_string()
/// );
/// ```
///
/// ##read()
/// You can read a `GiftTag` with the [`GiftTag::read()`] method. Example:
/// ```
/// assert_eq!(tag.read,
/// "
/// To: Bob,\
/// From: Sally,\
/// Message: Happy Cake Day!
/// "
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct GiftTag {
    recipient: String,
    sender: String,
    message: String
}

impl GiftTag {
    /// The `write` method accepts three arguments as Strings (a recipient, a sender, and a message)
    /// and returns a `GiftTag`.
    ///
    /// #Arguments
    /// * `recipient` - Accepts a String that represents a gift's recipient (the person receiving the
    /// gift).
    /// * `sender` - Accepts a String that represents a gift's sender (the person who sent the gift).
    /// * `message` - Accepts a string that represents a message to the recipient from the sender to
    /// be included with the gift.
    ///
    /// #Returns
    /// Returns a `GiftTag`.
    ///
    /// #Example
    /// ```
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
            message
        }
    }

    /// The `read()` method takes a `GiftTag` as `self` and returns a formatted String representing
    /// the contents of the `GiftTag`.
    ///
    /// #Arguments
    /// * `self` only.
    ///
    /// #Returns
    /// Returns a pre-formatted String.
    ///
    /// #Example
    /// ```
    /// assert_eq!(tag.read,
    /// "
    /// To: Bob,\
    /// From: Sally,\
    /// Message: Happy Cake Day!
    /// "
    /// );
    /// ```
    pub fn read(self) -> String {
        format!(
            "
            To: {},\
            From: {},\
            Message: {}
            ",
            self.recipient,
            self.sender,
            self.message
        )
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
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

    #[test]
    fn wrap_gift_box_with_tag() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string()
        );
        let wrapped_box = filled_box.wrap(
            Patterns::Polkadots,
            true,
            Some(tag)
        );
        assert_eq!(
            wrapped_box,
            {
                GiftWrap {
                    contents: {
                        GiftBox::Gifts(["Toys", "Candy", "Money"])
                    },
                    pattern: Patterns::Polkadots,
                    has_bow: true,
                    tag: Some(
                        GiftTag {
                            recipient: "Bob".to_string(),
                            sender: "Sally".to_string(),
                            message: "Happy Cake Day!".to_string()
                        }
                    )
                }
            }
        )
    }

    #[test]
    fn wrap_and_unwrap_gift_box() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string()
        );
        let wrapped_box = filled_box.wrap(
            Patterns::Polkadots,
            true,
            Some(tag)
        );
        let unwrapped_box = wrapped_box.unwrap();
        assert_eq!(unwrapped_box, filled_box);
    }

    #[test]
    fn wrap_and_open_gift_box_with_tag() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string()
        );
        let wrapped_box = filled_box.wrap(
            Patterns::Polkadots,
            true,
            Some(tag)
        );
        assert_eq!(wrapped_box.unwrap().open(), ["Toys", "Candy", "Money"]);
    }

    #[test]
    fn write_and_read_tag() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        let tag = GiftTag::write(
            "Bob".to_string(),
            "Sally".to_string(),
            "Happy Cake Day!".to_string()
        );
        let wrapped_box = filled_box.wrap(
            Patterns::Polkadots,
            true,
            Some(tag)
        );
        assert_eq!(
            wrapped_box.read_tag(),
            "
            To: Bob,\
            From: Sally,\
            Message: Happy Cake Day!
            "
        );
    }

    #[test]
    fn attempt_to_read_none_tag() {
        let wrapped_nothing = GiftWrap {
            contents: (),
            pattern: Patterns::KraftPaper,
            has_bow: false,
            tag: None
        };
        assert_eq!(wrapped_nothing.read_tag(), "There was no tag to read.");
    }
}