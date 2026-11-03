# Pagination

1. **Conditional entries in `class!`.** `INACTIVE if !active` includes the
   classes only when the condition holds. `!` negates a `bool`, as in
   TypeScript, but Rust has no truthiness: the condition must be a real
   `bool`, never a number or an `Option`.

2. **Ranges: `1..=5` and `0..100`.** `a..b` counts from `a` up to but not
   including `b`; `a..=b` includes `b`. `for page in 1..=5` replaces a
   `for (let i = 1; i <= 5; i++)` loop.

3. **Numbers render directly.** `(page)` in a view prints the integer, and
   `active: page == 2` passes the comparison's `bool` straight to the prop.
   No `String(page)` conversion is needed.
