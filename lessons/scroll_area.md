# Scroll area

1. **`struct`: a named record.** The horizontal example declares
   `struct Artwork { artist: &'static str, art: &'static str }`, an
   interface plus the object shape, in one. You build one with
   `Artwork { artist: "...", art: "..." }` and read fields with
   `artwork.artist`.

2. **Fixed-size arrays: `[Artwork; 3]`.** The length is part of the type, so
   `WORKS` lives in a `const` with no heap allocation. Add a fourth entry and
   the compiler asks you to update the `3`.

3. **`format!` is a template literal.**
   `format!("Photo by {}", artwork.artist)` builds a new `String`, like
   `` `Photo by ${artist}` ``. Use it when you need a computed string, not
   for class lists, where `class!` is the tool.

4. **A component with no children.** `scroll_bar` takes only `orientation` and
   renders a hidden marker. Since it has no `child` parameter, calling it with
   nodes inside is a compile error.
