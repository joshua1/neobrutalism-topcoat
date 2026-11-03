# Tabs

1. **`Expr<bool>`: a value now or a value later.** `active: Expr<bool>` is
   either a plain `true`/`false` or a runtime expression over a signal.
   `#[into]` lets callers pass `true` directly, and `#[default(false.into())]`
   converts the fallback the same way. Inside, `:hidden=$(!active)` binds the
   attribute so it can update in the browser.

2. **`if` at the node level picks an element.** `tabs_trigger` renders either
   an `<a>` or a `<button>`. Both branches use `child` and `class`, which
   would be a "use of moved value" error if both ran, but Rust knows only one
   branch of an `if` executes.

3. **`&self` vs `&mut self` methods.** `attrs.contains_key("href")` only looks,
   so it borrows `attrs` immutably. `attrs.remove("class")` changes it and
   needs `mut attrs`. The check runs first: once a value is moved out, it is
   gone.

4. **`let` with an inferred, unnamed type.** `let class = class!(TRIGGER, attrs.remove("class"));`
   has a long generic type nobody wants to write. `let` infers it; only
   `const` and function signatures need types spelled out.

5. **`match` in an attribute list.** `match orientation { Horizontal => data-orientation="horizontal", ... }`
   emits a different attribute per variant, with the same exhaustiveness
   checks as a `match` in code.
