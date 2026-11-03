# Textarea

1. **`#[default]` uses the `Default` trait.** A trait is a set of methods a
   type promises to have, like a TypeScript interface. `Child` implements
   `Default` as "no children", so `textarea()` with no text inside works.
   `#[default]` calls `Default::default()` for whatever the parameter's type
   is.

2. **String literals are `&'static str`.** `"Type your message here."` is
   baked into the program binary and lives for its whole run, which is what
   `'static` means. Passing one costs nothing: no copy, no allocation.

3. **Enumerated attributes take strings.** `aria-invalid="true"` is written
   as a string because ARIA gives `"false"` its own meaning. A Rust `bool`
   attribute, like `disabled=(is_disabled)`, is only present or absent.
