# Progress

1. **`Expr<f64>`: one prop, static or live.** `Expr<T>` is a generic value
   that renders on the server and can also update in the browser. With
   `#[into]`, callers pass a plain `56.0` (converted through
   `From<f64> for Expr<f64>`) or a runtime expression such as
   `$(value.get())`.

2. **`f64` and no implicit number conversion.** Write `value: 56.0`, not
   `56`: an integer literal won't silently become a float. TypeScript has one
   `number` type; Rust has many (`i32`, `usize`, `f64`, ...) and asks you to
   convert between them explicitly.

3. **Moves and `.clone()`.** Passing a value transfers ownership, so after
   `progress_indicator(value: value)` the root could no longer use `value`.
   The root uses it twice, so it passes `value.clone()` to `:aria-valuenow`
   first. A clone of an `Expr` copies the value and its browser code.

4. **Typed closures.** `|e: Event| { .. }` is an arrow function. Inside an
   attribute the compiler can't infer `e`'s type, so it is written out.

5. **`parse()` returns a `Result`.** The slider handler converts the input's
   text to a number with `raw!("Number(${input})", ..)` in the browser. Its
   Rust twin is `input.to_string().parse().unwrap_or(0.0)`. `parse` can fail,
   so it returns a `Result`, and `unwrap_or` supplies the fallback, where JS
   would quietly give `NaN`.
