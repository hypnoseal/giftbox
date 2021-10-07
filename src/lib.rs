#![doc(html_logo_url = "https://raw.githubusercontent.com/hypnoseal/giftbox/master/docs/logo/GiftBox_Logo.svg")]
//! A fun Rust crate called `giftbox` to help Rustlings learn and explore generics. **Let's wrap
//! our heads around [Rust Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)!**
//!
//! ```text
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
//! ```
//!
//! # Purpose
//!
//! The purpose of this crate is to:
//! * Help Rustlings understand Rust generics and how to use them.
//! * Expand upon related beginner topics to support a Rustling's understanding of generics.
//! * Provide a fun crate to be used in hands-on self-directed Rustling projects to assist the
//! learning process.
//! * Be approachable, supportive, and encourage participation in a GiftBox community.
//!
//! # Concept
//!
//! ***This crate uses the concept of a
//! [gift box](https://en.wikipedia.org/wiki/Decorative_box#Gift_box) to help explain Rust
//! generics.***
//!
//! Imagine a real-life gift box. You can put any gift inside this gift box, wrap this
//! gift box with wrapping paper, add a bow, and add a gift tag to show who it is for, who it is
//! from and a short message. The gift box is generic, so you can reasonably assume that you can use
//! this gift box to put any gift inside.
//!
//! Imagine more, if you will, that Rust's compiler is like the gift wrapping station at your local
//! shopping mall. This imaginary gift wrapping station already has gift boxes for every type of
//! gift available in the shopping mall. It can also build a new gift box on demand to put inside
//! other gifts that you bring to them, which may not be already in the shopping mall. This gift
//! wrapping station is so exceptional that they can pretty much put any gift you bring to them in
//! a gift box.
//!
//! Once your gift box is filled with a gift, it can then be gift wrapped or not. If it is wrapped
//! it can have different patterns available at the gift wrapping station. You can also decide to
//! add a gift tag or not.
//!
//! In this way, the gift box is a generic. It is generic enough so that you can use it to wrap a
//! huge--almost limitless--number of different things inside it. We will use this concept of the
//! gift box to help describe what generics are in Rust.
//!
//! ## Associating the Concept with Rust Concepts
//!
//! Sometimes a metaphor can assist in the learning process. Metaphors help abstract difficult
//! concepts so that the learner can relate these difficult concepts to other concepts that they may
//! already be accustomed to. It is in this way that this crate uses the gift box metaphor to
//! describe what a Rust generic is.
//!
//! In order for the use of metaphors to work best, and to help reduce confusion, let's relate the
//! metaphors to the Rust concepts we are trying to learn:
//!
//! * [**rustc** *The Rust Compiler*](https://doc.rust-lang.org/rustc/what-is-rustc.html)
//!
//!     The Rust compiler, in this example, can be thought of as the gift wrapping station. When
//!     compiling a program that has generics, the compiler needs to know what the generics are
//!     being used for in order to allocate the appropriate amount of memory. In this same way, the
//!     gift wrapping station needs to know what gift (or at least what type of gift) is being
//!     wrapped so that they know which sized box they should use. If the gift wrapping station uses
//!     a box that is too big, the box will be inefficient and more difficult to carry than it needs
//!     to be. If the box is too small, the gift might not even fit in the box at all!
//!
//! * [**Generics** *Rust Generic Data Types*](https://doc.rust-lang.org/book/ch10-01-syntax.html)
//!
//!     In Rust, a *generic* is a generalized type or functionality that can be used for different
//!     types. *"Wait, what?!", you might ask.* Think of generics as the gift box. A gift box is
//!     normally generic enough that it can be used for a large variety of gifts, as long as the
//!     gift box is sized correctly. The versatility of the gift box allows it to be useful in a
//!     large number of contexts. For example, a gift box can be used for many different occasions
//!     such as a birthday, Christmas, or an anniversary and hold many different types of gifts like
//!     a book, or toys, or candy, or many different things all at once! Therefore, similar to Rust
//!     generics, the gift box is a real-life "generic".
//!
//! # How to Use this Crate and Documentation
//!
//! **Though this crate is made for new Rustlings, this crate assumes the following:**
//! * **You have *some* prior programming experience.**
//! * **You have read [*The Rust Book*](https://doc.rust-lang.org/book/) at least once.**
//! * **You already have a preferred IDE set-up and you are ready to code in Rust.**
//! * **You are willing to have fun learning generics!**
//!
//! This crate is meant to be used in any Rust project and the documentation is written in a
//! tutorial style to aid new Rustlings in how to make use of generics. It is recommended that, if
//! you are very new to Rust, that you follow along with the tutorial articles (links forthcoming).
//!
//! If you are already comfortable using Rust crates and just want to play with these generics, you
//! can import this crate by adding `giftbox` to your `Cargo.toml` dependencies. Examples are
//! provided below to assist in using the crate.
//!
//! The documentation is verbose to help Rustlings in understanding how to use the crate. Concepts
//! are visited and described when required. Throughout the documentation examples are provided on
//! how to make use of the various modules.
//!
//! # Structure of Crate
//!
//! The `giftbox` crate is composed of four modules:
//!
//! 1. **[`giftbox`]**
//!
//!     Contains the [`giftbox::GiftBox`] enum which is the base instructional type for this crate.
//!
//! 2. **[`giftwrap`]**
//!
//!     Contains the [`giftwrap::GiftWrap`] struct meant to illustrate another use of generics and
//!     how generics can be used together.
//!
//! 3. **[`patterns`]**
//!
//!     Contains the [`patterns::Patterns`] enum meant to be used within the [`giftwrap::GiftWrap`]
//!     struct.
//!
//! 4. **[`gifttag`]**
//!
//!     Contains the [`gifttag::GiftTag`] struct also meant to be used within the
//!     [`giftwrap::GiftWrap`] struct.
//!
//! # Examples
//!
//! The following are some examples to help new Rustlings make use of this crate. Examples can be
//! copy-pasted and used. ***Be sure to add `giftbox` to your dependencies in `Cargo.toml`!***
//!
//! ## Import the crate and fill a gift box
//! ```
//! use giftbox::giftbox::GiftBox;
//! use giftbox::gifttag::GiftTag;
//! use giftbox::giftwrap::GiftWrap;
//! use giftbox::patterns::Patterns;
//! let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
//! let tag = GiftTag::write(
//!     "Bob".to_string(),
//!     "Sally".to_string(),
//!     "Happy Cake Day!".to_string()
//! );
//! let wrapped_box = filled_box.wrap(
//!     Patterns::Polkadots,
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
//!              pattern: Patterns::Polkadots,
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

pub mod giftbox;
pub mod gifttag;
pub mod giftwrap;
pub mod patterns;

#[cfg(test)]
mod tests {}
