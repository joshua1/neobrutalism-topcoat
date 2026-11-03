# Marquee

1. **Slices: `&[&str]`.** `items: &[&str]` is a borrowed view of a list of
   strings, of any length. Callers pass `&["Item 1", "Item 2"]`, an array
   borrowed as a slice. It's the `string[]` of read-only parameters.

2. **Borrowing lets you loop twice.** `for item in items` walks the slice
   without consuming it, so the second copy can loop over the same `items`.
   Looping over an owned `Vec` instead would *move* it into the first loop and
   leave nothing for the second.

3. **`*item` dereferences.** Iterating a `&[&str]` yields `&&str`, a reference
   to each element. `(*item)` follows that reference to the `&str` itself.
   Most of the time Rust does this for you; here it keeps the rendered type
   simple.
