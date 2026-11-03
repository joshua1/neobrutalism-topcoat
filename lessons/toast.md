# Toast

1. **A method is a function value.** `.filter_map(ToastVariant::icon)` passes
   the method itself where a closure is expected: `ToastVariant::icon` is a
   function from `ToastVariant` to `Option<(IconData, &str)>`. `filter_map`
   keeps the `Some` values and drops the `None`s, like `.map(..).filter(..)`
   in one step.

2. **`move` closures.** `.filter(move |&v| template || v == variant)` builds a
   lazy iterator that the view runs later, after the function has returned.
   A plain closure would borrow `template` and `variant` from the function,
   and those borrows would end too soon. `move` makes the closure own copies
   of them.

3. **Patterns in closure parameters.** `filter` hands its closure a
   reference, `&ToastVariant`. Writing the parameter as `|&v|` destructures
   it, so `v` is a plain `ToastVariant` (possible because the enum derives
   `Copy`). It is the closure-parameter form of `for &x in ..`.

4. **`Option` of a tuple.** `fn icon(self) -> Option<(IconData, &'static str)>`
   returns "maybe a pair": `None` for the default variant, otherwise the icon
   and its classes together. The `for (glyph, classes) in icons` loop
   destructures each pair.

5. **`Option` attributes with numbers.** `data-timeout=(timeout)` takes an
   `Option<u32>`: `None` leaves the attribute out, `Some(8000)` renders
   `data-timeout="8000"`. The toaster's `data-limit=(limit)` renders its
   `usize` the same way, with no `to_string()`.
