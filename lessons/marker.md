# Marker

1. **`Option<T>` instead of `undefined`.** `classes()` returns
   `Option<StaticClass>`: `Some(class!(..))` for the separator and border
   variants, `None` for the plain default. Rust has no `null`. A value that
   might be missing says so in its type, and you can't use it without handling
   `None`.

2. **`class!` skips `None`.** An absent entry adds no text and no stray space,
   so the default variant needs no special case.

3. **Types record the shape.** Because one entry is optional, the helper
   returns `Class<(StaticClass, Option<StaticClass>)>`. If you change
   `classes()`, the compiler points at every signature that has to follow.

4. **The `!` means macro.** `class!`, `view!` and `attributes!` aren't
   functions. They run at compile time and expand into ordinary Rust. That's
   how `view!` can reject a mistyped closing tag before the app runs.
