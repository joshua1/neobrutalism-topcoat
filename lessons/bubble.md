# Bubble

1. **`#[default(expr)]`.** `bubble` defaults `align` to
   `BubbleAlign::Start`, and `bubble_reactions` defaults it to
   `BubbleAlign::End`. `#[default(...)]` takes any expression and runs it only
   when the caller leaves the prop out.

2. **Not every enum derives `Default`.** `BubbleAlign` has no `#[default]`
   variant, because there's no single right answer. That's on purpose: if a
   new prop forgets to pick one, `#[default]` alone fails to compile, instead
   of quietly choosing a side.

3. **One enum, several methods.** `BubbleAlign` has `as_str()` for
   `data-align` and `reaction_classes()` for the reaction pill's corner. Each
   `match` must handle both variants, so adding `Center` later flags every
   place that needs a decision.

4. **`pub const` as a public class list.** `BUBBLE_CONTENT` is exported so an
   app can put the bubble styling on its own `<button>` or `<a>`. A `const` is
   copied into each place it's used, so sharing it costs nothing at runtime.
