# Popover

1. **Returning an owned `String`.** `popover_anchor(id: &str) -> String`
   borrows the id and returns a new string built with `format!`. The caller owns
   the result and can put it in a `style` attribute; `#[must_use]` warns if they
   build it and forget it.

2. **`then` vs `then_some`.** `anchor.then(|| popover_anchor(target))` takes a
   closure (`|| expr` is `() => expr`) and only calls it when `anchor` is true,
   so no string is built otherwise. `then_some(value)` always computes `value`
   first, which is fine for a constant but wasteful for a `format!`.

3. **Matching a tuple.** `match (side, align)` matches both enums at once:
   `(Bottom, Start) => ..`. With four sides and three alignments the compiler
   checks all twelve combinations are handled.

4. **Importing variants locally.** `use PopoverSide::{Bottom, Left, Right, Top};`
   at the top of `placement` lets its arms write `Bottom` instead of
   `PopoverSide::Bottom`. The import only applies inside that function.
