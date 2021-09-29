//! ./src/main.rs
//! A Rust gift box generics example. Help me `wrap` my head around Rust Generics!

// Import std::fmt for derive macro.
use std::fmt::*;

/// A simple generics enum example that represents a gift box. The gift box could contain any type
/// of gift or could be empty. `#[derive(Debug)]` macro is used to simplify the path to `println!`.
#[derive(Debug)]
enum GiftBox<T> {
    Gifts(T),
    Empty
}

/// In this example an empty gift box is declared and no gift box is declared with gifts. This is
/// doomed to compilation failure because the compiler does not know what type the gifts could be.
fn main() {
    // Declare an empty gift box called `empty_box`.
    let empty_box = GiftBox::Empty;

    // Attempt to `println!` to see what the gift box contains. However, this will not compile! An
    // error will be displayed stating "cannot infer type for type parameter `T` declared on the
    // enum `GiftBox`".
    println!("The gift box contains: {:?}", empty_box);
}

/// If the purpose of the `GiftBox<T>` enum is to allow any sort of type to be within
/// `GiftBox::Gifts`, how do we best accomplish this?
///
/// - Do we have to always hint to the type of `T` if we only declare `GiftBox::Empty`?
/// - Do we have to be explicit on the types that `T` can be?
/// - Is this where turbofish is supposed to be used?!
fn what_do() {
    todo!("Need to figure out how to make this a fully functional generic!");
}