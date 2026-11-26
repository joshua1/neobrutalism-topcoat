# Stars

1. **One enum instead of 40 components.** Octane has `Star1` to `Star40`,
   each a component holding one path. Here `Star` is an enum with a variant
   per shape and `star(shape: Star::S14)` draws it. A `match` in
   `Star::path` maps each variant to its path data, and the compiler checks
   that every variant has an arm: add `S41` and forget its path, and the
   build fails.

2. **An associated array constant.** `Star::ALL` is a `[Star; 40]`, an array
   whose length is part of its type. The gallery page loops over it with
   `for shape in Star::ALL`. The enum is `Copy`, so each iteration gets its
   own copy of a variant and nothing is borrowed.

3. **`#[into]` with an `Option` prop.** `color` is an `Option<&str>`, yet
   callers write `color: "red"` instead of `color: Some("red")`. The standard
   library implements `From<T> for Option<T>`, so `"red".into()` is
   `Some("red")`, and `#[into]` makes the component call `.into()` on what it
   is given. `#[default]` still lets callers leave it out (`None`).

4. **`None` leaves an attribute out.** `width=(size)` with `size: None`
   renders no `width` attribute at all, which is what TypeScript's
   `width={size ?? width}` achieves by passing `undefined`.

5. **`unwrap_or` for a fallback.** `color.unwrap_or("currentColor")` is
   `color ?? "currentColor"`: the value inside `Some`, or the fallback for
   `None`.
