# Sheet

1. **`pub` decides what other modules can see.** `sheet` imports
   `DIALOG_OVERLAY` and `LIGHT_DISMISS_SCRIPT` from `dialog`, which are `pub`.
   Dialog's own `FADE` is private, so `sheet.rs` declares its own `FADE` with
   different timings. Two private items with the same name in different modules
   never clash.

2. **Enums in data.** `const SHEET_SIDES: [(&str, SheetSide); 4]` pairs each
   label with a `SheetSide`. Because `SheetSide` derives `Copy`, the loop can
   pull each value out of the constant array and hand it to `sheet_content`.

3. **`format!` builds a `String`.** `let id = format!("sheet-side-{name}");`
   creates an owned, growable string, like a template literal. `{name}` reads
   the variable directly. `let` works inside `view!` loops, so each iteration
   gets its own `id`.

4. **`String` vs `&str` at call sites.** `target: &id` passes a borrowed
   `&String`, which Rust coerces to `&str` automatically. That coercion does not
   happen through `Into`, so the `#[into]` prop `id` takes `id.as_str()`,
   which is already a `&str`.

5. **Ranges and `_`.** `for _ in 0..10` repeats ten times: `0..10` is a range
   (0 up to, not including, 10) and `_` says "I don't need the number".
