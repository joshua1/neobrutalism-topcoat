# Badge

1. **`///` doc comments and `[`badge`]` links.** `//` is a comment; `///` is
   documentation attached to the next item, shown on hover in your editor.
   ``[`badge_variants`]`` links to another item, and `cargo doc` checks it
   exists. ` ```ignore ` examples are skipped by doc tests, because they only
   make sense inside an app.

2. **Generic types holding a tuple: `Class<(StaticClass, StaticClass)>`.**
   `Class<T>` is generic, a class list holding some `T`. Here `T` is a tuple of
   two static parts (`BASE` and the variant classes). Button's had three. Rust
   tracks the exact shape so it can join the strings without allocating memory.

3. **`#[must_use]`.** The compiler warns if someone calls `badge_variants(..)`
   and ignores the result. Calling it without using the classes is always a
   bug.

4. **`#[allow(dead_code)]`.** Rust warns about unused code. Inside the registry
   nothing uses `BadgeVariant::Neutral`; apps do. The attribute silences the
   warning for one item, so the registry check stays clean.

5. **No `render` prop.** Octane's `render={<a/>}` swaps the element. In Rust,
   reuse the classes instead:
   `<a class=(badge_variants(BadgeVariant::Neutral))>..</a>`.
