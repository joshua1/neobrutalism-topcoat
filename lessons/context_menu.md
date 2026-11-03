# Context Menu

1. **`&'static str` return types.** `fn as_str(self) -> &'static str` returns a
   string literal. Literals are baked into the program, so they live for its
   whole run: `'static` says the caller can keep the reference as long as it
   likes.

2. **The same method name on different types.** `ContextMenuSide::as_str` and
   `ContextMenuItemVariant::as_str` don't clash, because methods belong to
   their type. `side.as_str()` picks the right one from the value's type.

3. **Arrays of tuples and destructuring in `for`.** `let sides = [(Top, "top"), ...];`
   is a fixed-size array (`[T; 4]`). `for (side, name) in sides` unpacks each
   tuple into two variables, like `for (const [side, name] of sides)`.

4. **Owned `String` vs borrowed `&str`.** A prop typed `&str` borrows text the
   caller keeps alive. `&format!(..)` builds a temporary `String` that is
   dropped at the end of the statement, while the lazy view still needs it,
   so the compiler rejects it. `zone_text` takes `#[into] fine: String`
   instead: the component owns the text, and callers can pass either a literal
   or a `format!(..)`.
