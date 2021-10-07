# `GiftBox` A fun Rust crate to help Rustlings learn and explore generics.

## Purpose

The purpose of this crate is to:
* Help Rustlings understand Rust generics and how to use them.
* Expand upon related beginner topics to support a Rustling's understanding of generics.
* Provide a fun crate to be used in hands-on self-directed Rustling projects to assist the
learning process.
* Be approachable, supportive, and encourage participation in a GiftBox community.

## Concept

***This crate uses the concept of a
[gift box](https://en.wikipedia.org/wiki/Decorative_box#Gift_box) to help explain Rust
generics.***

Imagine a real-life gift box. You can put any gift inside this gift box, wrap this
gift box with wrapping paper, add a bow, and add a gift tag to show who it is for, who it is
from and a short message. The gift box is generic, so you can reasonably assume that you can use
this gift box to put any gift inside.

Imagine more, if you will, that Rust's compiler is like the gift wrapping station at your local
shopping mall. This imaginary gift wrapping station already has gift boxes for every type of
gift available in the shopping mall. It can also build a new gift box on demand to put inside
other gifts that you bring to them, which may not be already in the shopping mall. This gift
wrapping station is so exceptional that they can pretty much put any gift you bring to them in
a gift box.

Once your gift box is filled with a gift, it can then be gift wrapped or not. If it is wrapped
it can have different patterns available at the gift wrapping station. You can also decide to
add a gift tag or not.

In this way, the gift box is a generic. It is generic enough so that you can use it to wrap a
huge--almost limitless--number of different things inside it. We will use this concept of the
gift box to help describe what generics are in Rust.

### Associating the Concept with Rust Concepts

Sometimes a metaphor can assist in the learning process. Metaphors help abstract difficult
concepts so that the learner can relate these difficult concepts to other concepts that they may
already be accustomed to. It is in this way that this crate uses the gift box metaphor to
describe what a Rust generic is.

In order for the use of metaphors to work best, and to help reduce confusion, let's relate the
metaphors to the Rust concepts we are trying to learn:

* [**rustc** *The Rust Compiler*](https://doc.rust-lang.org/rustc/what-is-rustc.html)

    The Rust compiler, in this example, can be thought of as the gift wrapping station. When
    compiling a program that has generics, the compiler needs to know what the generics are
    being used for in order to allocate the appropriate amount of memory. In this same way, the
    gift wrapping station needs to know what gift (or at least what type of gift) is being
    wrapped so that they know which sized box they should use. If the gift wrapping station uses
    a box that is too big, the box will be inefficient and more difficult to carry than it needs
    to be. If the box is too small, the gift might not even fit in the box at all!

* [**Generics** *Rust Generic Data Types*](https://doc.rust-lang.org/book/ch10-01-syntax.html)

    In Rust, a *generic* is a generalized type or functionality that can be used for different
    types. *"Wait, what?!", you might ask.* Think of generics as the gift box. A gift box is
    normally generic enough that it can be used for a large variety of gifts, as long as the
    gift box is sized correctly. The versatility of the gift box allows it to be useful in a
    large number of contexts. For example, a gift box can be used for many different occasions
    such as a birthday, Christmas, or an anniversary and hold many different types of gifts like
    a book, or toys, or candy, or many different things all at once! Therefore, similar to Rust
    generics, the gift box is a real-life "generic".

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
giftbox = "0.1.1"
```

Here's a simple example that creates a newly filled `GiftBox` that is wrapped, has a bow, and has a gift tag:

```rust
use giftbox::giftbox::GiftBox;
use giftbox::gifttag::GiftTag;
use giftbox::giftwrap::GiftWrap;
use giftbox::patterns::Patterns;

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
             contents:{
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
);
```

## Participation and Contributions
Any and all participation is welcome with the project. It is meant to be fun and useful for all programmers who are new
to Rust. If you have found this to be of benefit in your Rust learning journey or would like to enhance this project 
with your expertise, please do!

Please consider opening an issue before making pull requests.

Areas that this project could benefit most:
- [ ] Adding more useful beginner examples, features and functionality throughout the project.
- [ ] Enhancing and writing more documentation that is beginner-friendly.
- [ ] Creating other crates that utilize `GiftBox` so that the value of generics is better understood in the Rust 
ecosystem.
- [ ] Finding and reporting bugs and issues.

*Most of all, this project would benefit most by sharing it with your friends and building a community!*

## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).