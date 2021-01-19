# Digressions

These are digressions from the standard instructions of the book, or digressions from what might be ideomatic Rust.  Here I've documented why the digression seemed appropriate.

1. The book says to make a reader class and implement peek and next.  I realized that there's basically nothing in this reader that is novel or different from the vanilla vec structure.  So instead of creating a new struct, I'm going to rely on the existing Iterator functionality that is built into vec.
2. My first instinct was to pass an iterator down through the reader functions.  Unfortunately the type signatures become really verbose, and there are ownership issues.  Instead of wrestling with the Rust type system and borrow checker I decided to just pass a separate `usize` to track the position of the iterator.  This was far simpler to reason about.
3. Instead of implementing `pr_str` I implemented the Display trait on RadNode.
