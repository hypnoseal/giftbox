# Please Help Me Better Understand Rust Generics!

This repository contains an example of a non-working generic in Rust called `GiftBox`. Within the example, a 
`GiftBox<T>` enum is defined with two options `Gifts(T)` and `Empty`. The `GiftBox` is meant to be able to contain any 
type of `Gifts(T)`. However, in this example, `T` is never actually declared and no type is ever specified.

In this example, only an `Empty` gift box is declared with `GiftBox::Empty`. The example purposefully does not declare a
gift box with any contents. This is to make the compiler complain about not being able to infer the type `T`.

It is my goal to be able to create a crate that utilizes generics so that other coders can utilize the generic without 
my library dictating the types to be used. It should be able to work so that the other coder can declare only an empty 
`GiftBox` without having to hint at what types `T` could be or create a fake `GiftBox` that declares a type but isn't 
used. It is my hope that `T` could be any type!

Some questions I have are:
- What are the idiomatic ways to define a generic enum (or other types of generics) so that the compiler won't complain?
- How does `Option<T>` accomplish this?
- Do we have to always hint to the type of `T` if we only declare `GiftBox::Empty`?
- Do we have to be explicit on the types that `T` can be?
- Is this where turbofish is supposed to be used?!

_I appreciate any and all help you can provide towards helping me better understand how to correctly implement generics.
Please feel free make pull requests or submit issues to respond._