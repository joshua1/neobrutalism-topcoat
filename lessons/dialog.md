# Dialog

1. **Custom defaults: `#[default(expr)]`.** `#[default]` uses the type's
   `Default` (`false`, `None`, empty). `#[default(true)] show_close_button`
   picks another value, like `showCloseButton = true` in a TypeScript
   destructuring default. The expression runs only when the caller omits the
   prop.

2. **`#[into]` and the `From` / `Into` traits.** `open: Expr<bool>` takes
   `true`, `false` or a runtime `$(signal.get())`, because `bool` implements
   `Into<Expr<bool>>`. Likewise `id: Option<&str>` with `#[into]` accepts a
   bare `"edit-profile"`, since every `T` converts into `Some(T)`. Conversions
   are explicit traits, not implicit coercion: if no `From` impl exists, it does
   not compile.

3. **Borrow, then move.** `unless_given(&attrs, "data-slot", "dialog")` takes
   `&Attributes`, a *borrow*: it reads the map without taking it. Later,
   `(attrs)` in the markup *moves* the map into the element. The order matters;
   the compiler rejects a borrow after the move, because the value is gone.

4. **`&'static str`.** The helper returns `Option<&'static str>`: a string that
   lives for the whole program, such as a literal or a `const`. Returning a
   borrow of something shorter-lived would be a compile error, so `'static`
   documents that the caller can keep the value as long as it likes.

5. **`pub const` and `\` line continuations.** The inline scripts are
   `pub const OPEN_MODAL_FALLBACK: &str`, so `sheet`, `drawer` and other
   modules can import them. A `\` at the end of a line inside a string literal
   skips the newline and the next line's leading spaces, so long strings can
   wrap without changing their contents.
