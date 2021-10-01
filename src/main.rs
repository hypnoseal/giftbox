//! ./src/main.rs
//! A Rust gift box generics example. Help me `wrap` my head around Rust Generics!

// Import std::fmt for derive macro.
use std::fmt::*;
use crate::GiftBox::*;

/// A simple generics enum example that represents a gift box. The gift box could contain any type
/// of gift or could be empty. `#[derive(Debug)]` macro is used to simplify the path to `println!`.
#[derive(Copy, Debug, PartialEq)]
pub enum GiftBox<T> {
    Gifts(T),
    Empty
}

impl<T> GiftBox<T> {
    pub fn fill(t: Option<T>) -> GiftBox<T> {
        match t {
            Some(t) => GiftBox::Gifts(t),
            None => GiftBox::Empty
        }
    }

    pub fn open(self) -> T {
        match self {
            Gifts(gift) => gift,
            Empty => panic!("Opened an empty gift box!"),
        }
    }

    pub fn wrap(self, pattern: Patterns, has_bow: bool, tag: Option<GiftTag>) -> GiftWrap<GiftBox<T>> {
        GiftWrap {
            contents: self,
            pattern,
            has_bow,
            tag
        }
    }
}

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

#[derive(Debug)]
pub struct GiftWrap<T> {
    contents: T,
    pattern: Patterns,
    has_bow: bool,
    tag: Option<GiftTag>
}

impl<T> GiftWrap<T> {
    pub fn unwrap(self) -> T {
        self.contents
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GiftTag {
    recipient: String,
    sender: String,
    message: String
}

/// In this example an empty gift box is declared and no gift box is declared with gifts. This is
/// doomed to compilation failure because the compiler does not know what type the gifts could be.
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
    fn test_filling_box() {
        let filled_box = GiftBox::fill(Some(["Toys", "Candy", "Money"]));
        assert_eq!(filled_box, GiftBox::Gifts(["Toys", "Candy", "Money"]));
    }

    #[test]
    fn test_open_filled_box() {
        let filled_box = GiftBox::fill(Some(vec![1, 3, 5, 7]));
        assert_eq!(filled_box.open(), vec![1, 3, 5, 7]);
    }
}