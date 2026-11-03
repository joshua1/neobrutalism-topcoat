# Dropdown Menu

1. **Matching on a tuple: `match (side, align)`.** Wrapping two values in
   parentheses makes one tuple, and each arm matches both at once:
   `(Bottom, Start) => ...`. The compiler checks all 12 combinations are
   covered, so adding a fifth side fails to build until every pairing is
   placed.

2. **`use` inside a function.** `use DropdownMenuSide::{Bottom, Left, Right, Top};`
   at the top of `position` lets the arms say `Bottom` instead of
   `DropdownMenuSide::Bottom`. The import only exists inside that function.

3. **`#[default(true)]` for a non-zero default.** `#[default]` uses the type's
   `Default` (`false` for `bool`). `#[default(true)]` picks another value, so
   `close_on_click` is on unless a caller writes `close_on_click: false`.

4. **`bool::then_some` turns a flag into an `Option`.**
   `close_on_click.then_some(CLOSE_JS)` is `Some(CLOSE_JS)` when true and
   `None` when false, and an attribute given `None` is left out of the HTML.
   It replaces `cond ? value : undefined`.

5. **Long string literals with `\`.** A backslash at the end of a line inside
   `"..."` joins the next line and drops its leading spaces, so the inline
   scripts and class lists stay readable without adding stray whitespace.
