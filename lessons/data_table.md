# Data Table

1. **A generic struct: `Column<T>`.** Like TanStack's `ColumnDef<TData>`,
   `Column<T>` works for any row type `T`, and `impl<T> Column<T>` gives
   every `Column<Payment>`, `Column<User>`, ... the same methods.
   `DataTable<'a, T>` takes a lifetime as well: it holds `&'a T`, borrowed
   rows that must outlive the table.

2. **Function pointers: `fn(&T) -> String`.** A field can hold a function.
   A closure that captures nothing converts to a plain `fn` pointer, which
   is why the columns fit in a `const` array. Call a field that holds a
   function with parentheses around it, `(column.value)(row)`, so Rust does
   not look for a method named `value`. The closures spell out
   `|payment: &Payment|` because the builder calls after `Column::new` keep
   Rust from inferring `T` in time.

3. **Sorting with `Ordering`.** `sort_by` takes a comparator that returns an
   `Ordering` (`Less`, `Equal` or `Greater`), not a negative or positive
   number. `a.cmp(&b)` produces one, and `.reverse()` flips it for
   descending order. The sort is stable, so equal rows keep their order.
   (`f64` has no `cmp`, since `NaN` breaks ordering; use `a.total_cmp(&b)`.)

4. **Iterators for the row model.** Filtering, sorting and paging are one
   pipeline: `.iter().filter(..).collect()` into a `Vec`, `sort_by`, then
   `.into_iter().skip(start).take(size).collect()` for the page.
   `len.div_ceil(size)` rounds the page count up, and
   `page.clamp(1, count)` keeps the page in range.

5. **Struct update syntax.** `Self { page, ..self.clone() }` builds a copy
   with one field changed, like `{ ...state, page }` in TypeScript. The
   rest of the fields come from the value after `..`.

6. **`let ... else` and `let` chains.** `let Ok(query) = .. else { return
   .. };` binds on success and must leave the function otherwise.
   `if let Some(a) = x && let Some(b) = y { .. }` (Rust 2024) tests several
   patterns in one `if`, instead of nesting them.
