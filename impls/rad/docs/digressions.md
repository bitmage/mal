# Digressions

These are digressions from the standard instructions of the book.  Here I've documented why the digression seemed appropriate.

1. The book says to make a reader class and implement peek and next.  I realized that there's basically nothing in this reader that is novel or different from the vanilla vec structure.  So instead of creating a new struct, I'm going to rely on the existing Iterator functionality that is built into vec.
