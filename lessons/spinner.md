# Spinner

1. **`#[default(expr)]` for a custom default.** `label` defaults to
   `String::from("Loading")` instead of the empty string that plain
   `#[default]` would give. The expression only runs when the caller leaves
   `label` out.

2. **Making a `String` from a literal.** A literal like `"Loading"` is a
   `&'static str`. `String::from("Loading")` (or `"Loading".to_owned()`)
   copies it into an owned `String` the component can keep and pass on.

3. **`into_*` methods consume their value.** `LOADER.into_body()` takes the
   `IconData` by value (ownership moves into the method), which Rust's
   naming convention signals with `into_`. `view_box()` only reads it. Each
   use of a `const` such as `LOADER` gets a fresh copy, so both calls work.
